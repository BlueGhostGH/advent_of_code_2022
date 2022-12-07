use std::{collections::HashMap, mem};

#[derive(Debug)]
struct Shell<'a> {
    file_system: crate::FileSystem<'a>,
    process_working_directory: crate::Path<'a>,
    directory_handle: Option<crate::DirectoryHandle>,
}

impl<'a> Shell<'a> {
    fn run_session(&mut self, input: &str) -> Option<()> {
        enum State {
            Cmd,
            Dir { size: crate::Size },
        }

        let mut state = State::Cmd;

        for line in input.lines() {
            let line = line.as_bytes();

            match state {
                State::Cmd => match line.get(0)? {
                    b'$' => self.process_command(&line.get(2..)?)?,
                    _ => {
                        state = State::Dir {
                            size: self.process_node(line)?,
                        };
                    }
                },
                State::Dir { size } => match line.get(0)? {
                    b'$' => {
                        self.file_system.add_size(self.directory_handle, size);
                        self.process_command(line.get(2..)?)?;

                        state = State::Cmd
                    }
                    _ => {
                        state = State::Dir {
                            size: size + self.process_node(line)?,
                        }
                    }
                },
            }
        }

        if let State::Dir { size } = state {
            self.file_system.add_size(self.directory_handle, size);
        }

        Some(())
    }

    fn process_command(&mut self, line: &[u8]) -> Option<()> {
        if line.get(0..2)? == b"cd" {
            let argument = line.get(3..)?;

            match argument {
                b"\\" => {
                    self.process_working_directory = crate::Path {
                        segments: Vec::new(),
                    };
                }
                b".." => {
                    self.process_working_directory.up();

                    let handle = self
                        .file_system
                        .directory_handle(&self.process_working_directory.segments);

                    self.directory_handle = Some(handle);
                }
                argument => {
                    self.process_working_directory
                        .down(unsafe { mem::transmute::<_, &str>(argument) });

                    let child = self
                        .file_system
                        .directory_handle(self.process_working_directory.as_slice());
                    self.file_system.directory_mut(child).parent = self.directory_handle;

                    self.directory_handle = Some(child);
                }
            }
        }

        Some(())
    }

    fn process_node(&mut self, line: &[u8]) -> Option<crate::Size> {
        match unsafe { mem::transmute::<_, &str>(line) }
            .split_ascii_whitespace()
            .next()?
        {
            "dir" => Some(0),
            size => size.parse::<crate::Size>().ok(),
        }
    }
}

pub fn parse(input: &str) -> Option<crate::FileSystem> {
    let mut shell = Shell {
        file_system: crate::FileSystem {
            directories: Vec::new(),
            path_to_handle: HashMap::new(),
            current_handle: 0,
        },
        process_working_directory: crate::Path {
            segments: crate::Segments::new(),
        },
        directory_handle: None,
    };

    shell.run_session(input)?;

    Some(shell.file_system)
}
