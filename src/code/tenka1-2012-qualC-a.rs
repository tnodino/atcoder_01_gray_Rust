// https://atcoder.jp/contests/tenka1-2012-qualC/tasks/tenka1_2012_9

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
    }
    let mut ans = 0;
    for i in 2..n {
        let mut flg = true;
        for j in 2..i {
            if i % j == 0 {
                flg = false;
                break;
            }
        }
        if flg {
            ans += 1;
        }
    }
    println!("{}", ans);
}