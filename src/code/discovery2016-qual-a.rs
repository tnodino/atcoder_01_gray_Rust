// https://atcoder.jp/contests/discovery2016-qual/tasks/discovery_2016_qual_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        W: usize,
    }
    let S = "DiscoPresentsDiscoveryChannelProgrammingContest2016".to_string();
    let mut cnt = 0;
    for s in S.chars() {
        print!("{}", s);
        cnt += 1;
        if cnt == W {
            println!();
            cnt = 0;
        }
    }
    if cnt != 0 {
        println!();
    }
}