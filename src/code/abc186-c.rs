// https://atcoder.jp/contests/abc186/tasks/abc186_c

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans = 0;
    'outer: for i in 1..=N {
        let mut n = i;
        while n > 0 {
            if n % 10 == 7 {
                continue 'outer;
            }
            n /= 10;
        }
        let mut n = format!("{:o}", i).parse::<usize>().unwrap();
        while n > 0 {
            if n % 10 == 7 {
                continue 'outer;
            }
            n /= 10;
        }
        ans += 1;
    }
    println!("{}", ans);
}