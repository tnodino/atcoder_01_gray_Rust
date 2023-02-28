// https://atcoder.jp/contests/abc207/tasks/abc207_c

use proconio::input;
use proconio::fastout;
use std::cmp::max;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut l = Vec::new();
    let mut r = Vec::new();
    for _ in 0..N {
        input! {
            a: usize,
            mut b: isize,
            mut c: isize,
        }
        b *= 10;
        c *= 10;
        match a {
            1 => (),
            2 => c -= 5,
            3 => b += 5,
            _ => {
                b += 5;
                c -= 5;
            },
        }
        l.push(b);
        r.push(c);
    }
    let mut ans = 0;
    for i in 0..N {
        for j in i+1..N {
            if !(min(r[i], r[j]) < max(l[i], l[j])) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}