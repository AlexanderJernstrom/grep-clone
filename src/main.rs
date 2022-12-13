//Grep clone let's go
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let mut file_path = String::new();
    let mut search_string = String::new();

    println!("Enter the path of the file: ");
    std::io::stdin().read_line(&mut file_path).unwrap();

    println!("Enter the search string: ");
    std::io::stdin().read_line(&mut search_string).unwrap();

    let trimmed_path = file_path.trim();

    let path = Path::new(&trimmed_path);
    let f = File::open(path);

    let file = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let reader = BufReader::new(file);

    for (l) in reader.lines() {
        let mut line = match l {
            Ok(line) => line,
            Err(error) => panic!("Problem opening the line: {:?}", error),
        };

        if line.contains(&search_string.trim()) {
            println!("'{}' was found here: {}", search_string.trim(), line);
        }
    }
}
