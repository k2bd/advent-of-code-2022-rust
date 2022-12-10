use std::collections::{HashMap, HashSet};

// References used: https://aloso.github.io/2021/03/09/creating-an-iterator

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

    fn is_dir(&self) -> bool {
        matches!(self.children, Some(_))
    }

    /// Get a mutable reference to a nested element
    fn _get_mut_at(&mut self, loc: &[String]) -> Option<&mut Filesystem> {
        let mut current_location = self;

        let mut loc_internal = loc.to_owned();
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
    fn insert_at(&mut self, loc: &[String], key: String, value: Filesystem) {
        self._get_mut_at(loc)
            .expect("No entry at location")
            .children
            .as_mut()
            .expect("Entry is not a directory")
            .insert(key, value);
    }

    /// Get an immutable reference to a nested element
    fn get_at(&self, loc: &[String]) -> Option<&Filesystem> {
        let mut current_location = self;

        let mut loc_internal = loc.to_owned();
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

    fn _walk(&self, prefix_location: Vec<String>) -> HashSet<Vec<String>> {
        let mut result: HashSet<Vec<String>> = HashSet::new();

        if let Some(children) = &self.children {
            children.iter().for_each(|(name, fs)| {
                let mut nested_loc = prefix_location.clone();
                nested_loc.push(name.clone());
                result.insert(nested_loc.clone());

                result.extend(fs._walk(nested_loc.clone()));
            })
        }

        result
    }

    /// Get all recursive locations
    fn walk(&self) -> HashSet<Vec<String>> {
        let mut result: HashSet<Vec<String>> = HashSet::from([vec![]]);
        result.extend(self._walk(vec![]));

        result
    }
}

impl From<&str> for Filesystem {
    fn from(input: &str) -> Self {
        let mut fs = Filesystem::new_dir(HashMap::new());
        let mut working_dir: Vec<String> = vec![];

        input.lines().for_each(|line| {
            if let Some((_, command)) = line.split_once("$ ") {
                if let Some((_, go_to)) = command.split_once("cd ") {
                    match go_to {
                        "/" => {
                            working_dir = vec![];
                        }
                        ".." => {
                            working_dir.pop();
                        }
                        subdir => {
                            working_dir.push(subdir.to_string());
                        }
                    }
                }
            } else {
                // Listing some directory contents
                let (info, name) = line.split_once(' ').expect("Invalid ls line");
                match info {
                    "dir" => {
                        fs.insert_at(
                            &working_dir,
                            name.to_string(),
                            Filesystem::new_dir(HashMap::new()),
                        );
                    }
                    size => {
                        fs.insert_at(
                            &working_dir,
                            name.to_string(),
                            Filesystem::new_file(size.parse().expect("Invalid filesize")),
                        );
                    }
                }
            }
        });

        fs
    }
}

impl From<String> for Filesystem {
    fn from(input: String) -> Self {
        Self::from(&input[..])
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let filesystem = Filesystem::from(input);
    Some(
        filesystem
            .walk()
            .iter()
            .map(|loc| filesystem.get_at(loc).expect("Invalid location"))
            .filter(|fs| fs.is_dir())
            .map(|fs| fs.total_size())
            .filter(|&size| size <= 100000)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let filesystem = Filesystem::from(input);

    let space_needed = filesystem.total_size() - 40000000;
    println!("Space Needed: {}", space_needed);

    let mut candidates = filesystem
        .walk()
        .iter()
        .map(|loc| filesystem.get_at(loc).expect("Invalid location"))
        .filter(|fs| fs.is_dir())
        .map(|fs| fs.total_size())
        .filter(|&size| size >= space_needed)
        .collect::<Vec<_>>();

    candidates.sort();
    candidates.first().copied()
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
            &vec!["a".to_string(), "e".to_string()],
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
            example_filesystem().get_at(&loc).unwrap().total_size(),
            expected_size
        )
    }

    #[test]
    fn test_parse_fs() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(Filesystem::from(input), example_filesystem());
    }

    #[test]
    fn test_fs_walk() {
        assert_eq!(
            example_filesystem().walk(),
            HashSet::from([
                vec![],
                vec!["c.dat".to_string()],
                vec!["b.txt".to_string()],
                vec!["a".to_string()],
                vec!["a".to_string(), "h.lst".to_string()],
                vec!["a".to_string(), "e".to_string()],
                vec!["a".to_string(), "e".to_string(), "i".to_string()],
                vec!["a".to_string(), "f".to_string()],
                vec!["a".to_string(), "g".to_string()],
                vec!["d".to_string()],
                vec!["d".to_string(), "d.ext".to_string()],
                vec!["d".to_string(), "j".to_string()],
                vec!["d".to_string(), "k".to_string()],
                vec!["d".to_string(), "d.log".to_string()]
            ])
        );
    }
    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
