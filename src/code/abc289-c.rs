// https://atcoder.jp/contests/abc289/tasks/abc289_c

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut a = Vec::new();
    for _ in 0..M {
        input! {
            x: usize,
            y: [usize; x],
        }
        a.push(y);
    }
    let mut ans = 0;
    'bit: for bit in 0..1<<M {
        let mut flg = vec![true; N];
        for i in 0..M {
            if bit & (1 << i) > 0 {
                for v in &a[i] {
                    flg[v-1] = false;
                }
            }
        }
        for i in 0..N {
            if flg[i] {
                continue 'bit;
            }
        }
        ans += 1;
    }
    println!("{}", ans);
}