// https://atcoder.jp/contests/abc116/tasks/abc116_b

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

fn f(n: usize) -> usize {
    if n % 2 == 0 {
        return n / 2;
    }
    return 3 * n + 1;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut s: usize,
    }
    let mut set = HashSet::new();
    set.insert(s);
    loop {
        s = f(s);
        if set.contains(&s) {
            break;
        }
        set.insert(s);
    }
    println!("{}", set.len() + 1);
}