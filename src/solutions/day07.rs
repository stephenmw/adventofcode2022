use crate::solutions::prelude::*;

use std::collections::HashMap;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let data = parse!(input);
    let file_tree = tree(data);

    let ans: usize = directory_sizes(&file_tree)
        .into_iter()
        .filter(|&x| x < 100000)
        .sum();

    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let data = parse!(input);
    let file_tree = tree(data);
    let dir_sizes = directory_sizes(&file_tree);

    let root_size = *dir_sizes.iter().max().unwrap();
    let free = 70000000 - root_size;
    let needed = 30000000 - free;

    let ans = *dir_sizes.iter().filter(|&&x| x > needed).min().unwrap();

    Ok(ans.to_string())
}

type InodeNumber = usize;

#[derive(Clone, Debug, Default)]
struct DirTree {
    inodes: Vec<Inode>,
}

impl DirTree {
    fn add_node(&mut self) -> InodeNumber {
        let ret = self.inodes.len();
        self.inodes.push(Inode::default());
        ret
    }

    fn get(&self, inode: InodeNumber) -> Option<&Inode> {
        self.inodes.get(inode)
    }

    fn get_mut(&mut self, inode: InodeNumber) -> Option<&mut Inode> {
        self.inodes.get_mut(inode)
    }

    fn get_child_or_insert(&mut self, cur: InodeNumber, name: String) -> InodeNumber {
        if let Some(id) = self.inodes[cur].subdirs.get(&name) {
            return *id;
        }

        let new_id = self.add_node();
        self.inodes[cur].subdirs.insert(name, new_id);
        new_id
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Inode {
    subdirs: HashMap<String, InodeNumber>,
    records: Vec<DirRecord>,
}

impl Inode {
    fn non_rec_size(&self) -> usize {
        self.records
            .iter()
            .filter_map(|r| match r {
                DirRecord::Dir(_) => None,
                DirRecord::File(_, size) => Some(size),
            })
            .sum()
    }
}

fn tree(commands_blocks: Vec<CommandBlock>) -> DirTree {
    let mut tree = DirTree::default();
    let root = tree.add_node();
    let mut pwd = vec![root];

    for block in commands_blocks {
        match block {
            CommandBlock::Cd { loc } => match loc {
                Location::Root => {
                    pwd.truncate(1);
                }
                Location::Parent => {
                    if pwd.len() > 1 {
                        pwd.pop();
                    }
                }
                Location::Directory(d) => {
                    let cur = *pwd.last().unwrap();
                    let next = tree.get_child_or_insert(cur, d);
                    pwd.push(next);
                }
            },
            CommandBlock::Ls { records } => {
                let cur = *pwd.last().unwrap();
                tree.get_mut(cur).unwrap().records = records;
            }
        }
    }

    tree
}

fn directory_sizes(tree: &DirTree) -> Vec<usize> {
    fn rec(sizes: &mut Vec<usize>, tree: &DirTree, node_id: InodeNumber) -> usize {
        let node = tree.get(node_id).unwrap();

        let sum_children: usize = node.subdirs.values().map(|&x| rec(sizes, tree, x)).sum();
        let total_size = sum_children + node.non_rec_size();

        sizes.push(total_size);
        total_size
    }

    let mut sizes = Vec::new();
    rec(&mut sizes, tree, 0);
    sizes
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandBlock {
    Cd { loc: Location },
    Ls { records: Vec<DirRecord> },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DirRecord {
    Dir(String),
    File(String, usize),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Location {
    Root,
    Parent,
    Directory(String),
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<CommandBlock>> {
        let command_block = alt((cd, ls));
        let blocks = separated_list1(line_ending, command_block);
        complete(blocks)(input)
    }

    fn cd(input: &str) -> IResult<&str, CommandBlock> {
        map(delimited(tag("$ cd "), location, space0), |loc| {
            CommandBlock::Cd { loc }
        })(input)
    }

    fn ls(input: &str) -> IResult<&str, CommandBlock> {
        let cmd_line = tag("$ ls");
        let dir_line = map(preceded(tag("dir "), filename), |d| {
            DirRecord::Dir(d.to_owned())
        });
        let file_line = map(separated_pair(uint, space1, filename), |(s, f)| {
            DirRecord::File(f.to_owned(), s)
        });

        let record = alt((dir_line, file_line));
        let records = separated_list1(line_ending, record);

        let mut block = map(
            separated_pair(cmd_line, line_ending, records),
            |(_, records)| CommandBlock::Ls { records },
        );

        block(input)
    }

    fn location(input: &str) -> IResult<&str, Location> {
        let root = value(Location::Root, tag("/"));
        let parent = value(Location::Parent, tag(".."));
        let dir = map(filename, |f| Location::Directory(f.to_owned()));
        alt((root, parent, dir))(input)
    }

    fn filename(input: &str) -> IResult<&str, &str> {
        input.split_at_position1_complete(
            |item| !(item.is_alphanum() || item == '.'),
            nom::error::ErrorKind::AlphaNumeric,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "$ cd /
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
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "95437")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "24933642")
    }
}
