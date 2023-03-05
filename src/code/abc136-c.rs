// https://atcoder.jp/contests/abc136/tasks/abc136_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut H: [usize; N],
    }
    H[0] -= 1;
    for i in 1..N {
        if H[i-1] < H[i] {
            H[i] -= 1;
        }
        if H[i-1] > H[i] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}