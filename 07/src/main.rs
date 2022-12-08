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
    let mut result = oup.lines()
        .fold(
            (HashMap::new(), "".to_string()),
            |(mut dirs, mut cwd), ln| {
                if let Some(cmd) = parse_command(ln) {
                    match cmd {
                        Command::Cd(arg) => {
                            cwd = ls(&cwd, arg);
                            if !dirs.contains_key(&cwd) {
                                dirs.insert(
                                    cwd.clone(),
                                    Directory {
                                        path: cwd.clone(),
                                        size: 0,
                                    },
                                );
                            }
                        }
                        Command::Ls => {}
                    }
                    return (dirs, cwd);
                }
                match parse_ls_output_line(ln) {
                    Ok(LsOutputLine::File(size, _name)) => {
                        // meh...
                        let mut tmp = cwd.clone();
                        loop {
                            if let Some(dir) = dirs.get_mut(&tmp) {
                                dbg!(&dir.path);
                                dir.size += size;
                                if tmp == "/" {
                                    break
                                }
                                tmp = ls(&tmp, "..");
                            } else {
                                break
                            }
                        }
                    }
                    Ok(LsOutputLine::Dir(_name)) => {}
                    Err(err) => panic!("Error parsing ls out-ln: {}", err),
                }
                (dirs, cwd)
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
}

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("07/test_data.txt".into());
    let input = fs::read_to_string(&input_file_path)
        .expect(&format!("Error reading input file {input_file_path}"));
    let dirs = read_term_output(&input);
    dbg!(dirs);
}
