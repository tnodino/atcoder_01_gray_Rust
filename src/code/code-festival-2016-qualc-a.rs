// https://atcoder.jp/contests/code-festival-2016-qualc/tasks/codefestival_2016_qualC_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: String,
    }
    let N = s.len();
    let s = s.chars().collect::<Vec<char>>();
    let mut c = -1;
    for i in 0..N {
        if s[i] == 'C' {
            c = i as isize;
            break;
        }
    }
    let mut f = -1;
    for i in (0..N).rev() {
        if s[i] == 'F' {
            f = i as isize;
            break;
        }
    }
    if c == -1 || f == -1 || c > f {
        println!("No");
    }
    else {
        println!("Yes");
    }
}