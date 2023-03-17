// https://atcoder.jp/contests/keyence2021/tasks/keyence2021_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut K: isize,
        a: [usize; N],
    }
    let mut cnt = vec![0; N+1];
    for i in 0..N {
        cnt[a[i]] += 1;
    }
    let mut ans = 0;
    for i in 0..=N {
        if K >= cnt[i] {
            ans += i as isize * (K - cnt[i]);
            K -= K - cnt[i];
        }
        if K <= 0 {
            break;
        }
    }
    println!("{}", ans);
}