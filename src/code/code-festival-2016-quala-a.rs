// https://atcoder.jp/contests/code-festival-2016-quala/tasks/codefestival_2016_qualA_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: String,
    }
    let mut s = s.chars().collect::<Vec<char>>();
    s.insert(4, ' ');
    println!("{}", s.iter().collect::<String>());
}