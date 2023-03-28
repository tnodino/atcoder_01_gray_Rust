// https://atcoder.jp/contests/abc295/tasks/abc295_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        R: usize,
        C: usize,
    }
    let mut B = Vec::new();
    for _ in 0..R {
        input! {
            b: String,
        }
        let b = b.chars().collect::<Vec<char>>();
        B.push(b);
    }
    let mut ans = vec![vec!['#'; C]; R];
    for i in 0..R {
        for j in 0..C {
            if B[i][j] == '.' {
                ans[i][j] = '.';
            }
            else if B[i][j] == '#' {
                continue;
            }
            else {
                let num = B[i][j] as isize - 48;
                for k in 0..R {
                    for l in 0..C {
                        let a = i as isize;
                        let b = j as isize;
                        let c = k as isize;
                        let d = l as isize;
                        if (a - c).abs() + (b - d).abs() <= num {
                            ans[k][l] = '.';
                        }
                    }
                }
            }
        }
    }
    for i in 0..R {
        println!("{}", ans[i].iter().map(|&x| x.to_string()).collect::<String>());
    }
}