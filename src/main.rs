extern crate num;
extern crate time;

use num::{Num};
use time::Duration;

fn tc<T, F: Fn() -> T>(f: F) -> (T, Duration) {
	let start = time::now();
	let res = f();
	(res, time::now() - start)
}

fn fib<T: Num + Copy>(n: T) -> T {
	let zero = T::zero();
	let one = T::one();
	if n == zero || n == one { n }
	else { fib(n - one) + fib(n - one - one) }
}

fn main() {
	//let mut f = Future::spawn(move || { 0 });
	//let value = f.get();
	//println!("{}", value);
	// let _ = fib(1i64);
	// let _ = fib(1i32);
	// let _ = fib(1u32);
	let (_, duration) = tc(|| for i in 0..40 { println!("fib({}) = {}", i, fib(i)) });
	println!("Done in {}", duration);

	// let _ = fib(1f64);

	println!("DONE.");

	//println!("1 = {}", 1.format());
	//let p = Person { name: "zai".to_string(), age: 41i8 };
	//println!("{:?} = {}", p, p.format());

	// let start = now();
	// for n in 0..40 {
	// 	println!("fib ({}) = {}", n, fib(n));
	// }
	// println!("Done in {}", now() - start);
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
