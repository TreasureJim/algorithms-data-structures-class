mod task1;

fn main() {
}

mod test_helpers {
    pub fn generate_random_list(n: u32) -> (Vec<i32>, Vec<i32>) {
        let mut v = vec![0; n as usize];
        for _ in 0..n {
            v.push(rand::random::<i32>());
        }

        let mut vs = v.clone();
        vs.sort();
        (v, vs)
    }

    const TEST_FILE_PATH: &str = "./random_numbers.txt";

    pub fn read_test_file() -> Vec<i32> {
        let v: Vec<&str> = std::fs::read_to_string(TEST_FILE_PATH).unwrap().split("\r\n").collect();
        v.iter().map(|x| )
    }
}
