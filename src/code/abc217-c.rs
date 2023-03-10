// https://atcoder.jp/contests/abc217/tasks/abc217_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        p: [usize; N],
    }
    let mut q = vec![0; N];
    for i in 0..N {
        q[p[i]-1] = i + 1;
    }
    println!("{}", q.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}