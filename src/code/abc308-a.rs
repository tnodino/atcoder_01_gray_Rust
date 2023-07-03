// https://atcoder.jp/contests/abc308/tasks/abc308_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let N = 8;
    input! {
        S: [usize; N],
    }
    for i in 1..N {
        if S[i-1] > S[i] {
            println!("No");
            return;
        }
    }
    for i in 0..N {
        if S[i] < 100 || 675 < S[i] || S[i] % 25 != 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}