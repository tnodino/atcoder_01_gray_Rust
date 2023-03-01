// https://atcoder.jp/contests/abc093/tasks/abc093_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        K: usize,
    }
    let mut vec = Vec::new();
    let mut cnt = 0;
    for i in A..=B {
        vec.push(i);
        cnt += 1;
        if cnt == K {
            break;
        }
    }
    let mut cnt = 0;
    for i in (A..=B).rev() {
        vec.push(i);
        cnt += 1;
        if cnt == K {
            break;
        }
    }
    vec.sort();
    vec.dedup();
    for v in vec {
        println!("{}", v);
    }
}