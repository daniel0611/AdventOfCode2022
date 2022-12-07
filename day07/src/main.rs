use std::collections::HashMap;

use aoc_utils::PuzzleInput;
const DAY: u8 = 7;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

struct Directory {
    sub_directories: HashMap<String, Directory>,
    files: HashMap<String, u32>,
}

impl Directory {
    fn new() -> Directory {
        Directory {
            sub_directories: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn add_file(&mut self, name: &str, size: u32) {
        self.files.insert(name.to_string(), size);
    }

    fn add_sub_directory(&mut self, name: &str, directory: Directory) {
        self.sub_directories.insert(name.to_string(), directory);
    }

    fn get_size(&self) -> u32 {
        let mut size = 0;
        for (_, file_size) in &self.files {
            size += file_size;
        }

        for (_, dir) in &self.sub_directories {
            size += dir.get_size();
        }

        size
    }
}

fn parse_input(input: &PuzzleInput) -> Directory {
    let steps: Vec<_> = input
        .raw_input
        .split('$')
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();
    let mut root_dir = Directory::new();
    let mut cwd = vec![];

    for step in steps {
        let cmd = step.lines().next().unwrap();
        let output = step
            .lines()
            .skip(1)
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n");

        let mut cmd_parts = cmd.split_whitespace();
        match cmd_parts.next().unwrap() {
            "cd" => {
                let dir_name = cmd_parts.next().unwrap();
                if dir_name == ".." {
                    cwd.pop();
                } else {
                    let dir = Directory::new();

                    let mut parent = &mut root_dir;
                    for dir_name in &cwd {
                        parent = parent.sub_directories.get_mut(*dir_name).unwrap();
                    }
                    parent.add_sub_directory(dir_name, dir);

                    cwd.push(dir_name);
                }
            },
            "ls" => {
                let mut parent = &mut root_dir;
                for dir_name in &cwd {
                    parent = parent.sub_directories.get_mut(*dir_name).unwrap();
                }

                for l in output.lines() {
                    let size = l.split_whitespace().next().unwrap().parse::<u32>();
                    if size.is_err() {
                        continue;
                    }

                    let name = l.split_whitespace().skip(1).collect::<Vec<_>>().join(" ");
                    parent.add_file(&name, size.unwrap());
                }

            },
            _ => panic!("Unknown command: {}", cmd),
        }
    }

    root_dir
}

fn get_directory_sizes(dir: &Directory) -> Vec<u32> {
    let mut sizes = vec![];
    sizes.push(dir.get_size());

    for (_, sub_dir) in &dir.sub_directories {
        sizes.extend(get_directory_sizes(sub_dir));
    }

    sizes

}

fn solve_a(input: &PuzzleInput) -> u32 {
    let root = parse_input(input);
    
    let dirs = get_directory_sizes(&root);
    
    dirs.iter()
    .filter(|dir| **dir <= 100000)
    .map(|dir| dir)
    .sum()
}

fn solve_b(input: &PuzzleInput) -> u32 {
    let root = parse_input(input);
    let mut dirs = get_directory_sizes(&root);
    dirs.sort();
    let current_size = *dirs.iter().max().unwrap();
    let needed_size = 30000000;
    let size_total = 70000000;
    
    for dir_size in dirs {
        if current_size - dir_size + needed_size <= size_total {
            return dir_size;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 95437);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 24933642);
    }
}
