// https://atcoder.jp/contests/arc152/tasks/arc152_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        L: isize,
        a: [isize; N],
    }
    let mut cnt = 0;
    for i in 0..N {
        if a[i] == 2 && L - cnt < 2 {
            println!("No");
            return;
        }
        cnt += a[i] + 1;
    }
    println!("Yes");
}