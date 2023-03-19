// https://atcoder.jp/contests/abc251/tasks/abc251_b

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        W: usize,
        A: [usize; N],
    }
    let mut set = HashSet::new();
    for i in 0..N {
        let s = A[i];
        if s <= W {
            set.insert(s);
        }
    }
    for i in 0..N {
        for j in i+1..N {
            let s = A[i] + A[j];
            if s <= W {
                set.insert(s);
            }
        }
    }
    for i in 0..N {
        for j in i+1..N {
            for k in j+1..N {
                let s = A[i] + A[j] + A[k];
                if s <= W {
                    set.insert(s);
                }
            }
        }
    }
    println!("{}", set.len());
}