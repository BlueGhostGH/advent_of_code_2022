use std::collections::HashMap;

pub const INPUT: &str = include_str!("./input.txt");
pub const DAY: usize = 7;

mod parse;
pub use parse::parse;

const MAX_USAGE: u64 = 70_000_000 - 30_000_000;

type Size = u64;
type DirectoryHandle = usize;
type Segment<'a> = &'a str;
type Segments<'a> = Vec<&'a str>;
type SegmentsSlice<'slf, 'a> = &'slf [&'a str];

#[derive(Debug)]
struct Directory {
    parent: Option<DirectoryHandle>,
    size: Size,
}

#[derive(Debug)]
pub struct FileSystem<'a> {
    directories: Vec<Directory>,
    path_to_handle: HashMap<Segments<'a>, DirectoryHandle>,
    current_handle: DirectoryHandle,
}

pub fn part1(file_system: &FileSystem<'_>) -> Size {
    file_system
        .directories
        .iter()
        .map(|directory| directory.size)
        .filter(|&size| size <= 100_000)
        .sum()
}

pub fn part2(file_system: &FileSystem<'_>) -> Option<Size> {
    let total_used = file_system.directories[0].size;
    let to_remove = total_used - MAX_USAGE;

    file_system
        .directories
        .iter()
        .map(|directory| directory.size)
        .filter(|&size| size >= to_remove)
        .min()
}

impl<'a> FileSystem<'a> {
    fn directory_handle<'slf>(&'slf mut self, path: SegmentsSlice<'slf, 'a>) -> DirectoryHandle {
        if let Some(&handle) = self.path_to_handle.get(path) {
            handle
        } else {
            let result = self.current_handle;

            self.directories.push(Directory {
                parent: None,
                size: 0,
            });
            self.path_to_handle.insert(Vec::from(path), result);
            self.current_handle += 1;

            result
        }
    }

    fn directory_mut(&mut self, handle: DirectoryHandle) -> &mut Directory {
        &mut self.directories[handle]
    }

    fn add_size(&mut self, mut handle: Option<DirectoryHandle>, size: Size) {
        while let Some(current_handle) = handle {
            let directory = self.directory_mut(current_handle);

            directory.size += size;

            handle = directory.parent;
        }
    }
}

#[derive(Debug)]
struct Path<'a> {
    segments: Segments<'a>,
}

impl<'a> Path<'a> {
    fn as_slice<'slf>(&'slf self) -> SegmentsSlice<'slf, 'a> {
        &self.segments
    }

    fn up(&mut self) {
        self.segments.pop();
    }

    fn down(&mut self, segment: Segment<'a>) {
        self.segments.push(segment);
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "$ cd /
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
    fn part1() {
        assert_eq!(crate::part1(&crate::parse(INPUT).unwrap()), 95437);
    }

    #[test]
    fn part2() {
        assert_eq!(crate::part2(&crate::parse(INPUT).unwrap()), Some(24933642));
    }
}
