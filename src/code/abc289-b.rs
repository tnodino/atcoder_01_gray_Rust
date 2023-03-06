// https://atcoder.jp/contests/abc289/tasks/abc289_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        a: [usize; M],
    }
    let mut flg = vec![false; N];
    for i in a.iter() {
        flg[i-1] = true;
    }
    let mut ans = Vec::new();
    let mut stack = Vec::new();
    for i in 0..N {
        if flg[i] {
            stack.push(i+1);
        }
        else {
            ans.push(i+1);
            while !stack.is_empty() {
                ans.push(stack.pop().unwrap());
            }
        }
    }
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}