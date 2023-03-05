// https://atcoder.jp/contests/abc135/tasks/abc135_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        p: [usize; N],
    }
    let mut cnt = 0;
    for i in 0..N {
        if i + 1 != p[i] {
            cnt += 1;
        }
    }
    if cnt <= 2 {
        println!("YES");
    }
    else {
        println!("NO");
    }
}