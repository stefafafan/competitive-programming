// Result: https://atcoder.jp/contests/adt_easy_20241225_1/tasks/abc308_a, https://atcoder.jp/contests/adt_easy_20241225_1/tasks/abc219_a

// https://qiita.com/penguinshunya/items/cd96803b74635aebefd6
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn solve_a() {
    let nums: Vec<i32> = read_vec();

    let mut res = "Yes";

    for i in 0..nums.len() {
        if i != 0 && nums[i - 1] > nums[i] {
            res = "No";
            break;
        }
        if nums[i] < 100 || 675 < nums[i] {
            res = "No";
            break;
        }
        if nums[i] % 25 != 0 {
            res = "No";
            break;
        }
    }

    println!("{}", res);
}

fn solve_b() {
    // 0 点以上40 点未満のとき、初級
    // 40 点以上70 点未満のとき、中級
    // 70 点以上90 点未満のとき、上級
    // 90 点以上のとき、エキスパート
    let num = read::<i32>();

    if num < 40 {
        println!("{}", 40 - num);
    } else if num < 70 {
        println!("{}", 70 - num);
    } else if num < 90 {
        println!("{}", 90 - num);
    } else {
        println!("expert");
    }
}

fn main() {
    solve_b();
}
