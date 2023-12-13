use std::fs;

fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename) 
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    let file_path: &str = "src/part-1.txt";
    let lines = read_lines(file_path);
    //let mut sum: i32 = 0;
    for line in lines.iter() {
        let v: Vec<&str> = line.split(':').collect();
        println!("{:?}", v)
    }
    //println!("{}", sum)
}