extern crate seminar3;

use seminar3::binaryheap;

use std::{
    collections::HashMap,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::{self},
};

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

use csv;

#[derive(serde::Serialize)]
struct Row {
    #[serde(rename = "Size")]
    size: usize,
    #[serde(rename = "Iterative Insert")]
    iter_insert: u128,
    #[serde(rename = "Heapify Insert")]
    heapify_insert: u128,
}

fn main() {
    let queue = JobQueue::new(4);

    let test_path = std::env::args().into_iter().skip(1).next().unwrap();

    let test_arr: Vec<u32> = std::fs::read_to_string(test_path).unwrap().lines().map(|x| x.parse::<u32>().unwrap()).collect();

    // multiple sizes of n and m = 1
    let mut results: [HashMap<usize, Arc<Mutex<Vec<u128>>>>; 4] = Default::default();

    let sizes = (1_000..=10_000).into_iter().step_by(1_000).collect::<Vec<_>>();
    let mut writer = csv::Writer::from_path("results.txt").expect("Couldn't open file");

    for &size in &sizes {
        for iter in 0..10 {
            let arr = test_arr[..size].to_vec();
            queue.benchmark_func(Arc::clone(results[0].entry(size).or_default()), move || {
                let mut heap = binaryheap::MinHeap::new();

                for x in arr {
                    heap.insert(x);
                }
                eprintln!("Finished iter_insert with size: {size} {iter}");
            });

            let arr = test_arr[..size].to_vec();
            queue.benchmark_func(Arc::clone(results[1].entry(size).or_default()), move || {
                binaryheap::MinHeap::from(arr);
                eprintln!("Finished heapify_insert with size: {size} {iter}");
            });
        }
    }

    queue.wait_all();

    let results: Vec<_> = results
        .into_iter()
        .map(|old_map| {
            old_map
                .into_iter()
                .map(|(size, vec)| {
                    let vec = Arc::into_inner(vec).unwrap().into_inner().unwrap();
                    let len = vec.len() as u128;
                    (size, vec.into_iter().sum::<u128>() / len)
                })
                .collect::<HashMap<_, _>>()
        })
        .collect();

    dbg!(&results);

    // each algorithm will be a different header eg "size", "Array List", "Array List Iterator", "Linked List", "Linked List Iterator"
    for size in sizes {
        writer.serialize(Row {
            size,
            iter_insert: *results[0].get(&size).unwrap(),
            heapify_insert: *results[1].get(&size).unwrap(),
        });
    }

    writer.flush();
}
