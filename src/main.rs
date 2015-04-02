#![feature(custom_derive)]
#![feature(std_misc)]

//extern crate time;
//use std::env;
//use time::now;

pub mod traits;
pub mod fib;

use traits::Formattable;
//use std::thread;
use std::sync::*;

impl traits::Formattable for i32 {
	fn format(&self) -> String {
		self.to_string()
	}
}

//#[derive(Debug)]
// struct Person {
//     name: String,
//     age: i8
// }

// impl Formattable for Person {
// 	fn format(&self) -> String {
// 		format!("Person: Name = {}, Age = {}", self.name, self.age)
// 	}
// }

fn main() {
	let f = Future::spawn(move|| { 0 });



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