use std::{fs, collections::HashMap, rc::Rc, cell::RefCell};

//Life is pain. If you asked me how the parser worked, I could not tell you

trait FileSize {
    fn size(&self) -> usize;
}

type File = usize;

impl FileSize for File {
    fn size(&self) -> usize {
        return *self;
    }
}

#[derive(Default)]
struct Directory {
    files : HashMap<String,File>,
    subdirs : HashMap<String,Rc<RefCell<Directory>>>,
    parent : Option<Rc<RefCell<Directory>>>
}

impl FileSize for Directory {
    fn size(&self) -> usize {
        let mut size:usize=0;
        for (_,file) in &self.files {
            size+=file.size();
        }
        for (_,subdir) in &self.subdirs {
            size+=subdir.as_ref().borrow().size();
        }
        return size;
    }
}

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("File not found!");
    let root = parse(&contents);
    println!("Part 1: {}",part1(&root));
    let minfoldersize = root.borrow().size()-40000000;
    println!("{}",minfoldersize);
    println!("Part 2: {}",part2(&root,minfoldersize).unwrap());
}

fn parse(contents: &String) -> Rc<RefCell<Directory>> {
    let root = Rc::new(RefCell::new(Directory::default()));
    //let mut workingdir = &mut root;
    let mut workingdir = Rc::clone(&root);
    let mut currentcommand:&str="";
    for line in contents.split("\n") {
        if &line[0..1]=="$" {
            if &line[2..4]=="cd" {
                workingdir=parse_dir(&line[5..].to_string(), &workingdir);
            }
            else { //if &line[2..4]=="ls"
                currentcommand="ls";
            }
        }
        else {//Is printed out as a result of currentcommand
            //Technically unnecessary, but useful if another command is added later
            //if it also gives return values
            if currentcommand=="ls" {
                //Represents file or directory
                if &line[0..3]=="dir" {
                    let newdir = Rc::new(RefCell::new(Directory {
                        files:HashMap::new(),
                        subdirs:HashMap::new(),
                        parent:Some(Rc::clone(&workingdir)),
                    }));
                    workingdir.borrow_mut().subdirs.insert(
                        line[4..].to_string(),
                        Rc::clone(&newdir)
                    );
                }
                else { //Parse file
                    let mut parts=line.split(" ");
                    let file:File=parts.next().unwrap().parse().unwrap();
                    workingdir.borrow_mut().files.insert(
                        parts.next().unwrap().to_string(),
                        file
                    );
                }
            }
        }
    }
    return root;
}

fn parse_dir(dir: &String, workingdir: &Rc<RefCell<Directory>>) -> Rc<RefCell<Directory>> {
    if dir==".." {
        return Rc::clone(&workingdir.borrow().parent.as_ref().unwrap());
    }
    else {
        return Rc::clone(&workingdir.borrow().subdirs[dir]);
    }
}

fn part1(dir: &Rc<RefCell<Directory>>) -> usize {
    let mut size=0;
    for (_,subdir) in dir.borrow().subdirs.iter() {
        size+=part1(subdir);
    }
    if dir.borrow().size()<=100000 {
        size+=dir.borrow().size();
    }

    return size;
}

fn part2(dir: &Rc<RefCell<Directory>>,minfoldersize: usize) -> Option<usize> {
    //capacity = 70 000 000
    //space_needed = 30 000 000
    //free_space = capacity - root.size()
    //Goal: Find smallest folder size s.t. dir.size()+free_space >= spaceneeded
    //a.k.a. minfoldersize>=root.size()-40000000
    let mut min:usize=dir.borrow().size(); //May not be valid if size is too small
    if min<minfoldersize {
        //If this folder is too small, then all subfolders will be too, so no need to check
        return None;
    }
    for (_,subdir) in dir.borrow().subdirs.iter() {
        let minsubsize=part2(subdir,minfoldersize);
        if let Some(minsize)=minsubsize {
            if minsize<min {
                min=minsize;
            }
        }
    }

    return Some(min);
}