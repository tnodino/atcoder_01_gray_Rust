// https://atcoder.jp/contests/abc196/tasks/abc196_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    for i in 1..=N {
        let x = i.to_string();
        let x = format!("{}{}", x, x);
        let x = x.parse::<usize>().unwrap();
        if x > N {
            println!("{}", i - 1);
            return;
        }
    }
}
