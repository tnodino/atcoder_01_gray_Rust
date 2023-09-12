// https://atcoder.jp/contests/arc019/tasks/arc019_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let N = S.len();
    let mut S = S.chars().collect::<Vec<char>>();
    for i in 0..N {
        S[i] = match S[i] {
            'O' => '0',
            'D' => '0',
            'I' => '1',
            'Z' => '2',
            'S' => '5',
            'B' => '8',
            _ => S[i],
        }
    }
    println!("{}", S.iter().collect::<String>());
}