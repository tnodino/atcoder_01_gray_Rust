// https://atcoder.jp/contests/discovery2016-qual/tasks/discovery_2016_qual_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        W: usize,
    }
    let S = "0DiscoPresentsDiscoveryChannelProgrammingContest2016".chars().collect::<Vec<char>>();
    let N = S.len();
    for i in 1..N {
        print!("{}", S[i]);
        if i % W == 0 || i + 1 == N {
            println!();
        }
    }
}