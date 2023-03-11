// https://atcoder.jp/contests/abc208/tasks/abc208_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut P: usize,
    }
    let mut idx = 1;
    let mut fac = 1;
    let mut vec = vec![1];
    while fac < P {
        idx += 1;
        fac *= idx;
        vec.push(fac);
    }
    let mut ans = 0;
    for i in (0..vec.len()).rev() {
        ans += P / vec[i];
        P %= vec[i];
    }
    println!("{}", ans);
}