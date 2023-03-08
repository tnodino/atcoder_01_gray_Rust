// https://atcoder.jp/contests/abc223/tasks/abc223_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut time = 0.;
    let mut A = Vec::new();
    let mut B = Vec::new();
    for _ in 0..N {
        input! {
            a: f64,
            b: f64,
        }
        time += a / b;
        A.push(a);
        B.push(b);
    }
    time /= 2.;
    let mut cnt = 0.;
    for i in 0..N {
        if time >= A[i] / B[i] {
            time -= A[i] / B[i];
            cnt += A[i];
        }
        else {
            println!("{}", cnt + time * B[i]);
            return;
        }
    }
}