use std::fs;

/*Feeling pretty happy about today's code. Finally feel like the logic of how Rust works is starting to sink in,
and using a few more advanced Rust features like Generics and Traits.
Also the shortest number of lines of code to date*/

fn unique<T: std::cmp::PartialEq>(slice: &[T]) -> bool {
    let mut j:usize=0;
    let mut k:usize;
    while j<slice.len() {
        k=j+1;
        while k<slice.len() {
            if slice[j]==slice[k] {
                return false;
            }
            k+=1;
        }
        j+=1;
    }
    return true;
}

type ByteNum = usize;

trait CommDevice {
    fn start_marker(&self,marker_len:usize) -> ByteNum;

    fn start_packet_marker(&self) -> ByteNum {
        return self.start_marker(4);
    }

    fn start_message_marker(&self) -> ByteNum {
        return self.start_marker(14);
    }
}

struct Device {
    datastream: Vec<char>
}

impl CommDevice for Device {
    fn start_marker(&self, marker_len:usize) -> ByteNum {
        let mut i:ByteNum=0;
        while i<self.datastream.len()-marker_len {
            if unique(&self.datastream[i..i+marker_len]) {
                return i+marker_len;
            }
            i+=1;
        }
        panic!("Could not find start marker!");
    }
}

fn main() {
    let contents=fs::read_to_string("./src/input.txt").expect("input.txt not found!");
    let device = make_device(&contents);
    println!("Part 1: {}",device.start_packet_marker());
    println!("Part 2: {}",device.start_message_marker());
}

fn make_device(contents: &str) -> Device {
    return Device {datastream: contents.chars().collect::<Vec<char>>()};
}