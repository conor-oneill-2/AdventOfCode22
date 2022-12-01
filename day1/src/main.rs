use std::fs;

fn main() {
    let filename = "./src/input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Couldn't read file.");

    let mut calories = Vec::new();
    let mut start : usize = 0;
    let mut new_entry = true;

    loop {
        let (nextnum,size) = get_next_number(&contents[start..]);

        match nextnum {
            Some(num) => {
                if new_entry {
                    calories.push(num);
                    new_entry=false;
                }
                else {
                    let size = calories.len();
                    calories[size-1]+=num;    
                }
            },

            None => new_entry=true,
        }

        start+=size;
        
        if start>=contents.len() {
            break;
        }
    }
    /*println!("Part 1 Answer: {}",get_max(calories));*/
    println!("Part 2 Answer: {}",get_top_n_sum(calories));
}

//Returns the next number if line is number or None if empty line,
//as well as size to find next start index
fn get_next_number(unsearchedlines: &str) -> (Option<u32>,usize) {
    let bytes = unsearchedlines.as_bytes();

    for (i, &c) in bytes.iter().enumerate() {
        if c == b'\n' {
            if i==0 {
                return (None,1);
            }
            else {
                return (Some(unsearchedlines[0..i].parse::<u32>().unwrap()),i+1);
            }
        }
    }
    //Should never reach here, but this makes the compiler happy
    return (None,1);
}

//Part 1 answer
#[allow(dead_code)]
fn get_max(calories:Vec<u32>) -> u32 {
    let mut max : u32 = 0;
    for num in calories {
        if num>max {
            max=num;
        }
    }
    return max;
}

//Part 2 answer
fn get_top_n_sum(calories:Vec<u32>) -> u32 {
    //Was going to make TOPNUMS an input variable, but the compiler objects to creating the topn vector with a passed-in variable
    const TOPNUMS:usize=3;
    let mut topn : [u32;TOPNUMS] = [0;TOPNUMS];
    for val in calories {
        if val>topn[TOPNUMS-1] {
            topn[TOPNUMS-1]=val;
            let mut i=TOPNUMS-1;
            while i>0 {
                //Swap with previous element until list is sorted again
                if topn[i]>topn[i-1] {
                    let temp=topn[i];
                    topn[i]=topn[i-1];
                    topn[i-1]=temp;
                    i-=1;
                }
                else {
                    break;
                }
            }
        }
    }
    let mut sum = 0;
    for i in topn {
        sum+=i;
    }
    return sum;
}