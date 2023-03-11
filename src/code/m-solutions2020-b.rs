// https://atcoder.jp/contests/m-solutions2020/tasks/m_solutions2020_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        mut B: usize,
        mut C: usize,
        K: usize,
    }
    let mut cnt = 0;
    while A >= B {
        B *= 2;
        cnt += 1;
    }
    while B >= C {
        C *= 2;
        cnt += 1;
    }
    if K >= cnt {
        println!("Yes");
    }
    else {
        println!("No");
    }
}