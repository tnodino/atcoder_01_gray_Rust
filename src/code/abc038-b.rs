// https://atcoder.jp/contests/abc038/tasks/abc038_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (H1, W1, H2, W2): (usize, usize, usize, usize),
    }
    if H1 == H2 || H1 == W2 || W1 == H2 || W1 == W2 {
        println!("YES");
    }
    else {
        println!("NO");
    }
}