// https://atcoder.jp/contests/abc194/tasks/abc194_c

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [isize; N],
    }
    let mut map = HashMap::new();
    for i in -200..=200 {
        map.insert(i, 0);
    }
    for i in 0..N {
        let v = map.get_mut(&A[i]).unwrap();
        *v += 1;
    }
    let mut ans = 0;
    for i in -200..=200 {
        for j in -200..i {
            ans += pow(i - j, 2) * map[&i] * map[&j]
        }
    }
    println!("{}", ans);
}