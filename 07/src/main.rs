use std::{collections::HashMap, env, fs};

use anyhow::Context;

//. #[derive(Debug, PartialEq)]
//. struct File {
//.     name: String,
//.     size: u64,
//. }

enum Command<'a> {
    Cd(&'a str),
    Ls,
}

fn parse_command<'a>(line: &'a str) -> Option<Command<'a>> {
    const CMD_CD: &str = "$ cd ";
    const CMD_LS: &str = "$ ls";
    if line.starts_with(&CMD_CD) {
        return Some(Command::Cd(&line[CMD_CD.len()..]));
    }
    if line.starts_with(&CMD_LS) {
        return Some(Command::Ls);
    }
    None
}

enum LsOutputLine<'a> {
    Dir(&'a str),
    File(u64, &'a str),
}

fn parse_ls_output_line(s: &str) -> Result<LsOutputLine, anyhow::Error> {
    let (left, name) = s
        .split_once(' ')
        .context(format!("failed to split ls output line '{}'", s))?;
    if left == "dir" {
        return Ok(LsOutputLine::Dir(name));
    }
    let size: u64 = left.parse().context(format!(
        "failed to parse size of file in ls output line '{}'",
        s
    ))?;
    Ok(LsOutputLine::File(size, name))
}

#[derive(Clone, Debug, PartialEq)]
struct Directory {
    path: String,
    size: u64,
}

fn ls(current_dir: &str, arg: &str) -> String {
    if arg.starts_with("/") {
        arg.to_owned() // assume normalized
    } else if arg == ".." {
        let (left, _) = current_dir
            .rsplit_once('/')
            .expect(&format!("Cannot 'cd ..' in '{current_dir}'"));
        if left.is_empty() {
            "/".to_owned()
        } else {
            left.to_owned()
        }
    } else {
        if current_dir.ends_with('/') {
            format!("{current_dir}{arg}")
        } else {
            format!("{current_dir}/{arg}")
        }
    }
}

fn read_term_output(oup: &str) -> Vec<Directory> {
    let mut result = oup
        .lines()
        .fold(
            (HashMap::new(), "".to_string(), false),
            |(mut dirs, mut cwd, visiting_new_dir), ln| {
                if let Some(cmd) = parse_command(ln) {
                    match cmd {
                        Command::Cd(arg) => {
                            cwd = ls(&cwd, arg);
                            if dirs.contains_key(&cwd) {
                                return (dirs, cwd, false);
                            } else {
                                dirs.insert(
                                    cwd.clone(),
                                    Directory {
                                        path: cwd.clone(),
                                        size: 0,
                                    },
                                );
                                return (dirs, cwd, true);
                            }
                        }
                        Command::Ls => {}
                    }
                    return (dirs, cwd, visiting_new_dir);
                }
                if !visiting_new_dir {
                    return (dirs, cwd, false);
                }
                match parse_ls_output_line(ln) {
                    Ok(LsOutputLine::File(size, _name)) => {
                        // meh...
                        let mut tmp = cwd.clone();
                        loop {
                            if let Some(dir) = dirs.get_mut(&tmp) {
                                dir.size += size;
                                if tmp == "/" {
                                    break;
                                }
                                tmp = ls(&tmp, "..");
                            } else {
                                break;
                            }
                        }
                    }
                    Ok(LsOutputLine::Dir(_name)) => {}
                    Err(err) => panic!("Error parsing ls out-ln: {}", err),
                }
                (dirs, cwd, true)
            },
        )
        .0
        .values()
        .cloned()
        .collect::<Vec<_>>();
    result.sort_by(|a, b| b.size.partial_cmp(&a.size).unwrap());
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_term_output_empty_root() {
        assert_eq!(
            read_term_output(
                "$ cd /\n\
                 $ ls"
            ),
            vec![Directory {
                path: "/".to_string(),
                size: 0
            }]
        );
    }

    #[test]
    fn read_term_output_some_files_in_root() {
        assert_eq!(
            read_term_output(
                "$ cd /\n\
                 $ ls\n\
                 11 hello\n\
                 100 a"
            ),
            vec![Directory {
                path: "/".to_string(),
                size: 111
            }]
        );
    }

    #[test]
    fn read_term_output_some_files_and_empty_dir_in_root() {
        assert_eq!(
            read_term_output(
                "$ cd /\n\
                 $ ls\n\
                 11 hello\n\
                 100 a\n\
                 dir yy\n\
                 $ cd yy\n\
                 $ ls"
            ),
            vec![
                Directory {
                    path: "/".to_string(),
                    size: 111
                },
                Directory {
                    path: "/yy".to_string(),
                    size: 0
                },
            ]
        );
    }

    #[test]
    fn read_term_output_cd_dotdot() {
        assert_eq!(
            read_term_output(
                "$ cd /\n\
                 $ ls\n\
                 11 hello\n\
                 100 a\n\
                 dir yy\n\
                 $ cd yy\n\
                 $ ls\n\
                 $ cd .."
            ),
            vec![
                Directory {
                    path: "/".to_string(),
                    size: 111
                },
                Directory {
                    path: "/yy".to_string(),
                    size: 0
                },
            ]
        );
    }

    #[test]
    fn read_term_output_subdirs() {
        assert_eq!(
            read_term_output(
                "$ cd /\n\
                 $ ls\n\
                 11 hello\n\
                 dir yy\n\
                 $ cd yy\n\
                 $ ls\n\
                 22 world\n\
                 $ cd .."
            ),
            vec![
                Directory {
                    path: "/".to_string(),
                    size: 33
                },
                Directory {
                    path: "/yy".to_string(),
                    size: 22
                },
            ]
        );
    }

    #[test]
    fn read_term_output_visit_subdir_twice() {
        assert_eq!(
            read_term_output(
                "$ cd /\n\
                 $ ls\n\
                 11 hello\n\
                 dir yy\n\
                 $ cd yy\n\
                 $ ls\n\
                 22 world\n\
                 $ cd ..\n\
                 $ cd yy\n\
                 $ ls\n\
                 22 world\n\
                 "
            ),
            vec![
                Directory {
                    path: "/".to_string(),
                    size: 33
                },
                Directory {
                    path: "/yy".to_string(),
                    size: 22
                },
            ]
        );
    }

    #[test]
    fn read_term_output_r1() {
        assert_eq!(
            read_term_output(
                "$ cd /\n\
                 $ ls\n\
                 11 hello\n\
                 dir l1\n\
                 $ cd l1\n\
                 $ ls\n\
                 dir l2\n\
                 22 world\n\
                 $ cd l2\n\
                 $ ls\n\
                 dir l3\n\
                 22 world\n\
                 $ cd l3\n\
                 $ ls\n\
                 33 b\n\
                 "
            ),
            vec![
                Directory {
                    path: "/".to_string(),
                    size: 88
                },
                Directory {
                    path: "/l1".to_string(),
                    size: 77
                },
                Directory {
                    path: "/l1/l2".to_string(),
                    size: 55
                },
                Directory {
                    path: "/l1/l2/l3".to_string(),
                    size: 33
                },
            ]
        );
    }

    #[test]
    fn read_term_output_r2() {
        assert_eq!(
            read_term_output(
                "$ cd /\n\
                 $ ls\n\
                 11 hello\n\
                 dir l1\n\
                 $ cd l1\n\
                 $ ls\n\
                 dir l2a\n\
                 dir l2b\n\
                 22 world\n\
                 $ cd l2a\n\
                 $ ls\n\
                 22 world\n\
                 $ cd ..\n\
                 $ cd l2b\n\
                 $ ls\n\
                 33 b\n\
                 "
            ),
            vec![
                Directory {
                    path: "/".to_string(),
                    size: 88
                },
                Directory {
                    path: "/l1".to_string(),
                    size: 77
                },
                Directory {
                    path: "/l1/l2b".to_string(),
                    size: 33
                },
                Directory {
                    path: "/l1/l2a".to_string(),
                    size: 22
                },
            ]
        );
    }
}

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("07/test_data.txt".into());
    let input = fs::read_to_string(&input_file_path)
        .expect(&format!("Error reading input file {input_file_path}"));
    let dirs = read_term_output(&input);
    const PART1_SMALL_DIR_LIMIT: u64 = 100_000;
    let sum_of_small_dir_sizes: u64 = dirs
        .iter()
        .map(|d| d.size)
        .filter(|&s| s <= PART1_SMALL_DIR_LIMIT)
        .sum();
    println!(
        "Sum of all direcctories of size at most {}: {}",
        PART1_SMALL_DIR_LIMIT, sum_of_small_dir_sizes
    );
    const TOTAL_DISK_SPACE: u64 = 70000000;
    const REQUIRED_DISK_SPACE: u64 = 30000000;
    let amount_to_delete = REQUIRED_DISK_SPACE + dirs[0].size - TOTAL_DISK_SPACE;
    let dir_to_delete = dirs.iter().rfind(|d| d.size >= amount_to_delete).unwrap();
    println!("Need to free up {amount_to_delete}, should delete {} with size {}",
             dir_to_delete.path, dir_to_delete.size
    )
}
