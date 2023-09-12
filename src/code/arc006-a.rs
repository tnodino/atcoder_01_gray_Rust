// https://atcoder.jp/contests/arc006/tasks/arc006_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let N = 6;
    input! {
        E: [usize; N],
        B: usize,
        L: [usize; N],
    }
    let mut flg = vec![false; 10];
    for i in 0..N {
        flg[E[i]] = true;
    }
    let mut cnt = 0;
    for i in 0..N {
        if flg[L[i]] {
            cnt += 1;
        }
    }
    println!("{}", match (cnt, L.contains(&B)) {
        (6, _) => 1,
        (5, true) => 2,
        (5, _) => 3,
        (4, _) => 4,
        (3, _) => 5,
        (_, _) => 0,
    });
}