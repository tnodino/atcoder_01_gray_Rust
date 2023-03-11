// https://atcoder.jp/contests/abc270/tasks/abc270_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut X: isize,
        mut Y: isize,
        mut Z: isize,
    }
    if X < 0 {
        X *= -1;
        Y *= -1;
        Z *= -1;
    }
    if 0 < Y && Y < X {
        if Y < Z {
            println!("-1");
        }
        else {
            println!("{}", Z.abs() + (X - Z).abs());
        }
    }
    else {
        println!("{}", X);
    }
}