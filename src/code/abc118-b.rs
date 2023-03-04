// https://atcoder.jp/contests/abc118/tasks/abc118_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut cnt = vec![0; M];
    for _ in 0..N {
        input! {
            K: usize,
            A: [usize; K],
        }
        for a in A.iter() {
            cnt[a-1] += 1;
        }
    }
    let mut ans = 0;
    for c in cnt.iter() {
        if c == &N {
            ans += 1;
        }
    }
    println!("{}", ans);
}