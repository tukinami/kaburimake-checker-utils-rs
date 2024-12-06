use chrono::Utc;

use crate::{
    ast::{GhostData, GhostJson},
    config::AppendArgs,
    io::{load_json, write_json},
};

use super::{build::read_ghost_collection, unique_fold};

pub(super) fn append(args: &AppendArgs) -> Result<(), std::io::Error> {
    let json = load_json(args.output())?;

    let mut ghost_list = Vec::new();
    for p in args.input().iter() {
        if let Some(list) = read_ghost_collection(p) {
            ghost_list.extend_from_slice(&list);
        }
    }

    let json = append_body(&json, &ghost_list);

    write_json(args.output(), &json)
}

fn append_body(json: &GhostJson, appends: &[GhostData]) -> GhostJson {
    println!(
        "raw length: original: {} + append: {} = {}",
        json.ghost_list().len(),
        appends.len(),
        json.ghost_list().len() + appends.len()
    );

    let ghost_list = json
        .ghost_list()
        .iter()
        .chain(appends.iter())
        .fold(Vec::new(), unique_fold);

    println!("result length: {}", ghost_list.len());

    GhostJson::new(Utc::now(), ghost_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod append {
        use std::path::PathBuf;

        use tempfile::tempdir;

        use crate::ast::GhostData;

        use super::*;

        #[test]
        fn checking_value() {
            let out_dir = tempdir().unwrap();

            let json_path = out_dir.path().join("json.json");
            let json = GhostJson::new(
                Utc::now(),
                vec![GhostData::new(
                    "a_d".to_string(),
                    "a_s".to_string(),
                    "a_k".to_string(),
                )],
            );
            write_json(&json_path, &json).unwrap();

            let input = vec![
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_target/ghost/valid"),
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_target/ghost/invalid"),
            ];

            let args = AppendArgs::new(input.clone(), json_path.clone());

            append(&args).unwrap();

            let result = load_json(&json_path).unwrap();
            assert_eq!(
                result.ghost_list(),
                &vec![
                    GhostData::new("a_d".to_string(), "a_s".to_string(), "a_k".to_string(),),
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

    mod append_body {
        use crate::ast::GhostData;

        use super::*;

        #[test]
        fn checking_value_when_all_unique() {
            let json = GhostJson::new(
                Utc::now(),
                vec![GhostData::new(
                    "a_d".to_string(),
                    "a_s".to_string(),
                    "a_k".to_string(),
                )],
            );
            let appends = vec![
                GhostData::new("b_d".to_string(), "b_s".to_string(), "b_k".to_string()),
                GhostData::new("c_d".to_string(), "c_s".to_string(), "c_k".to_string()),
            ];

            let result = append_body(&json, &appends);
            assert_eq!(
                result.ghost_list(),
                &vec![
                    GhostData::new("a_d".to_string(), "a_s".to_string(), "a_k".to_string()),
                    GhostData::new("b_d".to_string(), "b_s".to_string(), "b_k".to_string()),
                    GhostData::new("c_d".to_string(), "c_s".to_string(), "c_k".to_string())
                ]
            );
        }

        #[test]
        fn checking_value_when_not_unique() {
            let json = GhostJson::new(
                Utc::now(),
                vec![GhostData::new(
                    "a_d".to_string(),
                    "a_s".to_string(),
                    "a_k".to_string(),
                )],
            );
            let appends = vec![
                GhostData::new("a_d".to_string(), "a_s".to_string(), "a_k".to_string()),
                GhostData::new("a_d".to_string(), "a_s".to_string(), "a_k".to_string()),
            ];

            let result = append_body(&json, &appends);
            assert_eq!(
                result.ghost_list(),
                &vec![GhostData::new(
                    "a_d".to_string(),
                    "a_s".to_string(),
                    "a_k".to_string()
                ),]
            );
        }
    }
}
