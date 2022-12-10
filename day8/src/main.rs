use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("Couldn't find input.txt!");
    let trees = parse(&contents);
    println!("Part 1: {}",part1(&trees));
    println!("Part 2: {}",part2(&trees));
}

fn parse(contents: &str) -> Vec<Vec<u8>> {
    let mut treefield = Vec::new();
    for column in contents.split("\n") {
        let mut treecolumn=Vec::new();
        for char in column.chars() {
            treecolumn.push(char.to_digit(10).unwrap() as u8);
        }
        treefield.push(treecolumn);
    }
    return treefield;
}

fn part1(trees:&Vec<Vec<u8>>) -> u32 {
    let mut visible=0;
    let mut i=0;
    let mut j;
    while i<trees[0].len() {
        j=0;
        while j<trees.len() {
            if isvisible(trees, i, j) {
                visible+=1;
            }
            j+=1;
        }
        i+=1;
    }
    return visible;
}

#[allow(dead_code)]
fn isvisiblecase(trees:&Vec<Vec<u8>>,column:usize,row:usize,forward:bool,horizontal:bool) -> bool {
    let k:usize;
    let end:usize;
    if forward {
        k=0;
        if horizontal {
            end=column;
        }
        else {
            end=row;
        }
    }
    else {
        if horizontal {
            k=column+1;
            end=trees[0].len();
        }
        else {
            k=row+1;
            end=trees.len();
        }
    }
    let mut treecomp:u8;
    while k<end {
        if horizontal {
            treecomp=trees[k][column];
        }
        else {
            treecomp=trees[row][k];
        }

        if treecomp>=trees[row][column] {
            return false;
        }
    }
    return true;
}

fn isvisible(trees:&Vec<Vec<u8>>,column:usize,row:usize) -> bool {
    //Left Case
    let mut k=0;
    let mut visible=true;
    while k<row {
        if trees[k][column]>=trees[row][column] {
            //Not visible from left, go on to next case
            visible=false;
            break;
        }
        k+=1;
    }
    if visible {
        return true;
    }

    //Right Case
    k=trees[0].len()-1;
    visible=true;
    while k>row {
        if trees[k][column]>=trees[row][column] {
            //Not visible from left, go on to next case
            visible=false;
            break;
        }
        k-=1;
    }
    if visible {
        return true;
    }

    //Top Case
    k=0;
    visible=true;
    while k<column {
        if trees[row][k]>=trees[row][column] {
            //Not visible from left, go on to next case
            visible=false;
            break;
        }
        k+=1;
    }
    if visible {
        return true;
    }

    //Bottom Case
    k=trees.len()-1;
    visible=true;
    while k>column {
        if trees[row][k]>=trees[row][column] {
            //Not visible from left, go on to next case
            visible=false;
            break;
        }
        k-=1;
    }
    if visible {
        return true;
    }

    return false;
}

fn part2(trees:&Vec<Vec<u8>>) -> usize {
    let mut maxscenic=0;
    let mut i=0;
    let mut j;
    while i<trees[0].len() {
        j=0;
        while j<trees.len() {
            let scenicscore=scenic_score(trees,i,j);
            if scenicscore>maxscenic {
                maxscenic=scenicscore;
            }
            j+=1;
        }
        i+=1;
    }
    return maxscenic;
}

fn scenic_score(trees:&Vec<Vec<u8>>,column:usize,row:usize)->usize {
    let _test=trees[column][row];
    let mut scenic = 1;

    //Left Case
    if column==0 {
        return 0;
    }
    let mut k=0;
    while k<row {
        if trees[column][row-k-1]>=trees[column][row] {
            //Not visible from left, go on to next case
            k+=1;
            break;
        }
        k+=1;
    }
    scenic*=k;

    //Right Case
    if column==trees.len()-1 {
        return 0;
    }
    k=0;
    while k<trees[0].len()-row-1 {
        if trees[column][row+k+1]>=trees[column][row] {
            //Not visible from left, go on to next case
            k+=1;
            break;
        }
        k+=1;
    }
    scenic*=k;

    //Top Case
    if row==0 {
        return 0;
    }
    k=0;
    while k<column {
        if trees[column-k-1][row]>=trees[column][row] {
            //Not visible from left, go on to next case
            k+=1;
            break;
        }
        k+=1;
    }
    scenic*=k;

    //Bottom Case
    if row==trees.len()-1 {
        return 0;
    }
    k=0;
    while k<trees.len()-column-1 {
        if trees[column+k+1][row]>=trees[column][row] {
            //Not visible from left, go on to next case
            k+=1;
            break;
        }
        k+=1;
    }
    scenic*=k;

    return scenic;
}