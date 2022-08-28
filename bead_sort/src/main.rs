use std::fs::File;
use std::io::prelude::*;

fn bead_sort(pre_sort: Vec<usize>) -> Vec<usize> {
    let max = pre_sort.iter().max().unwrap();

    // 1行ずつビーズを落としてそのたびに，vectorに落ちたビーズ分 +1 する．
    let mut bead_stack = vec![0; *max];
    for n in pre_sort.clone() {
        let tmp_stacked: Vec<isize> = bead_stack
        .iter()
        .enumerate()
        .map(|x| {
            let i = x.0;
            let j = x.1;
            if i < n {*j+1} else {*j}   // i はスタックの添字，j はスタックに入ってる値へのポインタ
        })
        .collect();
        bead_stack.clear();
        bead_stack.extend(&tmp_stacked);
    }

    // ↓繰り返し
    // ビーズ配列のうち 1 以上の要素を数え上げて soted に push
    // ビーズ配列全体から 1 引く
    let mut sorted = Vec::new();
    for _ in pre_sort.clone() {
        sorted.push(
            bead_stack
                .iter()
                .fold(0, |acc, x| if x > &0 {acc+1} else {acc})
        );
        let tmp_sacked: Vec<isize> = bead_stack
            .iter()
            .map(|x| x-1)
            .collect();
        bead_stack.clear();
        bead_stack.extend(&tmp_sacked);
    }
    sorted
}

fn main() {
    let mut f = File::open("OriginalNums.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong with reading the file");
    let o_nums: Vec<usize> = contents.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();

    // ソート
    let s_nums = bead_sort(o_nums);

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
