// https://atcoder.jp/contests/arc151/tasks/arc151_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
        T: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let T = T.chars().collect::<Vec<char>>();
    let mut cnt = 0;
    for i in 0..N {
        if S[i] != T[i] {
            cnt += 1;
        }
    }
    if cnt % 2 == 1 {
        println!("-1");
        return;
    }
    let mut s = 0;
    for i in 0..N {
        if S[i] == T[i] {
            print!("0");
        }
        else {
            if S[i] == '0' {
                if s < cnt {
                    print!("0");
                    s += 1;
                }
                else {
                    print!("1");
                    s -= 1;
                }
            }
            else {
                if -cnt < s {
                    print!("0");
                    s -= 1;
                }
                else {
                    print!("1");
                    s += 1;
                }
            }
            cnt -= 1;
        }
    }
    println!()
}