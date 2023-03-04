// https://atcoder.jp/contests/yahoo-procon2019-qual/tasks/yahoo_procon2019_qual_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a1: usize,
        b1: usize,
        a2: usize,
        b2: usize,
        a3: usize,
        b3: usize,
    }
    let array = [a1, b1, a2, b2, a3, b3];
    for i in 1..=4 {
        if array.iter().filter(|&x| x == &i).count() == 3 {
            println!("NO");
            return;
        }
    }
    println!("YES");
}