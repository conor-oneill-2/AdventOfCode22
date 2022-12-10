use regex::Regex;
use std::fs;

struct ElfWork {
    min: u8,
    max: u8
}

struct ElfPair {
    left: ElfWork,
    right: ElfWork
}

fn main() {
    let elf_pairs=parse("./src/input.txt");
    println!("Part 1: {}",part1(&elf_pairs));
    println!("Part 2: {}",part2(&elf_pairs));
}

fn parse(filename: &str) -> Vec<ElfPair> {
    let contents=fs::read_to_string(filename).unwrap();
    let mut elf_pairs = Vec::new();

    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    for cap in re.captures_iter(&contents) {
        let elf_pair = ElfPair {
            left : ElfWork { min: cap[1].parse().unwrap(), max: cap[2].parse().unwrap() },
            right: ElfWork { min: cap[3].parse().unwrap(), max: cap[4].parse().unwrap() }
        };
        elf_pairs.push(elf_pair);
    }

    return elf_pairs;
}

fn fullycontained(elf_pair: &ElfPair) -> bool {
    if elf_pair.left.min<elf_pair.right.min {
        return elf_pair.left.max>=elf_pair.right.max;
    }
    else if elf_pair.left.min>elf_pair.right.min{
        return elf_pair.left.max<=elf_pair.right.max;
    }
    else { //elf_pair.left.min==elf_pair.right.min
        //Logically, one is guaranteed to be contained in the other
        return true;
    }
}

fn part1(elf_pairs: &Vec<ElfPair>) -> u32 {
    let mut sum=0;
    for pair in elf_pairs {
        if fullycontained(pair) {
            sum+=1;
        }
    }
    return sum;
}

fn partiallycontained(elf_pair: &ElfPair) -> bool {
    if elf_pair.left.min<elf_pair.right.min {
        return elf_pair.left.max>=elf_pair.right.min;
    }
    else if elf_pair.left.min>elf_pair.right.min{
        return elf_pair.left.min<=elf_pair.right.max;
    }
    else { //elf_pair.left.min==elf_pair.right.min
        //Logically, one is guaranteed to be contained in the other
        return true;
    }
}

fn part2(elf_pairs: &Vec<ElfPair>) -> u32 {
    let mut sum=0;
    for pair in elf_pairs {
        if partiallycontained(pair) {
            sum+=1;
        }
    }
    return sum;
}
