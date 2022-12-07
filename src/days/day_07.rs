use std::collections::{HashMap, HashSet};

const TOTAL_DISK_SIZE: u32 = 70000000;
const DISK_SPACE_NEEDED: u32 = 30000000;

pub fn main(input: &str) -> (u32, u32) {
    let mut current_path: Vec<&str> = vec!["/"];
    let mut filesystem = Filesystem {
        entries: HashMap::new(),
    };
    filesystem.entries.insert(
        "/".to_string(),
        FilesystemEntry::Directory(Directory {
            path: "/".to_string(),
            children: HashSet::new(),
        }),
    );

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let mut words = line.split_whitespace();

        match words.next().unwrap() {
            "$" => match words.next().unwrap() {
                "cd" => match words.next().unwrap() {
                    "/" => {
                        current_path.clear();
                        current_path.push("/");
                    }
                    ".." => {
                        current_path.pop().unwrap();
                    }
                    path => {
                        for piece in path.split('/') {
                            current_path.push(piece);
                        }
                    }
                },
                "ls" => {}
                cmd => {
                    panic!("unrecognized command {cmd}");
                }
            },
            "dir" => {
                let path = words.next().unwrap().to_string();
                let new_path = format!("{}/{}", current_path.join("/"), path).replace("//", "/");
                match filesystem
                    .entries
                    .get_mut(&current_path.join("/").replace("//", "/"))
                    .unwrap()
                {
                    FilesystemEntry::Directory(d) => {
                        d.children.insert(path.clone());
                    }
                    FilesystemEntry::File(_) => panic!("inside file?"),
                }
                filesystem.entries.insert(
                    new_path,
                    FilesystemEntry::Directory(Directory {
                        path,
                        children: HashSet::new(),
                    }),
                );
            }
            num => {
                let size: u32 = num.parse().expect("invalid number");
                let path = words.next().unwrap().to_string();
                let new_path = format!("{}/{}", current_path.join("/"), path).replace("//", "/");
                match filesystem
                    .entries
                    .get_mut(&current_path.join("/").replace("//", "/"))
                    .unwrap()
                {
                    FilesystemEntry::Directory(d) => {
                        d.children.insert(path.clone());
                    }
                    FilesystemEntry::File(_) => panic!("inside file?"),
                }

                filesystem
                    .entries
                    .insert(new_path, FilesystemEntry::File(File { path, size }));
            }
        }
    }

    let mut part1 = 0;

    let total_used = filesystem.get_recursive_size("/");

    let need_to_delete = total_used - (TOTAL_DISK_SIZE - DISK_SPACE_NEEDED);
    let mut size_to_delete = u32::MAX;

    for (path, entry) in filesystem.entries.iter() {
        if let FilesystemEntry::Directory(_) = entry {
            let entry_size = filesystem.get_recursive_size(path);

            if entry_size > need_to_delete && entry_size < size_to_delete {
                size_to_delete = entry_size;
            }

            if entry_size < 100000 {
                part1 += entry_size;
            }
        }
    }

    (part1, size_to_delete)
}

struct Filesystem {
    entries: HashMap<String, FilesystemEntry>,
}

impl Filesystem {
    fn get_recursive_size(&self, path: &str) -> u32 {
        match self.entries.get(path).unwrap() {
            FilesystemEntry::Directory(d) => d.children.iter().fold(0, |acc, child| {
                acc + self.get_recursive_size(&format!("{}/{}", path, child).replace("//", "/"))
            }),
            FilesystemEntry::File(f) => f.size,
        }
    }
}

#[derive(Debug)]
enum FilesystemEntry {
    Directory(Directory),
    File(File),
}

impl FilesystemEntry {
    fn path(&self) -> &str {
        match self {
            FilesystemEntry::Directory(d) => &d.path,
            FilesystemEntry::File(f) => &f.path,
        }
    }
}

#[derive(Debug)]
struct Directory {
    path: String,
    children: HashSet<String>,
}

#[derive(Debug)]
struct File {
    path: String,
    size: u32,
}
