use std::fs;


#[allow(non_camel_case_types)]
#[derive(Clone,Copy)]
enum Instruction {
    addx(i32),
    noop
}

impl Instruction {
    fn clockcycles(&self) -> usize {
        match self {
            Instruction::addx(_) => 2,
            Instruction::noop => 1
        }
    }
}

#[derive(Clone,Default)]
struct Device {
    instructions: Vec<Instruction>,
    ins_num: usize,
    x: i32,
    clockcycle:usize
}

impl Device {
    fn timestep(&mut self) {
        let instruction=self.instructions[self.ins_num];
        match instruction {
            Instruction::addx(val) => self.x+=val,
            Instruction::noop => {}
        }
        self.clockcycle+=instruction.clockcycles();
        self.ins_num+=1;
    }

    fn val_at_clock_cycle(&mut self,clock_cycle:usize) -> i32 {
        if self.clockcycle>clock_cycle {
            panic!("Already passed value!");
        }
        let mut lastx:i32=self.x;
        while self.clockcycle<clock_cycle {
            lastx=self.x;
            self.timestep();
        }

        return lastx;
    }
}


fn main() {
    let contents=fs::read_to_string("./src/input.txt").unwrap();
    let mut device = parse(&contents);
    println!("Part 1: {}",part1(&mut device.clone()));
    part2(&mut device);
}

fn parse(contents: &String) -> Device {
    let mut device=Device {
        instructions:Vec::new(),
        ins_num:0,
        x:1,
        clockcycle:0
    };
    for line in contents.split("\n") {
        if &line[..4] == "addx" {
            let instruction=Instruction::addx(line[5..].parse().unwrap());
            device.instructions.push(instruction);
        }
        else if &line[..4] == "noop" {
            device.instructions.push(Instruction::noop);
        }
        else {
            panic!();
        }
    }
    return device;
}

fn part1(device:&mut Device) -> i32 {
    let mut signalsum=0;
    let iter = (0..6).map(|x| 40*x+20);
    for i in iter {
        signalsum+=device.val_at_clock_cycle(i)*i as i32;
    }
    return signalsum;
}

fn part2(device:&mut Device) {
    let mut screen=vec![vec![' ';40];6];
    for i in 0..240 {
        let currentx=device.val_at_clock_cycle(i);
        if ((i%40) as i32-currentx).abs()<=1 {
            screen[i/40][i%40]='█';
        }
    }
    for row in screen {
        for y in row {
            print!("{}",y);
        }
        println!("\n");
    }
}