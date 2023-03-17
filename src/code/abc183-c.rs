// https://atcoder.jp/contests/abc183/tasks/abc183_c

use proconio::input;
use proconio::fastout;
use itertools::Itertools;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        T: [[usize; N]; N],
    }
    let mut ans = 0;
    for perm in (1..N).permutations(N-1) {
        let mut vec = vec![0];
        vec.extend(perm);
        vec.push(0);
        let mut time = 0;
        for i in 0..N {
            time += T[vec[i]][vec[i+1]];
        }
        if time == K {
            ans += 1;
        }
    }
    println!("{}", ans);
}