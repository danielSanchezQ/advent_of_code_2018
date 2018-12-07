use std::collections::BTreeMap;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn check_str_one(s : String) -> (i64, i64) {
    let mut visited : BTreeMap<char, i64> = BTreeMap::new();
    for c in s.chars() {
        let stat = visited.entry(c).or_insert(0);
        *stat += 1
    }
    let (mut two, mut three) : (i64, i64) = (0, 0);
    for (_, v) in visited.into_iter() {
        match v {
            2 => {two   = 1;}
            3 => {three = 1;}
            _ => {}
        }
    }
    (two, three)
}

fn check_str_two(s1: &String, s2: &String) -> Option<String> {
    let zipped : Vec<_> = s1.chars()
                            .zip(s2.chars())
                            .filter(|(a, b)| a == b)
                            .collect();
    if zipped.len() == s1.len()-1 {
        return Some(zipped.into_iter().map(|(a, _)| a).collect());
    }
    None
}

fn one(file_path : String) -> std::io::Result<()> {
    let file             = File::open(file_path)?;
    let buff_reader      = BufReader::new(file);
    let res : (i64, i64) = buff_reader.lines().into_iter()
        .map(|s|check_str_one(s.unwrap()))
        .fold((0,0),|(x, y), (x1, y1)| (x+x1, y+y1));
    let (x, y) = res;
    println!("Result: {:? }", x*y);
    Ok(())
}

fn two(file_path : String) -> std::io::Result<()> {
    let file                 = File::open(file_path)?;
    let buff_reader          = BufReader::new(file);
    let values : Vec<String> = buff_reader.lines()
        .into_iter()
        .map(|s|s.unwrap())
        .collect();
    for i in 0..values.len()-1 {
        for j in i..values.len() {
            match check_str_two(&values[i], &values[j]) {
                Some(x) => {println!("Result: {:? }", x); return Ok(())}
                None => {}
            }
        }
    }
    Ok(())
}


fn main() {
    one("input.txt".to_string());
    two("input.txt".to_string());
}
