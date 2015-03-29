extern crate time;
//use std::env;
use time::now;

fn fib(n: i32) -> i32 {
	match n {
		0 | 1 | 2 => n,
		_ => fib (n-1) + fib(n-2)
	}
}

fn main() {
	let start = now();
	for n in 0..40 {
		println!("fib ({}) = {}", n, fib(n));
	}
	println!("Done in {}", now() - start);
}

// fn main() {
// 	match env::args().nth(1) {
// 		Some(n) =>
// 			match n.parse::<i32>() {
// 				Ok(n) =>
// 					let start = time::now();
// 					for n in 1..40 {
// 						println!("fib ({}) = {}", n, fib(n));
// 					}
// 					let elapsed = time::now() - start;
// 					println!("Done in {}", elapsed);
// 				_ => print
// 		}
// 	}
// }