// https://atcoder.jp/contests/abc244/tasks/abc244_c

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    let stdin = std::io::stdin();
    let mut source = proconio::source::line::LineSource::new(stdin.lock());

    input! {
        from &mut source,
        N: usize,
    }
    let mut flg = vec![true; N * 2 + 2];
    let mut idx = N * 2 + 1;
    loop {
        while !flg[idx] {
            idx -= 1;
        }
        println!("{}", idx);
        flg[idx] = false;
        input! {
            from &mut source,
            x: usize,
        }
        if x == 0 {
            return;
        }
        flg[x] = false;
    }
}