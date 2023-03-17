// https://atcoder.jp/contests/abc222/tasks/abc222_c

use proconio::input;
use proconio::fastout;

fn rsp(x: char, y: char) -> usize {
    return match (x, y) {
        ('G', 'C') => 1,
        ('C', 'P') => 1,
        ('P', 'G') => 1,
        ('C', 'G') => 2,
        ('P', 'C') => 2,
        ('G', 'P') => 2,
        (_, _) => 0,
    };
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut A = Vec::new();
    for _ in 0..N*2 {
        input! {
            a: String,
        }
        let a = a.chars().collect::<Vec<char>>();
        A.push(a);
    }
    let mut ans = Vec::new();
    for i in 0..N*2 {
        ans.push((0, i));
    }
    for k in 0..M {
        for i in (0..N*2).step_by(2) {
            let ret = rsp(A[ans[i].1][k], A[ans[i+1].1][k]);
            if ret == 1 {
                ans[i].0 += 1;
            }
            if ret == 2 {
                ans[i+1].0 += 1;
            }
        }
        ans.sort_by(|a, b| if a.0 != b.0 {
            b.0.cmp(&a.0)
        }
        else {
            a.1.cmp(&b.1)
        });
    }
    for i in 0..N*2 {
        println!("{}", ans[i].1 + 1);
    }
}