// https://atcoder.jp/contests/tenka1-2012-qualC/tasks/tenka1_2012_9

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
    }
    let mut vec = Vec::new();
    'outer: for i in 2..n {
        for j in 0..vec.len() {
            if i % vec[j] == 0 {
                continue 'outer;
            }
        }
        vec.push(i);
    }
    println!("{}", vec.len());
}