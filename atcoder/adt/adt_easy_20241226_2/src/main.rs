// Result:
// https://atcoder.jp/contests/adt_easy_20241226_2/submissions/61098592
// https://atcoder.jp/contests/adt_easy_20241226_2/submissions/61098746

// https://qiita.com/penguinshunya/items/cd96803b74635aebefd6
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn _solve_a() {
    let n: String = read();

    let ones = n.chars().filter(|&c| c == '1').count();
    let twos: usize = n.chars().filter(|&c| c == '2').count();
    let threes: usize = n.chars().filter(|&c| c == '3').count();

    if ones == 1 && twos == 2 && threes == 3 {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn solve_b() {
    let _ = read::<i32>();
    let s = read::<String>();

    let ok_count = s.chars().filter(|&c| c == 'o').count();
    let ng_count = s.chars().filter(|&c| c == 'x').count();

    if ok_count > 0 && ng_count == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn main() {
    solve_b();
}
