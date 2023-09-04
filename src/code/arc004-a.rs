// https://atcoder.jp/contests/arc004/tasks/arc004_1

use proconio::input;
use proconio::fastout;
use libm::hypot;

fn max(x: f64, y: f64) -> f64 {
    return match x >= y {
        true => x,
        false => y,
    };
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut vec = Vec::new();
    for _ in 0..N {
        input! {
            x: f64,
            y: f64,
        }
        vec.push((x, y));
    }
    let mut ans = 0.;
    for i in 0..N {
        for j in i+1..N {
            ans = max(ans, hypot((vec[i].0 - vec[j].0).abs(), (vec[i].1 - vec[j].1).abs()));
        }
    }
    println!("{}", ans);
}