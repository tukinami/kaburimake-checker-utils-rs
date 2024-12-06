use std::path::Path;

use chrono::Utc;

use crate::{
    ast::{GhostData, GhostJson},
    config::BuildArgs,
    io::{load_setting_file, write_json},
    procedure::unique_fold,
};

pub(super) fn build(args: &BuildArgs) -> Result<(), std::io::Error> {
    let mut ghost_list = Vec::new();

    for p in args.input().iter() {
        if let Some(list) = read_ghost_collection(p) {
            ghost_list.extend_from_slice(&list);
        }
    }

    println!("raw length: {}", ghost_list.len());

    let ghost_list = ghost_list.iter().fold(Vec::new(), unique_fold);

    println!("result length: {}", ghost_list.len());

    let json = GhostJson::new(Utc::now(), ghost_list);

    write_json(args.output(), &json)
}

pub(crate) fn read_ghost_collection<P>(dir_path: P) -> Option<Vec<GhostData>>
where
    P: AsRef<Path>,
{
    let dir_path = dir_path.as_ref();
    let entries = match dir_path.read_dir() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}: {}", dir_path.display(), e);
            return None;
        }
    };

    let mut ghost_list = Vec::new();
    for entry in entries {
        let entry = match entry {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        match read_ghost_data(entry.path()) {
            Ok(v) => ghost_list.push(v),
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        }
    }

    if ghost_list.is_empty() {
        None
    } else {
        Some(ghost_list)
    }
}

fn read_ghost_data<P>(dir_path: P) -> Result<GhostData, std::io::Error>
where
    P: AsRef<Path>,
{
    let directory = read_directory_name(&dir_path)?;
    let (sakura_name, kero_name) = read_names_from_descript(&dir_path)?;

    Ok(GhostData::new(directory, sakura_name, kero_name))
}

fn read_directory_name<P>(dir_path: P) -> Result<String, std::io::Error>
where
    P: AsRef<Path>,
{
    if !dir_path.as_ref().exists() {
        return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
    } else if !dir_path.as_ref().is_dir() {
        return Err(std::io::Error::from(std::io::ErrorKind::NotADirectory));
    }

    if let Some(dir_name) = read_directory_name_from_installtxt(&dir_path)? {
        return Ok(dir_name);
    }

    dir_path
        .as_ref()
        .file_name()
        .ok_or(std::io::Error::from(std::io::ErrorKind::NotFound))
        .map(|v| v.to_string_lossy().to_string())
}

fn read_directory_name_from_installtxt<P>(root: P) -> Result<Option<String>, std::io::Error>
where
    P: AsRef<Path>,
{
    let path = root.as_ref().join("install.txt");
    if !path.is_file() {
        return Ok(None);
    }

    let contents = load_setting_file(path)?;

    contents
        .lines()
        .find_map(|v| get_labeled_value_from_line(v, "directory,"))
        .ok_or(std::io::Error::from(std::io::ErrorKind::InvalidData))
        .map(|v| Some(v.to_string()))
}

fn read_names_from_descript<P>(dir_path: P) -> Result<(String, String), std::io::Error>
where
    P: AsRef<Path>,
{
    let path = dir_path.as_ref().join("ghost/master/descript.txt");
    if !path.is_file() {
        return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
    }

    let contents = load_setting_file(&path)?;

    let sakura_name = contents
        .lines()
        .find_map(|v| get_labeled_value_from_line(v, "sakura.name,"))
        .ok_or(std::io::Error::from(std::io::ErrorKind::InvalidData))
        .map(|v| v.to_string())?;

    let kero_name = contents
        .lines()
        .find_map(|v| get_labeled_value_from_line(v, "kero.name,"))
        .unwrap_or("")
        .to_string();

    Ok((sakura_name, kero_name))
}

fn get_labeled_value_from_line<'a>(line: &'a str, label: &str) -> Option<&'a str> {
    let line = line.trim();
    line.strip_prefix(label)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod build {
        use std::path::PathBuf;

        use tempfile::tempdir;

        use crate::io::load_json;

        use super::*;

        #[test]
        fn checking_value() {
            let out_dir = tempdir().unwrap();
            let out_path = out_dir.path().join("test.json");

            let input = vec![
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_target/ghost/valid"),
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_target/ghost/invalid"),
            ];
            let args = BuildArgs::new(input, out_path.clone());

            build(&args).unwrap();

            let result = load_json(&out_path).unwrap();
            assert_eq!(
                result.ghost_list(),
                &vec![
                    GhostData::new(
                        "aaa".to_string(),
                        "さくらAAA".to_string(),
                        "ケロAAA".to_string()
                    ),
                    GhostData::new("bbb".to_string(), "さくらBBB".to_string(), "".to_string()),
                    GhostData::new(
                        "ccc".to_string(),
                        "さくらCCC".to_string(),
                        "ケロCCC".to_string()
                    )
                ]
            );

            out_dir.close().unwrap();
        }
    }

    mod read_ghost_collection {
        use std::path::PathBuf;

        use super::*;

        #[test]
        fn some_value_when_valid_dir() {
            let dir_path =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_target/ghost/valid");
            let result = read_ghost_collection(&dir_path).unwrap();
            assert_eq!(
                result,
                vec![
                    GhostData::new(
                        "aaa".to_string(),
                        "さくらAAA".to_string(),
                        "ケロAAA".to_string()
                    ),
                    GhostData::new("bbb".to_string(), "さくらBBB".to_string(), "".to_string()),
                    GhostData::new(
                        "ccc".to_string(),
                        "さくらCCC".to_string(),
                        "ケロCCC".to_string()
                    )
                ]
            );
        }

        #[test]
        fn none_when_invalid_dir() {
            let dir_path =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_target/ghost/invalid");
            assert!(read_ghost_collection(&dir_path).is_none());
        }
    }

    mod read_directory_name {
        use std::path::PathBuf;

        use super::*;

        #[test]
        fn success_when_valid_installtxt_exists() {
            let dir_name =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_target/ghost/valid/aaa");
            let result = read_directory_name(&dir_name).unwrap();
            assert_eq!(result, "aaa");
        }

        #[test]
        fn success_when_installtxt_does_not_exist() {
            let dir_name =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_target/ghost/valid/ccc");
            let result = read_directory_name(&dir_name).unwrap();
            assert_eq!(result, "ccc");
        }

        #[test]
        fn failed_when_invalid_installtxt_exists() {
            let dir_name =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_target/ghost/invalid/ccc");
            assert!(read_directory_name(&dir_name).is_err());
        }
    }

    mod read_directory_name_from_installtxt {
        use std::path::PathBuf;

        use super::*;

        #[test]
        fn success_none_when_no_installtxt() {
            let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_target");
            let result = read_directory_name_from_installtxt(&root).unwrap();
            assert!(result.is_none());
        }

        #[test]
        fn success_some_value_when_valid_installtxt() {
            let root =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_target/ghost/valid/aaa");
            let result = read_directory_name_from_installtxt(&root).unwrap();
            assert_eq!(result, Some("aaa".to_string()));
        }

        #[test]
        fn failed_when_invalid_installtxt() {
            let root =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_target/ghost/invalid/ccc");
            assert!(read_directory_name_from_installtxt(&root).is_err());
        }
    }

    mod read_names_from_descript {
        use std::path::PathBuf;

        use super::*;

        #[test]
        fn failed_when_no_descript() {
            let dir_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_target");
            assert!(read_names_from_descript(&dir_path).is_err());
        }

        #[test]
        fn failed_when_no_sakura_name() {
            let dir_path =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_target/ghost/invalid/aaa");
            assert!(read_names_from_descript(&dir_path).is_err());
        }

        #[test]
        fn success_when_valid_descript() {
            let dir_path =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_target/ghost/valid/aaa");
            let (sakura_name, kero_name) = read_names_from_descript(&dir_path).unwrap();
            assert_eq!(sakura_name, "さくらAAA".to_string());
            assert_eq!(kero_name, "ケロAAA".to_string());
        }
    }

    mod get_labeled_value_from_line {
        use super::*;

        #[test]
        fn some_value_when_valid_str() {
            let line = "directory,taidanaSanoSan";
            let label = "directory,";
            let result = get_labeled_value_from_line(line, label);
            assert_eq!(result, Some("taidanaSanoSan"));
        }

        #[test]
        fn none_when_invalid_str() {
            let line = "aaadirectory,bbb";
            let label = "directory,";
            assert!(get_labeled_value_from_line(line, label).is_none());
        }
    }
}
