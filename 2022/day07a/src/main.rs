use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Directory {
    files: Vec<u128>,
    subdirectories: HashSet<String>,
    size: u128
}

impl Directory {
    pub fn new() -> Directory {
        Directory {
            files: vec![],
            subdirectories: HashSet::new(),
            size: 0
        }
    }

    pub fn add_file(&mut self, name: &str, size: u128) {
        self.files.push(
            size
        );
    }

    pub fn add_dir(&mut self, name: &str) {
        if !self.subdirectories.contains(name) {
            self.subdirectories.insert(
                name.to_string(),
            );
        }
    }
}

#[derive(Debug)]
struct FileSystem {
    path: Vec<String>,
    directories: HashMap::<String, Directory>,
}

impl FileSystem {
    pub fn new() -> FileSystem {
        let mut fs = FileSystem {
            path: vec!["root".to_string()],
            directories: HashMap::new()
        };
        fs.directories.insert("root".to_string(), Directory::new());
        fs
    }

    pub fn add_dir(&mut self, name: &str) {
        let mut dirname = self.path.last().unwrap().clone();
        dirname.push_str("/");
        dirname.push_str(name);
        if !self.directories.contains_key(&dirname) {
            self.directories.insert(
                dirname.clone(),
                Directory::new()
            );
            self.directories
                .get_mut(self.path.last().unwrap().as_str())
                .unwrap()
                .add_dir(name);
        }
    }

    pub fn cd(&mut self, name: &str) {
        let mut dirname = self.path.last().unwrap().clone();
        dirname.push_str("/");
        dirname.push_str(name);
        if self.directories.contains_key(&dirname) {
            self.path.push(
                dirname
            );
        }
    }

    pub fn pop(&mut self) {
        self.path.pop();
    }

    pub fn root(&mut self) {
        while self.path.last().unwrap() != "/" {
            self.path.pop();
        }
    }

    pub fn add_file(&mut self, name: &str, size: u128) {
        self.directories
            .get_mut(self.path.last().unwrap().as_str())
            .unwrap()
            .add_file(name, size);

        for dir in &self.path {
            let d = self.directories
                .get_mut(dir.as_str()).unwrap();
            d.size += size;
        }
        
    }
}


fn main() {
    let input: Vec<&str> = include_str!("../input")
        .lines()
        .skip(1)
        .collect();

    let lines: Vec<Vec<&str>> = input
        .iter()
        .map(|l| l.split_whitespace().collect())
        .collect();

    let mut fs = FileSystem::new();

    for line in lines {
        match line[0] {
            "$" => {
                match line[1] {
                    "cd" => {
                        match line[2] {
                            "/" => {
                                fs.root();
                            },
                            ".." => {
                                fs.pop();
                            },
                            name => {
                                fs.cd(name);
                            }
                        }
                    },
                    "ls" => {},
                    _ => unreachable!()
                }
            }
            "dir" => {
                fs.add_dir(line[1]);
            }
            size => {
                if let Ok(size) = size.parse::<u128>() {
                    fs.add_file(line[1], size);
                }
                else {
                    dbg!(size);
                }
            }
        }
    }


    println!("The sum of directory sizes <= 10000 is {}", 
        fs.directories
            .values()
            .map(|d| d.size)
            .filter(|s| *s <= 100000)
            .sum::<u128>()
    );
}


