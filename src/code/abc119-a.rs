// https://atcoder.jp/contests/abc119/tasks/abc119_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut sp = S.trim().split('/');
    let _y = sp.next().unwrap();
    let m = sp.next().unwrap();
    let m = m.parse::<usize>().unwrap();
    if m <= 4 {
        println!("Heisei");
    }
    else {
        println!("TBD");
    }
}