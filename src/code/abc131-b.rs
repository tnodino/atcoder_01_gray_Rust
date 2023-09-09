// https://atcoder.jp/contests/abc131/tasks/abc131_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, L): (isize, isize),
    }
    let mut vec = Vec::new();
    let mut sum = 0;
    for i in 0..N {
        vec.push(((L + i).abs(), L + i));
        sum += L + i;
    }
    vec.sort();
    println!("{:?}", sum - vec[0].1);
}