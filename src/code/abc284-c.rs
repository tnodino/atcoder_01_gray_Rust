// https://atcoder.jp/contests/abc284/tasks/abc284_c

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn dfs(pos: usize, G: &mut Vec<Vec<usize>>, flg: &mut Vec<bool>) {
    for i in 0..G[pos].len() {
        let nxt = G[pos][i];
        if flg[nxt] {
            flg[nxt] = false;
            dfs(nxt, G, flg);
        }
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut G = vec![Vec::new(); N];
    for _ in 0..M {
        input! {
            u: usize,
            v: usize,
        }
        G[u-1].push(v-1);
        G[v-1].push(u-1);
    }
    let mut flg = vec![true; N];
    let mut ans = 0;
    for i in 0..N {
        if flg[i] {
            flg[i] = false;
            ans += 1;
            dfs(i, &mut G, &mut flg);
        }
    }
    println!("{}", ans);
}