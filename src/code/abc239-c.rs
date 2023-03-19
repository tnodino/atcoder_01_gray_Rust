// https://atcoder.jp/contests/abc239/tasks/abc239_c

use proconio::input;
use proconio::fastout;

const DX: [isize; 8] = [-1, 1, -2, 2, -2, 2, -1, 1];
const DY: [isize; 8] = [2, 2, 1, 1, -1, -1, -2, -2];

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        x1: isize,
        y1: isize,
        x2: isize,
        y2: isize,
    }
    for i in 0..8 {
        let nx = x1 + DX[i];
        let ny = y1 + DY[i];
        for j in 0..8 {
            let mx = x2 + DX[j];
            let my = y2 + DY[j];
            if nx == mx && ny == my {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}