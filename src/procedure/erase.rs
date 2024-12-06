use chrono::Utc;

use crate::{
    ast::{GhostData, GhostJson},
    config::EraseArgs,
    io::{load_json, write_json},
    procedure::unique_fold,
};

pub(super) fn erase(args: &EraseArgs) -> Result<(), std::io::Error> {
    let json = load_json(args.target())?;

    let ghost_json = erase_body(args, &json);

    write_json(args.target(), &ghost_json)
}

fn erase_body(args: &EraseArgs, json: &GhostJson) -> GhostJson {
    let ghost_list = json
        .ghost_list()
        .iter()
        .filter_map(|v| erase_filter_map(v, args.directory(), args.sakura_name(), args.kero_name()))
        .fold(Vec::new(), unique_fold);

    println!("input raw length: {}", json.ghost_list().len());
    println!("output length: {}", ghost_list.len());

    GhostJson::new(Utc::now(), ghost_list)
}

fn erase_filter_map<'a>(
    v: &'a GhostData,
    directory: Option<&String>,
    sakura_name: Option<&String>,
    kero_name: Option<&String>,
) -> Option<&'a GhostData> {
    match (directory, sakura_name, kero_name) {
        (Some(d), _, _) if d == v.directory() => None,
        (_, Some(s), _) if s == v.sakura_name() => None,
        (_, _, Some(k)) if k == v.kero_name() => None,
        _ => Some(v),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod erase {
        use tempfile::tempdir;

        use super::*;

        #[test]
        fn checking_value() {
            let out_dir = tempdir().unwrap();
            let out_path = out_dir.path().join("test.json");
            let original_contents = r#"{"update":"2023-12-18T07:18:31.808Z","ghostList":[{"directory":"100th_year","sakuraName":"霊","keroName":""},{"directory":"FoxTheory","sakuraName":"リサ","keroName":"book"},{"directory":"tanumki","sakuraName":"きつね","keroName":"たぬき"},{"directory":"tcidelam","sakuraName":"シデラム","keroName":""}]}"#;
            let original_json = serde_json::from_str(&original_contents).unwrap();
            write_json(&out_path, &original_json).unwrap();

            let args = EraseArgs::new(out_path.clone(), Some("100th_year".to_string()), None, None);

            erase(&args).unwrap();

            let result = load_json(&out_path).unwrap();
            assert_eq!(
                result.ghost_list(),
                &vec![
                    GhostData::new(
                        "FoxTheory".to_string(),
                        "リサ".to_string(),
                        "book".to_string(),
                    ),
                    GhostData::new(
                        "tanumki".to_string(),
                        "きつね".to_string(),
                        "たぬき".to_string(),
                    ),
                    GhostData::new(
                        "tcidelam".to_string(),
                        "シデラム".to_string(),
                        "".to_string(),
                    ),
                ]
            );

            out_dir.close().unwrap();
        }
    }

    mod erase_body {
        use std::path::PathBuf;

        use super::*;

        #[test]
        fn erase_when_specified_value() {
            let args = EraseArgs::new(PathBuf::new(), Some("100th_year".to_string()), None, None);
            let json = GhostJson::new(
                Utc::now(),
                vec![
                    GhostData::new("100th_year".to_string(), "霊".to_string(), "".to_string()),
                    GhostData::new(
                        "FoxTheory".to_string(),
                        "リサ".to_string(),
                        "book".to_string(),
                    ),
                    GhostData::new(
                        "tanumki".to_string(),
                        "きつね".to_string(),
                        "たぬき".to_string(),
                    ),
                    GhostData::new(
                        "tcidelam".to_string(),
                        "シデラム".to_string(),
                        "".to_string(),
                    ),
                ],
            );
            let result = erase_body(&args, &json);
            assert_eq!(
                result.ghost_list(),
                &vec![
                    GhostData::new(
                        "FoxTheory".to_string(),
                        "リサ".to_string(),
                        "book".to_string(),
                    ),
                    GhostData::new(
                        "tanumki".to_string(),
                        "きつね".to_string(),
                        "たぬき".to_string(),
                    ),
                    GhostData::new(
                        "tcidelam".to_string(),
                        "シデラム".to_string(),
                        "".to_string(),
                    ),
                ]
            );
        }

        #[test]
        fn not_erase_when_no_specified_value() {
            let args = EraseArgs::new(PathBuf::new(), Some("d_invalid".to_string()), None, None);
            let json = GhostJson::new(
                Utc::now(),
                vec![
                    GhostData::new("100th_year".to_string(), "霊".to_string(), "".to_string()),
                    GhostData::new(
                        "FoxTheory".to_string(),
                        "リサ".to_string(),
                        "book".to_string(),
                    ),
                    GhostData::new(
                        "tanumki".to_string(),
                        "きつね".to_string(),
                        "たぬき".to_string(),
                    ),
                    GhostData::new(
                        "tcidelam".to_string(),
                        "シデラム".to_string(),
                        "".to_string(),
                    ),
                ],
            );
            let result = erase_body(&args, &json);
            assert_eq!(result.ghost_list(), json.ghost_list());
        }

        #[test]
        fn not_erase_when_nothing_value() {
            let args = EraseArgs::new(PathBuf::new(), None, None, None);
            let json = GhostJson::new(
                Utc::now(),
                vec![
                    GhostData::new("100th_year".to_string(), "霊".to_string(), "".to_string()),
                    GhostData::new(
                        "FoxTheory".to_string(),
                        "リサ".to_string(),
                        "book".to_string(),
                    ),
                    GhostData::new(
                        "tanumki".to_string(),
                        "きつね".to_string(),
                        "たぬき".to_string(),
                    ),
                    GhostData::new(
                        "tcidelam".to_string(),
                        "シデラム".to_string(),
                        "".to_string(),
                    ),
                ],
            );
            let result = erase_body(&args, &json);
            assert_eq!(result.ghost_list(), json.ghost_list());
        }
    }

    mod erase_filter_map {
        use super::*;

        #[test]
        fn none_when_same_directory() {
            let v = GhostData::new(
                "d_valid".to_string(),
                "s_valid".to_string(),
                "k_valid".to_string(),
            );
            let directory = Some("d_valid".to_string());
            let sakura_name = None;
            let kero_name = None;
            let result = erase_filter_map(
                &v,
                directory.as_ref(),
                sakura_name.as_ref(),
                kero_name.as_ref(),
            );
            assert!(result.is_none());

            let sakura_name = Some("s_valid".to_string());
            let kero_name = Some("k_valid".to_string());
            let result = erase_filter_map(
                &v,
                directory.as_ref(),
                sakura_name.as_ref(),
                kero_name.as_ref(),
            );
            assert!(result.is_none());

            let sakura_name = Some("s_valid".to_string());
            let kero_name = Some("k_valid".to_string());
            let result = erase_filter_map(
                &v,
                directory.as_ref(),
                sakura_name.as_ref(),
                kero_name.as_ref(),
            );
            assert!(result.is_none());

            let sakura_name = Some("s_invalid".to_string());
            let kero_name = Some("k_invalid".to_string());
            let result = erase_filter_map(
                &v,
                directory.as_ref(),
                sakura_name.as_ref(),
                kero_name.as_ref(),
            );
            assert!(result.is_none());
        }

        #[test]
        fn none_when_same_sakura_name() {
            let v = GhostData::new(
                "d_valid".to_string(),
                "s_valid".to_string(),
                "k_valid".to_string(),
            );
            let directory = None;
            let sakura_name = Some("s_valid".to_string());
            let kero_name = None;
            let result = erase_filter_map(
                &v,
                directory.as_ref(),
                sakura_name.as_ref(),
                kero_name.as_ref(),
            );
            assert!(result.is_none());

            let directory = Some("d_valid".to_string());
            let kero_name = Some("k_valid".to_string());
            let result = erase_filter_map(
                &v,
                directory.as_ref(),
                sakura_name.as_ref(),
                kero_name.as_ref(),
            );
            assert!(result.is_none());

            let directory = Some("d_invalid".to_string());
            let kero_name = Some("k_invalid".to_string());
            let result = erase_filter_map(
                &v,
                directory.as_ref(),
                sakura_name.as_ref(),
                kero_name.as_ref(),
            );
            assert!(result.is_none());
        }

        #[test]
        fn none_when_same_kero_name() {
            let v = GhostData::new(
                "d_valid".to_string(),
                "s_valid".to_string(),
                "k_valid".to_string(),
            );
            let directory = None;
            let sakura_name = None;
            let kero_name = Some("k_valid".to_string());
            let result = erase_filter_map(
                &v,
                directory.as_ref(),
                sakura_name.as_ref(),
                kero_name.as_ref(),
            );
            assert!(result.is_none());

            let directory = Some("d_valid".to_string());
            let sakura_name = Some("s_valid".to_string());
            let result = erase_filter_map(
                &v,
                directory.as_ref(),
                sakura_name.as_ref(),
                kero_name.as_ref(),
            );
            assert!(result.is_none());

            let directory = Some("d_invalid".to_string());
            let sakura_name = Some("s_invalid".to_string());
            let result = erase_filter_map(
                &v,
                directory.as_ref(),
                sakura_name.as_ref(),
                kero_name.as_ref(),
            );
            assert!(result.is_none());
        }

        #[test]
        fn some_value_when_all_different() {
            let v = GhostData::new(
                "d_valid".to_string(),
                "s_valid".to_string(),
                "k_valid".to_string(),
            );
            let directory = Some("d_invalid".to_string());
            let sakura_name = Some("s_invalid".to_string());
            let kero_name = Some("k_invalid".to_string());
            let result = erase_filter_map(
                &v,
                directory.as_ref(),
                sakura_name.as_ref(),
                kero_name.as_ref(),
            );
            assert_eq!(result, Some(&v));
        }

        #[test]
        fn some_value_when_all_none() {
            let v = GhostData::new(
                "d_valid".to_string(),
                "s_valid".to_string(),
                "k_valid".to_string(),
            );
            let directory = None;
            let sakura_name = None;
            let kero_name = None;
            let result = erase_filter_map(
                &v,
                directory.as_ref(),
                sakura_name.as_ref(),
                kero_name.as_ref(),
            );
            assert_eq!(result, Some(&v));
        }
    }
}
