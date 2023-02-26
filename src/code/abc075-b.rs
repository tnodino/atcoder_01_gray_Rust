// https://atcoder.jp/contests/abc075/tasks/abc075_b

use proconio::input;
use proconio::fastout;

const DX: [usize; 8] = [!0, !0, !0, 0, 0, 1, 1, 1];
const DY: [usize; 8] = [!0, 0, 1, !0, 1, !0, 0, 1];

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        S: [String; H],
    }
    let S: Vec<Vec<char>> = S.iter().map(|x| x.chars().collect::<Vec<char>>()).collect();
    for i in 0..H {
        for j in 0..W {
            if S[i][j] == '#' {
                print!("#");
                continue;
            }
            let mut cnt = 0;
            for d in 0..8 {
                let nx = i.wrapping_add(DX[d]);
                let ny = j.wrapping_add(DY[d]);
                if H <= nx || W <= ny {
                    continue;
                }
                if S[nx][ny] == '#' {
                    cnt += 1;
                }
            }
            print!("{}", cnt);
        }
        println!();
    }
}