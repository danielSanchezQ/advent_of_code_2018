extern crate regex;


use std::vec::Vec;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashSet;
use regex::Regex;



fn one(fpath: String) -> std::io::Result<()> {
    let file             = File::open(fpath)?;
    let buff_reader      = BufReader::new(file);
    let mut grid: Vec<Vec<i32>> = (0..1000)
        .map(
        |_| (0..1000).map(|_| 0).collect()
        ).collect();
    let mut claims: Vec<Vec<HashSet<i32>>> = (0..1000)
        .map(
            |_| (0..1000).map(|_| HashSet::new()).collect()
        ).collect();
    let re = Regex::new(r"#(?P<id>\d+) @ (?P<row>\d+),(?P<col>\d+): (?P<row_size>\d+)x(?P<col_size>\d+)").unwrap();
    let lines : Vec<String> = buff_reader.lines().into_iter().map(|s|s.unwrap()).collect();
    for l in  &lines {
        let data     = re.captures(l.as_str()).unwrap();
        let id       = data["id"].parse::<i32>().unwrap();
        let row      = data["row"].parse::<i32>().unwrap();
        let col      = data["col"].parse::<i32>().unwrap();
        let row_size = data["row_size"].parse::<i32>().unwrap();
        let col_size = data["col_size"].parse::<i32>().unwrap();
        for r in (row..row+row_size) {
            for c in (col..col+col_size) {
                grid[r as usize][c as usize] += 1;
                claims[r as usize][c as usize].insert(id);
            }
        } 
    }
    //This loops correspond to part 2 checking
    'outer : for l in &lines {
        let data     = re.captures(l.as_str()).unwrap();
        let id       = data["id"].parse::<i32>().unwrap();
        let row      = data["row"].parse::<i32>().unwrap();
        let col      = data["col"].parse::<i32>().unwrap();
        let row_size = data["row_size"].parse::<i32>().unwrap();
        let col_size = data["col_size"].parse::<i32>().unwrap();
        for r in (row..row+row_size) {
            for c in (col..col+col_size) {
                if grid[r as usize][c as usize] != 1 {
                    continue 'outer
                }
            }
        } 
        println!("Claim id: {:? }", id);
        break 'outer
    }
    let result : i32 = grid.into_iter()
                           .map(
                               |v| -> i32 {
                                    v.into_iter()
                                     .map(|x| if x >= (2 as i32) {return 1} else {0})
                                     .sum()
                                }
                            )
                            .sum();
    println!("Square meters: {:? }", result);

    Ok(())
}

fn main() {
    one("input.txt".to_string());
}
