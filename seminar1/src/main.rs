#![allow(unused)]
mod task1;
mod task2;

use std::{
    collections::HashMap,
    sync::{mpsc, Arc, Mutex},
    thread, time::{self, Duration},
};

use task1::*;
use test_helpers::read_test_file;

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
        self.sender.send(Box::new(job));
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
        for thread in self.workers {
            thread.join().unwrap();
        }
    }
}

fn main() {
    let queue = JobQueue::new(4);
    let results = Result::new();

    let test_arr = test_helpers::read_test_file();

    let mut size = 100;
    while size <= 1_000_000 {
        // quicksort_iter_median
        // quicksort_iter_random
        // quicksort_iter_first
        // quicksort_recur_median
        // quicksort_recur_random
        // quicksort_recur_first
        // insertion_iter
        // insertion_recur
        // binary_search_iter
        // binary_search_recur

        let mut sub_test_arr = Vec::from(&test_arr[..size]);

        println!("Queuing quicksort_iterative with size: {size}");
        queue.benchmark_func(Arc::clone(&results.quicksort_iter_median), move || {
            quicksort_iterative(&mut sub_test_arr, &task1::median_pivot)
        });

        size *= 10;
    }

    queue.wait_all();
    thread::sleep(Duration::from_secs(1));

    // all threads should be dead
    dbg!(&results);
}

#[derive(Debug)]
struct Result {
    // quicksort
    //  iter
    //  recursive
    //  ------
    //  median of 3
    //  random
    //  first
    quicksort_iter_median: Arc<Mutex<Vec<u128>>>,
    quicksort_iter_random: Arc<Mutex<Vec<u128>>>,
    quicksort_iter_first: Arc<Mutex<Vec<u128>>>,
    quicksort_recur_median: Arc<Mutex<Vec<u128>>>,
    quicksort_recur_random: Arc<Mutex<Vec<u128>>>,
    quicksort_recur_first: Arc<Mutex<Vec<u128>>>,

    // insertion
    //  iter
    //  recursive
    insertion_iter: Arc<Mutex<Vec<u128>>>,
    insertion_recur: Arc<Mutex<Vec<u128>>>,

    // binary search
    //  iter
    //  recursive
    binary_search_iter: Arc<Mutex<Vec<u128>>>,
    binary_search_recur: Arc<Mutex<Vec<u128>>>,
}

impl Result {
    pub fn new() -> Self {
        Self {
            quicksort_iter_median: Arc::new(Mutex::new(Vec::new())),
            quicksort_iter_random: Arc::new(Mutex::new(Vec::new())),
            quicksort_iter_first: Arc::new(Mutex::new(Vec::new())),
            quicksort_recur_median: Arc::new(Mutex::new(Vec::new())),
            quicksort_recur_random: Arc::new(Mutex::new(Vec::new())),
            quicksort_recur_first: Arc::new(Mutex::new(Vec::new())),
            insertion_iter: Arc::new(Mutex::new(Vec::new())),
            insertion_recur: Arc::new(Mutex::new(Vec::new())),
            binary_search_iter: Arc::new(Mutex::new(Vec::new())),
            binary_search_recur: Arc::new(Mutex::new(Vec::new())),
        }
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
