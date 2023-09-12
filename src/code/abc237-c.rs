// https://atcoder.jp/contests/abc237/tasks/abc237_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let N = S.len();
    let mut S = S.chars().rev().collect::<Vec<char>>();
    let mut cntl = 0;
    for i in 0..N {
        if S[i] != 'a' {
            break;
        }
        cntl += 1;
    }
    let mut cntr = 0;
    for i in (0..N).rev() {
        if S[i] != 'a' {
            break;
        }
        cntr += 1;
    }
    if cntl < cntr {
        println!("No");
        return;
    }
    for _ in 0..cntl-cntr {
        S.push('a');
    }
    let T = S.clone().into_iter().rev().collect::<Vec<char>>();
    println!("{}", match S == T {
        true => "Yes",
        false => "No",
    });
}