// https://atcoder.jp/contests/arc117/tasks/arc117_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: isize,
        B: isize,
    }
    let N = (A + B) as usize;
    let mut ans: Vec<isize> = Vec::new();
    for i in 1..=A {
        ans.push(i);
    }
    for i in 1..=B {
        ans.push(-i);
    }
    if A < B {
        ans[0] += (B + 1) * B / 2 - (A + 1) * A / 2;
    }
    else if A > B {
        ans[N-1] -= (A + 1) * A / 2 - (B + 1) * B / 2;
    }
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}