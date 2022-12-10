use std::collections::HashMap;

#[derive(PartialEq, Debug)]
struct Filesystem {
    /// Size of this element itself, not including any children
    size: u32,

    /// Children of the element, i.e. contents of a directory
    children: Option<HashMap<String, Filesystem>>,
}

impl Filesystem {
    /// A file
    fn new_file(size: u32) -> Filesystem {
        Filesystem {
            size,
            children: None,
        }
    }

    /// A directory
    fn new_dir(children: HashMap<String, Filesystem>) -> Filesystem {
        Filesystem {
            size: 0,
            children: Some(children),
        }
    }

    /// Get a mutable reference to a nested element
    fn _get_mut_at(&mut self, loc: Vec<String>) -> Option<&mut Filesystem> {
        let mut current_location = self;

        let mut loc_internal = loc.clone();
        loc_internal.reverse();

        loop {
            if let Some(item) = loc_internal.pop() {
                if let Some(children) = &mut current_location.children {
                    if let Some(child) = children.get_mut(&item.to_string()) {
                        current_location = child;
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            } else {
                return Some(current_location);
            }
        }
    }

    /// Insert a new entry in a directory
    fn insert_at(&mut self, loc: Vec<String>, key: String, value: Filesystem) {
        self._get_mut_at(loc)
            .expect("No entry at location")
            .children
            .as_mut()
            .expect("Entry is not a directory")
            .insert(key, value);
    }

    /// Get an immutable reference to a nested element
    fn get_at(&self, loc: Vec<String>) -> Option<&Filesystem> {
        let mut current_location = self;

        let mut loc_internal = loc.clone();
        loc_internal.reverse();

        loop {
            if let Some(item) = loc_internal.pop() {
                if let Some(children) = &current_location.children {
                    if let Some(child) = children.get(&item.to_string()) {
                        current_location = child;
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            } else {
                return Some(current_location);
            }
        }
    }

    // Get total size of an item, counting contents recursively
    fn total_size(&self) -> u32 {
        self.size
            + self
                .children
                .as_ref()
                .unwrap_or(&HashMap::new())
                .iter()
                .map(|(_, entry)| entry.total_size())
                .sum::<u32>()
    }
}

impl From<&str> for Filesystem {
    fn from(_: &str) -> Self {
        todo!()
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
    use rstest::rstest;

    fn example_filesystem() -> Filesystem {
        Filesystem::new_dir(HashMap::from([
            (
                "a".to_string(),
                Filesystem::new_dir(HashMap::from([
                    (
                        "e".to_string(),
                        Filesystem::new_dir(HashMap::from([(
                            "i".to_string(),
                            Filesystem::new_file(584),
                        )])),
                    ),
                    ("f".to_string(), Filesystem::new_file(29116)),
                    ("g".to_string(), Filesystem::new_file(2557)),
                    ("h.lst".to_string(), Filesystem::new_file(62596)),
                ])),
            ),
            ("b.txt".to_string(), Filesystem::new_file(14848514)),
            ("c.dat".to_string(), Filesystem::new_file(8504156)),
            (
                "d".to_string(),
                Filesystem::new_dir(HashMap::from([
                    ("j".to_string(), Filesystem::new_file(4060174)),
                    ("d.log".to_string(), Filesystem::new_file(8033020)),
                    ("d.ext".to_string(), Filesystem::new_file(5626152)),
                    ("k".to_string(), Filesystem::new_file(7214296)),
                ])),
            ),
        ]))
    }

    #[test]
    fn test_insert_at() {
        let mut fs = example_filesystem();
        fs.insert_at(
            vec!["a".to_string(), "e".to_string()],
            "kevin.txt".to_string(),
            Filesystem::new_file(1234),
        );

        assert_eq!(
            fs,
            Filesystem::new_dir(HashMap::from([
                (
                    "a".to_string(),
                    Filesystem::new_dir(HashMap::from([
                        (
                            "e".to_string(),
                            Filesystem::new_dir(HashMap::from([
                                ("i".to_string(), Filesystem::new_file(584),),
                                ("kevin.txt".to_string(), Filesystem::new_file(1234),),
                            ])),
                        ),
                        ("f".to_string(), Filesystem::new_file(29116)),
                        ("g".to_string(), Filesystem::new_file(2557)),
                        ("h.lst".to_string(), Filesystem::new_file(62596)),
                    ])),
                ),
                ("b.txt".to_string(), Filesystem::new_file(14848514)),
                ("c.dat".to_string(), Filesystem::new_file(8504156)),
                (
                    "d".to_string(),
                    Filesystem::new_dir(HashMap::from([
                        ("j".to_string(), Filesystem::new_file(4060174)),
                        ("d.log".to_string(), Filesystem::new_file(8033020)),
                        ("d.ext".to_string(), Filesystem::new_file(5626152)),
                        ("k".to_string(), Filesystem::new_file(7214296)),
                    ])),
                ),
            ]))
        )
    }

    #[rstest]
    #[case(vec!["a".to_string(), "e".to_string()], 584)]
    #[case(vec!["a".to_string()], 94853)]
    #[case(vec!["d".to_string()], 24933642)]
    #[case(vec![], 48381165)]
    fn test_total_size(#[case] loc: Vec<String>, #[case] expected_size: u32) {
        assert_eq!(
            example_filesystem().get_at(loc).unwrap().total_size(),
            expected_size
        )
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
