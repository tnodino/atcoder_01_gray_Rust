// https://atcoder.jp/contests/tenka1-2014-quala/tasks/tenka1_2014_qualA_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let N = 1000;
    let mut vec = vec![];
    for i in 1..=N {
        vec.push(i.to_string());
    }
    vec.sort();
    for v in vec {
        println!("{}", v);
    }
}