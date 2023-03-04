// https://atcoder.jp/contests/abc275/tasks/abc275_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        H: [usize; N],
    }
    let ma = H.iter().max().unwrap();
    for i in 0..N {
        if H[i] == *ma {
            println!("{}", i + 1);
            return;
        }
    }
}