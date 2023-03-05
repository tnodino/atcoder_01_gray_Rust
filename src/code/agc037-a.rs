// https://atcoder.jp/contests/agc037/tasks/agc037_a

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn op(idx: usize, S: &Vec<char>) -> usize {
    if idx == 0 {
        return 1;
    }
    if idx == 1 {
        if S[idx] == S[idx-1] {
            return 1;
        }
        else {
            return 2;
        }
    }
    if idx == 2 {
        if S[idx] == S[idx-1] {
            return 2;
        }
        else {
            return op(idx - 1, S) + 1;
        }
    }
    if S[idx] == S[idx-1] {
        return op(idx - 3, S) + 2;
    }
    else {
        return op(idx - 1, S) + 1;
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    println!("{}", op(S.len() - 1, &S));
}