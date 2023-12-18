use std::{fs, str, time::Instant};
fn main() {
    let now = Instant::now();
    let input = fs::read_to_string("input.txt").unwrap();
    let lines:Vec<&str> = input.split("\n").collect();
    let mut part1 = 0;
    let mut part2 = 0;
    for line in lines {
        let arr:Vec<&str> = line.split(":").collect();
        let game_no = arr[0].split(" ").last().unwrap().parse::<i32>().unwrap();
        let sp2 = arr[1].split("; ");
        let mut games:Vec<(&str,i32)> = [].to_vec();
        
        for arr in sp2 {
            let sp3:Vec<&str> = arr.split(",").collect();
            for colour in sp3 {
                let sp4:Vec<&str> = colour.trim().split(" ").collect();
                games.push((sp4[1], sp4[0].parse::<i32>().unwrap()));
            }
        }

        let max_b = games.iter().filter(|tp| tp.0 == "blue").map(|tp| tp.1).chain([0]).max().unwrap();
        let max_r = games.iter().filter(|tp| tp.0 == "red").map(|tp| tp.1).chain([0]).max().unwrap();
        let max_g = games.iter().filter(|tp| tp.0 == "green").map(|tp| tp.1).chain([0]).max().unwrap();

        if max_b <= 14 && max_g <= 13 && max_r <= 12 {
            part1 += game_no;
        }        
        part2 += max_b * max_r * max_g;
    }
    
    let elapsed = now.elapsed().as_micros();
    println!("Part1: {part1} in {elapsed}us");
    println!("Part2: {part2}");

}
