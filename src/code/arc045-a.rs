// https://atcoder.jp/contests/arc045/tasks/arc045_a

use proconio::input;
use proconio::fastout;
use proconio::is_stdin_empty;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let mut ans = Vec::new();
    loop {
        if is_stdin_empty() {
            break;
        }
        input! {
            S: String,
        }
        ans.push(match S.as_str() {
            "Left" => "<",
            "Right" => ">",
            "AtCoder" => "A",
            _ => unreachable!()
        });
    }
    println!("{}", ans.join(" "));
}