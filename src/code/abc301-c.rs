// https://atcoder.jp/contests/abc301/tasks/abc301_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        T: String,
    }
    let N = S.len();
    let S = S.chars().collect::<Vec<char>>();
    let T = T.chars().collect::<Vec<char>>();
    let mut s = [0; 27];
    let mut t = [0; 27];
    for i in 0..N {
        if S[i] == '@' {
            s[26] += 1;
        }
        else {
            let idx = (S[i] as usize) - 97;
            s[idx] += 1;
        }
        if T[i] == '@' {
            t[26] += 1;
        }
        else {
            let idx = (T[i] as usize) - 97;
            t[idx] += 1;
        }
    }
    let mut c = Vec::new();
    for x in "atcoder".chars() {
        c.push((x as usize) - 97);
    }
    let mut cnts = 0;
    let mut cntt = 0;
    for i in 0..7 {
        if s[c[i]] > t[c[i]] {
            cntt += s[c[i]] - t[c[i]];
        }
        else if s[c[i]] < t[c[i]] {
            cnts += t[c[i]] - s[c[i]];
        }
        s[c[i]] = t[c[i]];
    }
    if cnts > s[26] || cntt > t[26] {
        println!("No");
        return;
    }
    s[26] -= cnts;
    t[26] -= cntt;
    for i in 0..=26 {
        if s[i] != t[i] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}