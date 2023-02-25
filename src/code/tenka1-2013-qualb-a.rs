// https://atcoder.jp/contests/tenka1-2013-qualb/tasks/tenka1_2013_qualB_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let mut array = [
        "ABGGEGBCFEBFBAF",
        "FFGFACCCECDGCDGAFFFACGDA",
        "EEDCAEAFBDDEEDGGA",
        "GDCAGFFAACBGEDBAFBCDECGAE",
        "EDB",
        "GADGADEDBCGABDDCBBDBEAD",
        "GADBB",
        "DFCE",
        "BFGCGCBEDC",
        "EDGADBGGDDFEEGGFDGCAFBFGFAAD",
        "DDAEBGACDFDGDAB",
        "EEDCECFFAE",
        "ADDBEEABFEAB",
        "FEEBFDGAADAE",
        "GB"
    ];
    array.sort();
    println!("{}", array[6]);
}