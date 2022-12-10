use std::fs;

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: i16,
    y:i16
}

impl Point {
    fn touching(&self,other:&Point) -> bool {
        if (self.x - other.x).abs() > 1 {
            return false;
        }
        if (self.y - other.y).abs() > 1 {
            return false;
        }
        return true;
    }

    fn pull_other(&self,other:&mut Point) {
        if (self.x-other.x) > 0 {
            other.x+=1;
        }
        else if (self.x-other.x)<0 {
            other.x-=1;
        }

        if (self.y-other.y) > 0 {
            other.y+=1;
        }
        else if (self.y-other.y)<0 {
            other.y-=1;
        }
    }
}


fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("Couldn't find input.txt");
    let head_positions = parse(&contents);
    println!("Part 1: {}",part1(&head_positions));
    println!("Part 2: {}",part2(head_positions));
}

fn parse(contents: &String) -> Vec<Point> {
    let mut last_head_pos= Point{x:0,y:0};
    let mut head_positions = vec![last_head_pos];

    let mut dir:char;
    let mut dist:i16;
    for line in contents.split("\n") {
        dir=line.chars().next().unwrap();
        dist=line[2..].parse().unwrap();

        for _ in 0..dist {
            if dir=='L' {
                last_head_pos.x-=1;
            }
            else if dir=='R' {
                last_head_pos.x+=1;
            }
            else if dir=='U' {
                last_head_pos.y+=1;
            }
            else if dir=='D' {
                last_head_pos.y-=1;
            }
            else {
                panic!("dir should be L,R,U or D!");
            }
            head_positions.push(last_head_pos);
        }
    }


    return head_positions;
}

fn next_positions(prev_positions: &Vec<Point>) -> Vec<Point> {
    let mut last_tail_pos = Point{ x:0, y:0 };
    let mut tail_positions = vec![last_tail_pos];

    for pos in prev_positions {
        if !pos.touching(&last_tail_pos) {
            pos.pull_other(&mut last_tail_pos);
            tail_positions.push(last_tail_pos);
        }
    }

    return tail_positions;
}

fn unique_points(knot_positions: &Vec<Point>) -> i16 {
    let mut uniquepoints=0;

    for i in 0..knot_positions.len() {
        let mut new_point=true;
        for j in 0..i {
            if knot_positions[i]==knot_positions[j] {
                new_point=false;
                break;
            }
        }
        if new_point {
            uniquepoints+=1;
        }
    }

    return uniquepoints;
}

fn part1(head_positions: &Vec<Point>) -> i16 {
    let tail_positions = next_positions(&head_positions);
    let uniquepoints = unique_points(&tail_positions);
    return uniquepoints;
}

fn part2(head_positions: Vec<Point>) -> i16 {
    let rope_len = 10;
    let mut prev_knot_positions=head_positions;
    for _ in 0..rope_len-1 {
        prev_knot_positions=next_positions(&prev_knot_positions);
    }
    let unique_points = unique_points(&prev_knot_positions);
    return unique_points;
}