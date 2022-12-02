use std::fs;
use regex::Regex;

//Some ugly case-by-case logic, but for this kind of problem any other way would add obfuscation without much gain

fn main() {
    let filename = "./src/input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Couldn't read file.");
    let rounds = get_rounds(&contents);
    //println!("{}",rounds.len());1
    let score1 = calculate_score_1(&rounds);
    println!("Part 1: {}",score1);
    let score2 = calculate_score_2(&rounds);
    println!("Part 2: {}",score2);
}

//Process text file
fn get_rounds(contents: &String) -> Vec<(char,char)> {
    let mut rounds = Vec::new();

    let re = Regex::new(r"([A-C]) ([X-Z])").unwrap();

    for cap in re.captures_iter(contents) {
        rounds.push((cap[1].chars().next().unwrap(),cap[2].chars().next().unwrap()));
    }

    return rounds;
}

//Part 1 Answer
fn calculate_score_1(rounds:&Vec<(char,char)>) -> u32 {
    let mut score = 0;
    for &round in rounds {
        match round.1 {
            'X' => score+=1,
            'Y' => score+=2,
            'Z' => score+=3,
            _ => panic!(),
        }

        if  (round.0=='A' && round.1=='Y') ||
            (round.0=='B' && round.1=='Z') ||
            (round.0=='C' && round.1=='X') {
                score+=6;
        }
        else if (round.0=='A' && round.1=='X') ||
            (round.0=='B' && round.1=='Y') ||
            (round.0=='C' && round.1=='Z') {
                score+=3;
        }
    }
    return score;
}


//Part 1 Answer
fn calculate_score_2(rounds:&Vec<(char,char)>) -> u32 {
    let mut score = 0;
    for &round in rounds {
        match round.1 {
            'X' => {}, //You lose, add nothing to score, and do nothing!
            'Y' => score+=3,
            'Z' => score+=6,
            _ => panic!(),
        }

        if  (round.0=='A' && round.1=='X') ||
            (round.0=='B' && round.1=='Z') ||
            (round.0=='C' && round.1=='Y') {
                score+=3;
        }
        else if (round.0=='A' && round.1=='Y') ||
            (round.0=='B' && round.1=='X') ||
            (round.0=='C' && round.1=='Z') {
                score+=1;
        }
        else {
            score+=2;
        }
    }
    return score;
}
