use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;
use std::vec::Vec;

fn one(file_path : String) -> std::io::Result<()> {
    let file = File::open(file_path)?;
    let buff_reader = BufReader::new(file);
    let res : i64 = buff_reader.lines().into_iter()
    .map(|s| {
        s.unwrap().parse::<i64>().unwrap()
    })
    .sum();
    println!("Result: {:? }", res);
    Ok(())
}

fn two(file_path : String) -> std::io::Result<()> {
    let file = File::open(file_path)?;
    let buff_reader = BufReader::new(file);
    let values : Vec<i64> = buff_reader.lines().into_iter()
    .map(|s| {
        s.unwrap().parse::<i64>().unwrap()
    })
    .collect();
    let mut visited : HashSet<i64> = HashSet::new();
    let mut res : i64 = 0;

    loop {
        for val in values.iter() {
            res += val;
            if visited.contains(&res) {
                println!("Result: {:? }", res);
                return Ok(())
            }
            visited.insert(res);
        }
    }
    println!("Could not find any repeated pattern");
    Ok(())
}


fn main() -> std::io::Result<()> {
    one("./input.txt".to_string())?;
    two("./input.txt".to_string())?;
    Ok(())
}
