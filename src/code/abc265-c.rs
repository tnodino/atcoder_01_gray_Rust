// https://atcoder.jp/contests/abc265/tasks/abc265_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    let mut G = Vec::new();
    for _ in 0..H {
        input! {
            g: String,
        }
        let g = g.chars().collect::<Vec<char>>();
        G.push(g);
    }
    let mut flg = vec![vec![false; W]; H];
    let mut x = 0;
    let mut y = 0;
    flg[x][y] = true;
    loop {
        let mut out = false;
        match G[x][y] {
            'U' if x == 0 => out = true,
            'U' => x -= 1,
            'D' if x == H - 1 => out = true,
            'D' => x += 1,
            'L' if y == 0 => out = true,
            'L' => y -= 1,
            'R' if y == W - 1 => out = true,
            _ => y += 1,
        }
        if out {
            break;
        }
        if flg[x][y] {
            println!("-1");
            return;
        }
        flg[x][y] = true;
    }
    println!("{} {}", x + 1, y + 1);
}