// Result:
// https://atcoder.jp/contests/adt_easy_20241226_2/submissions/61098592
// https://atcoder.jp/contests/adt_easy_20241226_2/submissions/61098746
// https://atcoder.jp/contests/adt_easy_20241226_2/submissions/61099222

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

fn _solve_b() {
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

fn solve_c() {
    let (xa, ya) = {
        let v = read_vec::<i32>();
        (v[0], v[1])
    };
    let (xb, yb) = {
        let v = read_vec::<i32>();
        (v[0], v[1])
    };
    let (xc, yc) = {
        let v = read_vec::<i32>();
        (v[0], v[1])
    };

    let ab = (((xa - xb).pow(2) + (ya - yb).pow(2)) as f64).sqrt();
    let bc = (((xb - xc).pow(2) + (yb - yc).pow(2)) as f64).sqrt();
    let ca = (((xc - xa).pow(2) + (yc - ya).pow(2)) as f64).sqrt();

    if bc <= ab && ca <= ab {
        let sum = bc.powf(2.0) + ca.powf(2.0);
        let ab_pow = ab.powf(2.0);
        if (sum - ab_pow).abs() < 0.0000001 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else if ab <= bc && ca <= bc {
        let sum = ab.powf(2.0) + ca.powf(2.0);
        let bc_pow = bc.powf(2.0);
        if (sum - bc_pow).abs() < 0.0000001 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else if ab <= ca && bc <= ca {
        let sum = ab.powf(2.0) + bc.powf(2.0);
        let ca_pow = ca.powf(2.0);
        if (sum - ca_pow).abs() < 0.0000001 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }
}

fn main() {
    solve_c();
}
