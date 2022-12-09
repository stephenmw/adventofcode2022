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
        let blocks = many1(command_block);
        ws_all_consuming(blocks)(input)
    }

    fn cd(input: &str) -> IResult<&str, CommandBlock> {
        let prefix = tuple((tag("$"), space0, tag("cd"), space1));
        let block = preceded(prefix, location).map(|loc| CommandBlock::Cd { loc });
        ws_line(block)(input)
    }

    fn ls(input: &str) -> IResult<&str, CommandBlock> {
        let cmd_line = tuple((tag("$"), space0, tag("ls")));
        let dir_line = preceded(pair(tag("dir"), space1), filename)
            .map(|name| DirRecord::Dir(name.to_owned()));
        let file_line = separated_pair(uint, space1, filename)
            .map(|(size, name)| DirRecord::File(name.to_owned(), size));

        let record = alt((dir_line, file_line));
        let records = many1(ws_line(record));

        preceded(ws_line(cmd_line), records)
            .map(|records| CommandBlock::Ls { records })
            .parse(input)
    }

    fn location(input: &str) -> IResult<&str, Location> {
        alt((
            value(Location::Root, tag("/")),
            value(Location::Parent, tag("..")),
            filename.map(|f| Location::Directory(f.to_owned())),
        ))(input)
    }

    fn filename(input: &str) -> IResult<&str, &str> {
        let f = many1_count(alt((alphanumeric1, tag("."))));
        recognize(f)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        $ cd /
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
        7214296 k
    ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "95437")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "24933642")
    }
}
