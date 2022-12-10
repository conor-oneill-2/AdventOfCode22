use std::fs;
use regex::Regex;

struct Movement {
    quantity: usize,
    source: usize,
    dest: usize
}

fn main() {
    let mut cargo = parse_cargo("./src/cargo.txt");
    let movements = parse_movements("./src/movements.txt");
    println!("Part 1: {}",part1(&mut cargo.clone(), &movements));
    println!("Part 2: {}",part2(&mut cargo, &movements));
}

fn parse_cargo(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).unwrap();
    let mut cargo = vec![String::new();9];

    let mut i=1;
    let mut stacknum=0;

    while i<contents.len() {
        
        let char = &contents[i..i+1];

        if i%4==1 {
            if char!=" " {
                cargo[stacknum]+=&contents[i..i+1];
            }    
        }

        else { //i%4==3
            if char=="\n" {
                stacknum=0;
            }
            else { //char==" "
                stacknum+=1;
            }
        }

        i+=2;
    }
    let mut i:usize=0;
    while i<9 {
        cargo[i]=cargo[i].chars().rev().collect::<String>();
        i+=1;
    }
    /* Couldn't get this to work, just did it another way
    Not sure what I'm doing wrong
    for mut stack in &mut cargo {
        stack= &mut stack.chars().rev().collect::<String>();
    }
    */
    return cargo;
    
}

fn parse_movements(filename: &str) -> Vec<Movement> {
    let contents = fs::read_to_string(filename).unwrap();
    let mut movements = Vec::new();
    let re=Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
    
    for cap in re.captures_iter(&contents) {
        let movement = Movement {
            quantity: cap[1].parse().unwrap(),
            source: cap[2].parse::<usize>().unwrap()-1,
            dest: cap[3].parse::<usize>().unwrap()-1,
        };
        movements.push(movement);
    }
    return movements;
}


fn part1(cargo: &mut Vec<String>, movements: &Vec<Movement>) -> String {
    for movement in movements {
        //clone is unnecessary here logically, but the compiler yells otherwise
        //For long strings, this could cause slowdown, so figure out a better way to do it
        let source=cargo[movement.source].clone();
        let mut lastboxesiter = source.chars().rev();
        let mut i=0;
        while i<movement.quantity {
            cargo[movement.dest]+=&lastboxesiter.next().unwrap().to_string();
            i+=1;
        }
        cargo[movement.source]=String::from(&cargo[movement.source][..(&cargo[movement.source].len()-movement.quantity)]);
        //let tempstack=&String::from(&source[(source.len()-movement.quantity)..]);
        //println!("Temp: {}",tempstack);
        //print!("Old: {}",source);
        //cargo[movement.source]=String::from(&source[..(source.len()-movement.quantity)]);
        //println!(" {}",cargo[movement.source]);
        //print!("New: {}",cargo[movement.dest]);
        //cargo[movement.dest]+=tempstack;
        //println!(" {}\n",cargo[movement.dest]);
    }

    let mut result = String::new();
    for i in cargo {
        //println!("{}",i);
        result+=&i[i.len()-1..];
    }
    return result;
}

fn part2(cargo: &mut Vec<String>, movements: &Vec<Movement>) -> String {
    for movement in movements {
        let source=cargo[movement.source].clone();
        let tempstack=&String::from(&source[(source.len()-movement.quantity)..]);
        //println!("Temp: {}",tempstack);
        //print!("Old: {}",source);
        cargo[movement.source]=String::from(&source[..(source.len()-movement.quantity)]);
        //println!(" {}",cargo[movement.source]);
        //print!("New: {}",cargo[movement.dest]);
        cargo[movement.dest]+=tempstack;
        //println!(" {}\n",cargo[movement.dest]);
    }
    let mut result = String::new();
    for i in cargo {
        //println!("{}",i);
        result+=&i[i.len()-1..];
    }
    return result;
}