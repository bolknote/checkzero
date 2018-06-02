use std::io;
use std::io::prelude::Read;

fn read_chunk<R: Read> (reader: R, size: u64) -> Vec<u8> {
	let mut buf = vec![];
	let _size = reader.take(size).read_to_end(&mut buf);
	buf
}

fn check_not_zeroed(buffer: Vec<u8>) -> bool {
	for byte in buffer.iter() {
		if *byte != 0 {
			return true
		}
	}

	false
}

fn check_file_zeroed(filename: String) -> Result<bool, io::Error> {
	use std::fs::File;

	let mut f = File::open(filename)?;
	let mut filesize = 0;

	loop {
		let chunk = read_chunk(&mut f, 8192);
		let chunk_len = chunk.len();
		filesize += chunk_len;

		if chunk_len == 0 {
			return Ok(filesize > 0)
		}

		if check_not_zeroed(chunk) {
			return Ok(false)
		}
	}	
}

// 2 - error
// 1 - not zeroed
// 0 - zeroed

fn run() -> i32 {
	use std::env;

	match env::args().nth(1) {
		Some(filename) => {
			match check_file_zeroed(filename) {
				Ok(zeroed) => if zeroed {0} else {1},
				Err(_) => 2
			}
		}
		None => 2
	}
}

fn main() {
	std::process::exit(run());
}
