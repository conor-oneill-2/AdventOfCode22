use std::fs::File;
use std::io::{BufRead,BufReader};

struct Rucksack {
    left: String,
    right: String
}

fn main() {
    let rucksacks = parse("./src/input.txt");
    println!("Part 1: {}",part1(&rucksacks));
    println!("Part 2: {}",part2(&rucksacks));
}

fn parse(filename: &str) -> Vec<Rucksack> {
    let mut rucksacks = Vec::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let halflen=line.as_ref().unwrap().len()/2;
        let rucksack=Rucksack {
            left: String::from(&line.as_ref().unwrap()[..halflen]),
            right: String::from(&line.as_ref().unwrap()[halflen..]),
        };
        rucksacks.push(rucksack);
    }
    return rucksacks;
}

fn char_value(asciival:&u8) -> u32 {
    if char::from(*asciival).is_lowercase() {
        //a=1, b=2, etc.
        return u32::from(*asciival)-96;
    }
    else { //is_uppercase()
        //A=27, B=28, etc.
        return u32::from(*asciival)-38;
    }
}

fn common_element_1(rucksack: &Rucksack) -> Option<u32> {
    for leftchar in rucksack.left.as_bytes() {
        for rightchar in rucksack.right.as_bytes() {
            if leftchar==rightchar {
                return Some(char_value(leftchar));
            }
        }
    }
    return None;
}

fn part1(rucksacks: &Vec<Rucksack>) -> u32 {
    let mut sum : u32 = 0;
    for rucksack in rucksacks {
        sum+=common_element_1(rucksack).unwrap();
    }
    return sum;
}

fn common_element_2(rucksack1: &Rucksack, rucksack2: &Rucksack, rucksack3: &Rucksack) -> Option<u32> {
    for char1 in (format!("{}{}",rucksack1.left,rucksack1.right)).as_bytes() {
        for char2 in (format!("{}{}",rucksack2.left,rucksack2.right)).as_bytes() {
            if char1==char2 {
                for char3 in (format!("{}{}",rucksack3.left,rucksack3.right)).as_bytes() {
                    if char1==char3 {
                        return Some(char_value(char1));
                    }
                }
            }
        }
    }
    return None;
}

fn part2(rucksacks: &Vec<Rucksack>) -> u32 {
    let mut i=0;
    let mut sum=0;
    while i<rucksacks.len()/3 {
        sum+=common_element_2(&rucksacks[3*i], &rucksacks[3*i+1], &rucksacks[3*i+2]).unwrap();    
        i+=1;
    }
    return sum;
}

