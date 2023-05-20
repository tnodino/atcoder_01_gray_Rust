// https://atcoder.jp/contests/abc298/tasks/abc298_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
    }
    let M = 200_000;
    let mut vec2 = vec![Vec::new(); N+1];
    let mut vec3 = vec![Vec::new(); M+1];
    for _ in 0..Q {
        input! {
            query: usize,
        }
        if query == 1 {
            input! {
                i: usize,
                j: usize,
            }
            vec2[j].push(i);
            vec3[i].push(j);
        }
        else if query == 2 {
            input! {
                i: usize,
            }
            vec2[i].sort();
            println!("{}", vec2[i].iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
        }
        else {
            input! {
                i: usize,
            }
            vec3[i].sort();
            vec3[i].dedup();
            println!("{}", vec3[i].iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
        }
    }
}