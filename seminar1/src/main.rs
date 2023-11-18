mod task1;
mod task2;

use std::{
    collections::HashMap,
    fmt::{self, Display},
    io,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::{self},
};

use rand::{thread_rng, Rng};
use task1::*;

use crate::task2::binary_search_recursive;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct JobQueue {
    sender: mpsc::Sender<Job>,
    workers: Vec<thread::JoinHandle<()>>,
}

impl JobQueue {
    pub fn new(num_threads: usize) -> Self {
        assert!(num_threads > 0);

        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(num_threads);

        for _ in 0..num_threads {
            let receiver = receiver.clone();

            let worker = thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv();

                match job {
                    Ok(job) => (job)(),
                    Err(err) => {
                        println!("Receiver dropped, shutting down thread. Err: {err}");
                        return;
                    }
                }
            });

            workers.push(worker);
        }

        Self { sender, workers }
    }

    pub fn publish<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let _ = self.sender.send(Box::new(job));
    }

    pub fn benchmark_func<F>(&self, result_acc: Arc<Mutex<Vec<u128>>>, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.publish(move || {
            let start = time::Instant::now();
            (func)();
            let length = start.elapsed();
            result_acc.lock().unwrap().push(length.as_micros());
        });
    }

    pub fn wait_all(self) {
        drop(self.sender);

        for thread in self.workers {
            thread.join().unwrap();
        }
    }
}

fn main() {
    let queue = JobQueue::new(1);
    let mut results = ResultMeasurements::new();

    let test_arr = test_helpers::read_test_file();
    // let test_arr = test_helpers::generate_random_list(1_000_000, 0, 1000).0;

    let mut size = 100;
    while size <= 1_000_000 {
        for _ in 1..=10 {
            // quicksort_iter_median
            let mut sub_test_arr = Vec::from(&test_arr[..size]);
            queue.benchmark_func(
                Arc::clone(&results.quicksort_iter_median.entry(size).or_default()),
                move || {
                    quicksort_iterative(&mut sub_test_arr, &task1::median_pivot);
                    eprintln!("Finished quicksort_iterative with size: {size}");
                },
            );

            // quicksort_iter_random
            let mut sub_test_arr = Vec::from(&test_arr[..size]);
            queue.benchmark_func(
                Arc::clone(&results.quicksort_iter_random.entry(size).or_default()),
                move || {
                    quicksort_iterative(&mut sub_test_arr, &task1::random_pivot);
                    eprintln!("Finished quicksort_iter_random with size: {size}");
                },
            );

            // quicksort_iter_first
            let mut sub_test_arr = Vec::from(&test_arr[..size]);
            queue.benchmark_func(
                Arc::clone(&results.quicksort_iter_first.entry(size).or_default()),
                move || {
                    quicksort_iterative(&mut sub_test_arr, &task1::median_pivot);
                    eprintln!("Finished quicksort_iter_first with size: {size}");
                },
            );

            // quicksort_recur_median
            let mut sub_test_arr = Vec::from(&test_arr[..size]);
            queue.benchmark_func(
                Arc::clone(&results.quicksort_recur_median.entry(size).or_default()),
                move || {
                    quick_sort_recursive(&mut sub_test_arr, &task1::median_pivot);
                    eprintln!("Finished quicksort_recur_median with size: {size}");
                },
            );

            // quicksort_recur_random
            let mut sub_test_arr = Vec::from(&test_arr[..size]);
            queue.benchmark_func(
                Arc::clone(&results.quicksort_recur_random.entry(size).or_default()),
                move || {
                    quick_sort_recursive(&mut sub_test_arr, &task1::random_pivot);
                    eprintln!("Finished quicksort_recur_random with size: {size}");
                },
            );

            // quicksort_recur_first
            let mut sub_test_arr = Vec::from(&test_arr[..size]);
            queue.benchmark_func(
                Arc::clone(&results.quicksort_recur_first.entry(size).or_default()),
                move || {
                    quick_sort_recursive(&mut sub_test_arr, &task1::first_pivot);
                    eprintln!("Finished quicksort_recur_first with size: {size}");
                },
            );

            // insertion_iter
            let mut sub_test_arr = Vec::from(&test_arr[..size]);
            queue.benchmark_func(
                Arc::clone(&results.insertion_iter.entry(size).or_default()),
                move || {
                    insertion_sort_iter(&mut sub_test_arr);
                    eprintln!("Finished insertion_iter with size: {size}");
                },
            );

            // insertion_recur
            let mut sub_test_arr = Vec::from(&test_arr[..size]);
            queue.benchmark_func(
                Arc::clone(&results.insertion_recur.entry(size).or_default()),
                move || {
                    insertion_sort_iter(&mut sub_test_arr);
                    eprintln!("Finished insertion_recur with size: {size}");
                },
            );

            // binary_search_iter
            let sub_test_arr = Vec::from(&test_arr[..size]);
            let search_num = sub_test_arr[thread_rng().gen_range(0..size)];
            queue.benchmark_func(
                Arc::clone(&results.binary_search.entry(size).or_default()),
                move || {
                    binary_search_recursive(&sub_test_arr, search_num);
                    eprintln!("Finished binary_search_iter with size: {size}");
                },
            );
        }

        size *= 10;
    }

    queue.wait_all();

    // all threads should be dead
    println!("{}", &results);
    let _ = results.display_averages();
}

type ResultHolder = HashMap<usize, Arc<Mutex<Vec<u128>>>>;

#[derive(Debug)]
struct ResultMeasurements {
    // quicksort
    //  iter
    //  recursive
    //  ------
    //  median of 3
    //  random
    //  first
    quicksort_iter_median: ResultHolder,
    quicksort_iter_random: ResultHolder,
    quicksort_iter_first: ResultHolder,
    quicksort_recur_median: ResultHolder,
    quicksort_recur_random: ResultHolder,
    quicksort_recur_first: ResultHolder,

    // insertion
    //  iter
    //  recursive
    insertion_iter: ResultHolder,
    insertion_recur: ResultHolder,

    // binary search
    //  recursive
    binary_search: ResultHolder,
}

impl Display for ResultMeasurements {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Result {{\n")?;
        self.format_result_holder(f, "quicksort_iter_median", &self.quicksort_iter_median)?;
        self.format_result_holder(f, "quicksort_iter_random", &self.quicksort_iter_random)?;
        self.format_result_holder(f, "quicksort_iter_first", &self.quicksort_iter_first)?;
        self.format_result_holder(f, "quicksort_recur_median", &self.quicksort_recur_median)?;
        self.format_result_holder(f, "quicksort_recur_random", &self.quicksort_recur_random)?;
        self.format_result_holder(f, "quicksort_recur_first", &self.quicksort_recur_first)?;
        self.format_result_holder(f, "insertion_iter", &self.insertion_iter)?;
        self.format_result_holder(f, "insertion_recur", &self.insertion_recur)?;
        self.format_result_holder(f, "binary_search", &self.binary_search)?;
        write!(f, "}}")
    }
}

impl ResultMeasurements {
    pub fn new() -> Self {
        Self {
            quicksort_iter_median: HashMap::new(),
            quicksort_iter_random: HashMap::new(),
            quicksort_iter_first: HashMap::new(),
            quicksort_recur_median: HashMap::new(),
            quicksort_recur_random: HashMap::new(),
            quicksort_recur_first: HashMap::new(),
            insertion_iter: HashMap::new(),
            insertion_recur: HashMap::new(),
            binary_search: HashMap::new(),
        }
    }

    fn format_result_holder(
        &self,
        f: &mut fmt::Formatter<'_>,
        name: &str,
        holder: &ResultHolder,
    ) -> fmt::Result {
        write!(f, "  {}: {{\n", name)?;
        for (key, value) in holder {
            let arr = value.as_ref().lock().unwrap();
            write!(f, "    {}: {:?}\n", key, arr)?;
        }
        write!(f, "  }},\n")
    }

    fn display_average_for_holder(
        &self,
        label: &str,
        holder: &ResultHolder,
    ) -> io::Result<()> {
        println!("{}", label);
        for (size, times) in holder {
            let times = times.lock().unwrap();
            let average = times.iter().sum::<u128>() as f64
                / times.len() as f64;
            println!("  Size {}: Average: {:.2}", size, average);
        }

        Ok(())
    }

    pub fn display_averages(&self) -> io::Result<()> {
        self.display_average_for_holder(
            "QuickSort - Iterative - Median of 3:",
            &self.quicksort_iter_median,
        )?;
        self.display_average_for_holder(
            "QuickSort - Iterative - Random:",
            &self.quicksort_iter_random,
        )?;
        self.display_average_for_holder(
            "QuickSort - Iterative - First:",
            &self.quicksort_iter_first,
        )?;
        self.display_average_for_holder(
            "QuickSort - Recursive - Median of 3:",
            &self.quicksort_recur_median,
        )?;
        self.display_average_for_holder(
            "QuickSort - Recursive - Random:",
            &self.quicksort_recur_random,
        )?;
        self.display_average_for_holder(
            "QuickSort - Recursive - First:",
            &self.quicksort_recur_first,
        )?;
        self.display_average_for_holder("Insertion - Iterative:", &self.insertion_iter)?;
        self.display_average_for_holder("Insertion - Recursive:", &self.insertion_recur)?;
        self.display_average_for_holder("Binary Search:", &self.binary_search)?;

        Ok(())
    }
}

mod test_helpers {
    use std::time::Instant;

    use rand::Rng;

    pub fn generate_random_list(n: u32, min: i32, max: i32) -> (Vec<i32>, Vec<i32>) {
        let mut v = vec![0; n as usize];
        let mut rng = rand::prelude::thread_rng();
        for i in 0..n {
            v[i as usize] = rng.gen_range(min..=max);
        }

        let mut vs = v.clone();
        vs.sort();
        (v, vs)
    }

    const TEST_FILE_PATH: &str = "./random_numbers.txt";

    pub fn read_test_file() -> Vec<i32> {
        let v = std::fs::read_to_string(TEST_FILE_PATH).unwrap();
        v.lines()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect()
    }

    pub fn benchmark_func<F>(func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let start = Instant::now();
        (func)();
        let length = start.elapsed();
    }
}
