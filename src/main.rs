use std::env;
use std::io::Read;

fn print_help() {
    println!(
        "
The purpose of this program is the find the binary difference of 2 files of the same size.
They can be supplied in the form
    bindiff [file1] [file2]
    "
    );
}

fn bindiff(fn1: &str, fn2: &str) -> Result<u32, String> {
    let bytes1: Vec<u8> = std::fs::File::open(fn1)
        .unwrap()
        .bytes()
        .collect::<Result<_, _>>()
        .unwrap();
    let bytes2: Vec<u8> = std::fs::File::open(fn2)
        .unwrap()
        .bytes()
        .collect::<Result<_, _>>()
        .unwrap();
    if bytes1.len() != bytes2.len() {
        return Err("Files are not the same size.".to_string());
    }
    Ok(bytes1
        .into_iter()
        .zip(bytes2.into_iter())
        .map(|(x, y)| (x ^ y).count_ones())
        .sum::<u32>())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        //may plan to allow base64 files (?)
        print_help();
        return;
    }
    let fn1 = &args[1];
    let fn2 = &args[2];

    let bindiff_result = bindiff(&fn1, &fn2);
    if let Err(err) = &bindiff_result {
        println!("Error: {}", err);
        return;
    }

    let bindiff_result = bindiff_result.unwrap();
    println!("The two files differ by {} bit(s).", bindiff_result)
}
