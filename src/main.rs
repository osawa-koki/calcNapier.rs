extern crate rug;
use rug::*;
use ops::Pow;
use std::fs;
use std::io::Write;

fn main() {
    let prec = 32 * 32 * 2048;
	let one = Float::with_val(prec, 1);
	let mut small:Float = Float::with_val(prec, 0.1).pow(1000);

    let mut file = fs::File::create("fruits.txt").unwrap();
    let contents = format!("{}", Float::from(one + small).pow(Integer::from(10).pow(1000)));
    file.write_all(contents.as_bytes()).unwrap();
}
