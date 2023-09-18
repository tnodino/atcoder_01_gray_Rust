// https://atcoder.jp/contests/mujin-pc-2016/tasks/mujin_pc_2016_a

use itertools::Itertools;
use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        c: char,
    }
    println!("{}", match "KOLP".chars().contains(&c) {
        true => "Right",
        false => "Left",
    });
}