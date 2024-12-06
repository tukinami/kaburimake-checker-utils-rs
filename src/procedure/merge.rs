use chrono::Utc;

use crate::{
    ast::GhostJson,
    config::MergeArgs,
    io::{load_json, write_json},
    procedure::unique_fold,
};

pub(super) fn merge(args: &MergeArgs) -> Result<(), std::io::Error> {
    let mut jsons = Vec::new();
    for p in args.input().iter() {
        match load_json(p) {
            Ok(v) => jsons.push(v),
            Err(e) => {
                eprintln!("{}: {}", p.display(), e);
            }
        }
    }

    let json = merge_body(&jsons);

    write_json(args.output(), &json)
}

fn merge_body(jsons: &[GhostJson]) -> GhostJson {
    let ghost_list_iter = jsons.iter().flat_map(|v| v.ghost_list().iter());
    let old_size_hint = ghost_list_iter.clone().size_hint();
    let old_size = old_size_hint.1.unwrap_or(old_size_hint.0);

    println!("input raw length: {}", old_size);

    let ghost_list = ghost_list_iter.fold(Vec::new(), unique_fold);

    println!("output length: {}", ghost_list.len());

    GhostJson::new(Utc::now(), ghost_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod merge {
        use tempfile::tempdir;

        use crate::ast::GhostData;

        use super::*;

        #[test]
        fn checking_value() {
            let out_dir = tempdir().unwrap();

            let json_a_path = out_dir.path().join("json_a.json");
            let json_a = GhostJson::new(
                Utc::now(),
                vec![GhostData::new(
                    "a_d".to_string(),
                    "a_s".to_string(),
                    "a_k".to_string(),
                )],
            );
            write_json(&json_a_path, &json_a).unwrap();

            let json_b_path = out_dir.path().join("json_b.json");
            let json_b = GhostJson::new(
                Utc::now(),
                vec![GhostData::new(
                    "a_d".to_string(),
                    "a_s".to_string(),
                    "a_k".to_string(),
                )],
            );
            write_json(&json_b_path, &json_b).unwrap();

            let json_c_path = out_dir.path().join("json_c.json");
            let json_c = GhostJson::new(
                Utc::now(),
                vec![GhostData::new(
                    "a_d".to_string(),
                    "a_s".to_string(),
                    "a_k".to_string(),
                )],
            );
            write_json(&json_c_path, &json_c).unwrap();

            let out_path = out_dir.path().join("test.json");
            let args = MergeArgs::new(
                vec![
                    json_a_path.clone(),
                    json_b_path.clone(),
                    json_c_path.clone(),
                ],
                out_path.clone(),
            );

            merge(&args).unwrap();

            let result = load_json(&out_path).unwrap();
            assert_eq!(
                result.ghost_list(),
                &vec![GhostData::new(
                    "a_d".to_string(),
                    "a_s".to_string(),
                    "a_k".to_string(),
                )]
            );

            out_dir.close().unwrap();
        }
    }

    mod merge_body {
        use crate::ast::GhostData;

        use super::*;

        #[test]
        fn checking_value_when_all_unique() {
            let json_a = GhostJson::new(
                Utc::now(),
                vec![GhostData::new(
                    "a_d".to_string(),
                    "a_s".to_string(),
                    "a_k".to_string(),
                )],
            );
            let json_b = GhostJson::new(
                Utc::now(),
                vec![GhostData::new(
                    "b_d".to_string(),
                    "b_s".to_string(),
                    "b_k".to_string(),
                )],
            );
            let json_c = GhostJson::new(
                Utc::now(),
                vec![GhostData::new(
                    "c_d".to_string(),
                    "c_s".to_string(),
                    "c_k".to_string(),
                )],
            );
            let jsons = vec![json_a, json_b, json_c];

            let result = merge_body(&jsons);
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
            let json_a = GhostJson::new(
                Utc::now(),
                vec![GhostData::new(
                    "a_d".to_string(),
                    "a_s".to_string(),
                    "a_k".to_string(),
                )],
            );
            let json_b = GhostJson::new(
                Utc::now(),
                vec![GhostData::new(
                    "a_d".to_string(),
                    "a_s".to_string(),
                    "a_k".to_string(),
                )],
            );
            let json_c = GhostJson::new(
                Utc::now(),
                vec![GhostData::new(
                    "a_d".to_string(),
                    "a_s".to_string(),
                    "a_k".to_string(),
                )],
            );
            let jsons = vec![json_a, json_b, json_c];

            let result = merge_body(&jsons);
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
