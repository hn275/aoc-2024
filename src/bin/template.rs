use std::{
    env, fs,
    io::{self, BufRead},
};

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let file = fs::File::open(&args[1])?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
