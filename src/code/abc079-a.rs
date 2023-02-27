// https://atcoder.jp/contests/abc079/tasks/abc079_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: String,
    }
    let N = N.chars().collect::<Vec<char>>();
    if (N[0] == N[1] && N[1] == N[2]) || (N[1] == N[2] && N[2] == N[3]) {
        println!("Yes");
    }
    else {
        println!("No");
    }
}