// https://atcoder.jp/contests/mujin-pc-2016/tasks/mujin_pc_2016_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        c: String,
    };
    if "OPKL".contains(&c) {
        println!("Right");
    }
    else {
        println!("Left");
    }
}