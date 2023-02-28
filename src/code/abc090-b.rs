// https://atcoder.jp/contests/abc090/tasks/abc090_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    let mut ans = 0;
    for i in A..=B {
        let mut x = i;
        let mut array = [0; 5];
        for j in 0..5 {
            array[j] = x % 10;
            x /= 10;
        }
        if array[0] == array[4] && array[1] == array[3] {
            ans += 1;
        }
    }
    println!("{}", ans);
}