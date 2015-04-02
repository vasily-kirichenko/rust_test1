//#![feature(custom_derive)]
#![feature(std_misc)]

//pub mod consequtive_ints;
use std::fmt::Debug;

mod fib;

// fn print_array_by_ref<T: Debug>(xs: &[T]) {
// 	for x in xs.iter() {
// 		println!("{:?}", x)
// 	}
// }
//
// fn print_boxed_array<T: Debug>(xs: Box<[T]>) {
// 	for x in xs.iter() {
// 		println!("{:?}", x)
// 	}
// }

fn main() {
	let n = 30;
	println!("fib({}) = {}", n, fib::fib(n).get());

	// let a = [1, 2, 3];
	// print_array_by_ref(&a);
	//
	// let b = Box::new(a);
	// print_boxed_array(b);
	// print_array_by_ref(&a);
}
