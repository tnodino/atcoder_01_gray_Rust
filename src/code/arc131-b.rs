// https://atcoder.jp/contests/arc131/tasks/arc131_b

use proconio::input;
use proconio::fastout;

const DX: [usize; 4] = [!0, 1, 0, 0];
const DY: [usize; 4] = [0, 0, !0, 1];

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    let mut c = Vec::new();
    for _ in 0..H {
        input! {
            x: String,
        }
        let x = x.chars().collect::<Vec<char>>();
        c.push(x);
    }
    for i in 0..H {
        for j in 0..W {
            if c[i][j] != '.' {
                print!("{}", c[i][j]);
                continue;
            }
            let mut flg = [true; 6];
            for d in 0..4 {
                let nx = i.wrapping_add(DX[d]);
                let ny = j.wrapping_add(DY[d]);
                if H <= nx || W <= ny {
                    continue;
                }
                if c[nx][ny] == '.' {
                    continue;
                }
                let x = c[nx][ny] as usize - 48;
                flg[x] = false;
            }
            for k in 1..=5 {
                if flg[k] {
                    print!("{}", k);
                    c[i][j] = (k as u8 + 48) as char;
                    break;
                }
            }
        }
        println!();
    }
}