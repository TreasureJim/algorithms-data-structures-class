#![allow(unused)]

mod task1;
mod task2;
mod task3;
mod task4;

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
    #[serde(rename = "Array List")]
    arraylist: u128,
    #[serde(rename = "Array List Iterator")]
    arraylist_iter: u128,
    #[serde(rename = "Linked List")]
    linkedlist: u128,
    #[serde(rename = "Linked List Iterator")]
    linkedlist_iter: u128,
}

fn main() {
    let queue = JobQueue::new(4);

    // multiple sizes of n and m = 1
    let mut results: [HashMap<usize, Arc<Mutex<Vec<u128>>>>; 4] = Default::default();

    let sizes = (10_000..=100_000).into_iter().step_by(10_000).collect::<Vec<_>>();
    let mut writer = csv::Writer::from_path("results.txt").expect("Couldn't open file");

    for iter in 0..10 {
        for &n in &sizes {
            let arr = task4::generate_arraylist(n);
            queue.benchmark_func(Arc::clone(results[0].entry(n).or_default()), move || {
                task4::josephus_arraylist(arr, 1);
                eprintln!("Finished quicksort_iterative with size: {n}");
            });

            let arr = task4::generate_arraylist(n);
            queue.benchmark_func(Arc::clone(results[1].entry(n).or_default()), move || {
                task4::josephus_arraylist_iter(arr, 1);
                eprintln!("Finished quicksort_iterative with size: {n}");
            });

            let arr = task4::generate_linkedlist(n);
            queue.benchmark_func(Arc::clone(results[2].entry(n).or_default()), move || {
                task4::josephus_linkedlist(arr, 1);
                eprintln!("Finished quicksort_iterative with size: {n}");
            });

            let arr = task4::generate_linkedlist(n);
            queue.benchmark_func(Arc::clone(results[3].entry(n).or_default()), move || {
                task4::josephus_linkedlist_iter(arr, 1);
                eprintln!("Finished quicksort_iterative with size: {n}");
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
            arraylist: *results[0].get(&size).unwrap(),
            arraylist_iter: *results[1].get(&size).unwrap(),
            linkedlist: *results[2].get(&size).unwrap(),
            linkedlist_iter: *results[3].get(&size).unwrap(),
        });
    }

    writer.flush();
}
