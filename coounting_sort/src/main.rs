use std::fs::File;
use std::io::prelude::*;

// 計数ソート
fn counting_sort(a: Vec<i32>) ->  Vec<i32> {
    let mut c = [0; 15000];

    let alength = a.len();
    for i in 0..alength {
        let tmp = a[i] as usize;
        c[tmp] += 1;
    }

    let clength = c.len();
    for i in 1..clength {
        c[i] += c[i-1];
    }

    

    let mut b = [0; 15000];
    for i in 0..alength {
        let atmp = a[i] as usize;
        let ctmp = c[atmp] as usize;
        b[ctmp - 1] = a[i];
        c[atmp] -= 1;
    }

    let mut sorted = Vec::new();
    for i in 0..alength {
        sorted.push(b[i]);
    }

    sorted
}

fn main() {
    // 整列前の数列の入ったファイルを読み込む．
    let mut f = File::open("OriginalNums.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong with reading the file");
    let o_nums: Vec<i32> = contents.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();

    // ソート
    let s_nums = counting_sort(o_nums);

    // ファイル出力
    let mut f = File::create("SortedNums.txt").unwrap();

    let length = s_nums.len();
    for i in 0..length {
        print!("{} ", s_nums[i]);
        write!(f, "{} ", s_nums[i]).unwrap();
        if i % 100 == 99 {
            print!("\n");
            write!(f, "\n").unwrap();
        }
    }
}