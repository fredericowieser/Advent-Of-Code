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
    let mut sum: i32 = 0;
    for line in lines.iter() {
        let cvec: Vec<char> = line.chars().collect();
        let mut dvec: Vec<char> = Vec::new();
        let digits: Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        for c in cvec.iter() {
            if digits.contains(&c) {
                dvec.push(*c);
            }
        }
        let num: i32 = format!("{}{}", &dvec[0], &dvec[dvec.len() - 1])
            .parse::<i32>()
            .unwrap();
        sum += num;
    }
    println!("{}", sum)
}