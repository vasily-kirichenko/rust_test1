use std::sync::*;

fn sfib(n: i64) -> i64 {
    match n {
        0 | 1 | 2 => n,
 		_ => sfib (n-1) + sfib(n-2)
 	}
}

pub fn fib(n: i64) -> (i64, i32) {
    let mut fc = &0i32;

    fn cfib(n: i64) -> Future<i64> {
        //*fc = *fc + 1;
        Future::spawn(move || {
            if n < 13 { sfib(n) }
            else {
                let mut n2f = cfib(n - 2);
                let n1 = cfib(n - 1).get();
                let n2 = n2f.get();
                n1 + n2
            }
        })
    }
    (cfib(n).get(), *fc)
}