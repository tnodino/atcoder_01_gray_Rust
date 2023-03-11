// https://atcoder.jp/contests/abc175/tasks/abc175_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        L: [usize; N],
    }
    let mut ans = 0;
    for i in 0..N {
        for j in i+1..N {
            for k in j+1..N {
                if L[i] + L[j] > L[k] && L[i] + L[k] > L[j] && L[j] + L[k] > L[i]
                    && L[i] != L[j] && L[i] != L[k] && L[j] != L[k] {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}