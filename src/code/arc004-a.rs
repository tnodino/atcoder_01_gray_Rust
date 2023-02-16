// https://atcoder.jp/contests/arc004/tasks/arc004_1

use proconio::input;
use proconio::fastout;
use libm::hypot;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut Points = vec![];
    for _ in 0..N {
        input! {
            x: f64,
            y: f64,
        }
        Points.push((x, y));
    }
    let mut ans = 0.0;
    for i in 0..N {
        for j in 0..N {
            let distance = hypot((Points[i].0 - Points[j].0).abs(), (Points[i].1 - Points[j].1).abs());
            if ans < distance {
                ans = distance;
            }
        }
    }
    println!("{}", ans);
}