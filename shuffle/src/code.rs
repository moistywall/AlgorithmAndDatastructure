use std::fs::File;
use std::io;
use std::io::Write;
use std::io::prelude::*;
use rand::Rng;

pub fn instructor() {
    println!("\"cargo run 1\" to generate sequence.");
    println!("\"cargo run 2\" to shuffle numbers in text/sequence.txt.");
}

pub fn gen_numbers() {
    println!("Enter the maximum value of the sequence you wish to create.");
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("Failed to read line.");
    let max = string.trim().parse().unwrap();

    let mut file = File::create("numbers.txt").unwrap();

    for number in 1..=max {
        print!("{} ", number);
        write!(file, "{} ",number).unwrap();
        if number % 100 == 0 {
            print!("\n");
            write!(file, "\n").unwrap();
        }
    }
}

// Fisher-Yates シャッフルアルゴリズムによる実装
pub fn shuffle() {
    println!("Shuffle sequel of numbers in file numbers.txt.");

    let mut f = File::open("numbers.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let mut number: Vec<i32> = contents.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();

    // let dst: Vec<String> = number.iter().map(|x| x.to_string()).collect();
    // println!("{}", dst.join(" "));

    // シャッフル
    let length = number.len();
    let mut rng = rand::thread_rng();
    for i in 0..length {
        let j = rng.gen_range(0..length);
        let tmp = number[i];
        number[i] = number[j];
        number[j] = tmp;
    }

    // ファイル出力
    let mut f = File::create("ShuffledNums.txt").unwrap();

    for i in 0..length {
        print!("{} ", number[i]);
        write!(f, "{} ", number[i]).unwrap();
        if i % 100 == 0 && i != 0 {
            print!("\n");
            write!(f, "\n").unwrap();
        }
    }
}