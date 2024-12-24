// Result: https://atcoder.jp/contests/adt_easy_20241224_2/submissions/61056886
// AC x11, WA x3

// https://qiita.com/penguinshunya/items/cd96803b74635aebefd6
fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

// Yes
// a < b <= c: a < b && b <= c
// b <= c < a: b <= c && c < a

// No
// a <= b > c: a <= b && c < b
// a > b <= c: b < a && b <= c

fn main() {
    let nums: Vec<i32> = read_vec();
    let a = nums[0];
    let b = nums[1];
    let c = nums[2];

    if a < b && b <= c {
        println!("Yes");
    } else if b <= c && c < a {
        println!("Yes");
    } else if a <= b && c < b {
        println!("No");
    } else if b < a && b <= c {
        println!("No");
    } else {
        println!("No");
    }
}
