use std::mem;

struct FileInfo {
    name: String,
    size: u32,
}

/// A node in a filesystem
enum FSNode {
    /// A file with a filename and size
    File(FileInfo),

    /// A directory with a filename and some inner contents
    Directory(String, Vec<FSNode>),
}

impl FSNode {
    fn get_child(&self, name: String) -> &mut FSNode {
        if let FSNode::Directory(_, mut children) = self {
            &children.iter_mut().find(|child| match child {
                FSNode::File(info) => info.name == name,
                FSNode::Directory(dir_name, _) => *dir_name == name,
            }).unwrap()
        } else {
            panic!("TODO")
        }
    }

    /// Iterate over contents recursively
    fn walk(&self) -> FSIter<'_> {
        FSIter {
            children: std::slice::from_ref(self),
            parent: None,
        }
    }

    /// Get the size of the node; either the size of a single file or the
    /// size of all contents of a directory
    fn size(&self) -> u32 {
        self.walk()
            .map(|node| match node {
                FSNode::File(info) => info.size,
                _ => 0,
            })
            .sum()
    }
}

impl From<&str> for FSNode {
    /// Create a filesystem from the puzzle input
    fn from(mut input: &str) -> Self {
        let mut filesystem = FSNode::Directory(String::from('/'), vec![]);
        let mut current_path: Vec<String> = vec![];

        input.lines().for_each(|line| {
            if let Some((_, cmd)) = line.split_once("$ ") {
                // Some user command
                if let Some((_, go_to)) = line.split_once("cd ") {
                    match go_to {
                        "/" => {
                            current_path = vec![];
                        }
                        ".." => {
                            current_path.pop();
                        }
                        other => {
                            current_path.push(String::from(other));
                        }
                    }
                }
            } else {
                // Some directory info
                let mut cwd = &mut filesystem;
                current_path
                    .iter()
                    .for_each(|subdir| cwd = cwd.get_child(subdir.to_string()));

                if let FSNode::Directory(_, ref mut children) = *cwd {
                    if let Some((_, dir_name)) = line.split_once("dir ") {
                        children.push(FSNode::Directory(String::from(dir_name), vec![]));
                    } else {
                        let (size, name) = line.split_once(" ").unwrap();
                        children.push(FSNode::File(FileInfo {
                            size: size.parse().unwrap(),
                            name: name.to_string(),
                        }))
                    }
                } else {
                    panic!("CWD is a file!")
                }
            }
        });

        filesystem
    }
}

/// An iterator over the contents of a filesystem
#[derive(Default)]
struct FSIter<'a> {
    children: &'a [FSNode],
    parent: Option<Box<FSIter<'a>>>,
}

// See https://aloso.github.io/2021/03/09/creating-an-iterator#the-collection-type
/// Iterate over contents of a filesystem
impl<'a> Iterator for FSIter<'a> {
    type Item = &'a FSNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.children.get(0) {
            self.children = &self.children[1..];
            match node {
                FSNode::File(_) => Some(node),
                FSNode::Directory(_, children) => {
                    *self = FSIter {
                        children: children.as_slice(),
                        parent: Some(Box::new(mem::take(self))),
                    };
                    Some(node)
                }
            }
        } else {
            if let Some(parent) = self.parent.take() {
                *self = *parent;
                self.next()
            } else {
                None
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_filesystem() -> FSNode {
        FSNode::Directory(
            String::from('/'),
            vec![
                FSNode::Directory(
                    String::from('a'),
                    vec![
                        FSNode::Directory(
                            String::from('e'),
                            vec![FSNode::File(FileInfo {
                                name: String::from('i'),
                                size: 584,
                            })],
                        ),
                        FSNode::File(FileInfo {
                            name: String::from('f'),
                            size: 29116,
                        }),
                        FSNode::File(FileInfo {
                            name: String::from('g'),
                            size: 2557,
                        }),
                        FSNode::File(FileInfo {
                            name: String::from("h.lst"),
                            size: 62596,
                        }),
                    ],
                ),
                FSNode::File(FileInfo {
                    name: String::from("b.txt"),
                    size: 14848514,
                }),
                FSNode::File(FileInfo {
                    name: String::from("c.dat"),
                    size: 8504156,
                }),
                FSNode::Directory(
                    String::from('d'),
                    vec![
                        FSNode::File(FileInfo {
                            name: String::from('j'),
                            size: 4060174,
                        }),
                        FSNode::File(FileInfo {
                            name: String::from("d.log"),
                            size: 8033020,
                        }),
                        FSNode::File(FileInfo {
                            name: String::from("d.ext"),
                            size: 5626152,
                        }),
                        FSNode::File(FileInfo {
                            name: String::from('k'),
                            size: 7214296,
                        }),
                    ],
                ),
            ],
        )
    }

    #[test]
    fn test_fs_size_root() {
        let fs = example_filesystem();
        assert_eq!(fs.size(), 48381165);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
