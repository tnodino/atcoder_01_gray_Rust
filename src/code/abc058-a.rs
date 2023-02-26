// https://atcoder.jp/contests/abc058/tasks/abc058_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let mut array = [a, b, c];
    array.sort();
    if array[1] - array[0] == array[2] - array[1] {
        println!("YES");
    }
    else {
        println!("NO");
    }
}