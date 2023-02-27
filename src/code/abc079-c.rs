// https://atcoder.jp/contests/abc079/tasks/abc079_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut vec = Vec::new();
    for s in S.chars() {
        vec.push((s as u8 - b'0') as isize);
    }
    for bit in 0..1<<3 {
        let mut ans = vec[0];
        for i in 0..3 {
            if bit & (1 << i) == 0 {
                ans += vec[i+1];
            }
            else {
                ans -= vec[i+1];
            }
        }
        if ans == 7 {
            print!("{}", vec[0]);
            for i in 0..3 {
                if bit & (1 << i) == 0 {
                    print!("+");
                }
                else {
                    print!("-");
                }
                print!("{}", vec[i+1]);
            }
            println!("=7");
            return;
        }
    }
}