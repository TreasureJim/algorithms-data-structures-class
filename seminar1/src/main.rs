#![allow(unused)]
mod task1;
mod task2;

use std::{sync::{mpsc, Arc, Mutex}, thread};

type Job = (&'static str, Box<dyn FnOnce() + Send + 'static>);

struct JobQueue {
    sender: mpsc::Sender<Job>,
    threads: Vec<thread::JoinHandle<()>>
}

impl JobQueue {
    pub fn new(num_threads: usize) -> Self {
        assert!(num_threads > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(num_threads);

        for _ in 0..num_threads {
            let receiver = receiver.clone();
            
            thread::spawn(move || {
                let job = receiver.lock().unwrap().recv();

                match
            })
        }
    }
}

fn main() {


}


mod test_helpers {
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
}
