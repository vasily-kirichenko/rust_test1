#![feature(test)]
#![feature(std_misc)]
#![feature(slice_patterns)]

pub mod fib;

//extern crate crypto;
extern crate time;
extern crate test;
extern crate getopts;

//use crypto::md5::Md5;
//use crypto::sha1::Sha1;
//use crypto::sha2::{Sha256, Sha512};
//use crypto::digest::Digest;
use time::now;
use time::Duration;
use std::env;

fn tc<R, F: Fn() -> R>(f: F) -> (R, Duration) {
  let started = time::now();
  let r = f();
  (r, time::now() - started)
}

// fn bench<T: Digest>(name: &str, digest: &mut T, input: &Vec<u8>) {
// 	let started = time::now();
//     for _ in 0..10 {
// 		digest.input(&input);
// 		println!("size = {}, hash = {}", input.len(), digest.result_str());
// 		digest.reset();
//     }

//     let elapsed = time::now() - started;
//     println!("{:?} elapled {}", name, elapsed);
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: i64 =
        match args.as_slice() {
            [_, nStr, ..] => nStr,
            _ => panic!("wrong arguments.")
        }.parse().unwrap();
    let ((r, futureCount), elapsed) = tc(|| { fib::fib(n) });
    println!("fib({}) = {}, elapsed: {}", n, r, elapsed);

    //let input = vec![0u8; 100000000];
    //bench("MD5", &mut Md5::new(), &input);
    //bench("SHA1", &mut Sha1::new(), &input);
    //bench("SHA256", &mut Sha256::new(), &input);
    //test::black_box(bench("SHA512", &mut Sha512::new(), &input));
}