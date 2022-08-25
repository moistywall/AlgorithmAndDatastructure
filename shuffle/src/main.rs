mod code;

fn main() {
    let mut no = 0;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        no = args[1].parse::<i32>().unwrap();
    }
    run(no);
}

fn run(no: i32) {
    match no {
        0 => code::instructor(),
        1 => code::gen_numbers(),
        2 => code::shuffle(),
        _ => {},
    }
}