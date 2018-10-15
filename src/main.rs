use std::env;
use std::fs::File;
use std::io::prelude::Read;
use std::io;

#[derive(Copy, Clone)]
enum CheckResult {
    Zero = 0,
    Empty = 1,
    NotZero = 2,
    Error = 3,
}

fn check_file_zeroed(filename: &str) -> Result<CheckResult, io::Error> {
    let mut f = File::open(filename)?;
    let mut res = CheckResult::Empty;
    let mut chunk = [0; 8192];

    loop {
        let chunk_len = f.read(&mut chunk[..])?;
        if chunk_len == 0 {
            return Ok(res);
        }

        res = CheckResult::Zero;

        if chunk[0..chunk_len].iter().any(|&i| i != 0) {
            return Ok(CheckResult::NotZero);
        }
    }
}

fn run() -> i32 {
    env::args()
        .nth(1)
        .map(|filename| check_file_zeroed(&filename).unwrap_or_else(|_| CheckResult::Error) as i32)
        .unwrap_or_else(|| CheckResult::Error as i32)
}

fn main() {
    std::process::exit(run());
}
