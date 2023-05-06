// https://atcoder.jp/contests/abc300/tasks/abc300_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    let mut A = Vec::new();
    for _ in 0..H {
        input! {
            a: String,
        }
        let a = a.chars().collect::<Vec<char>>();
        A.push(a);
    }
    let mut B = Vec::new();
    for _ in 0..H {
        input! {
            b: String,
        }
        let b = b.chars().collect::<Vec<char>>();
        B.push(b);
    }
    for _ in 0..H {
        for _ in 0..W {
            let mut C = vec![vec!['.'; W]; H];
            for i in 0..H {
                for j in 0..W {
                    C[i][(j+1)%W] = A[i][j];
                }
            }
            A = C;
            if A == B {
                println!("Yes");
                return;
            }
        }
        let mut C = vec![vec!['.'; W]; H];
        for i in 0..H {
            for j in 0..W {
                C[(i+1)%H][j] = A[i][j];
            }
        }
        A = C;
        if A == B {
            println!("Yes");
            return;
        }
    }
    println!("No");
}