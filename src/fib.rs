use std::sync::*;

fn sfib(n: i32) -> i32 {
    match n {
        0 | 1 | 2 => n,
 		_ => sfib (n-1) + sfib(n-2)
 	}
}

pub fn fib(n: i32) -> Future<i32> {
    Future::spawn(move || {
        if n < 13 { sfib(n) }
        else {
            let mut n2f = fib(n - 2);
            let n1 = fib(n - 1).get();
            let n2 = n2f.get();
            n1 + n2
        }
    })
}
