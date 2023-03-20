// https://atcoder.jp/contests/abc294/tasks/abc294_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
    }
    let mut flg = vec![true; N+1];
    let mut idx = 0;
    for _ in 0..Q {
        input! {
            event: usize,
        }
        if event == 1 {
            continue;
        }
        else if event == 2 {
            input! {
                x: usize,
            }
            flg[x-1] = false;
        }
        else {
            while !flg[idx] {
                idx += 1;
            }
            println!("{}", idx + 1);
        }
    }
}