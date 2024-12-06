use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use encoding_rs::{EUC_JP, ISO_2022_JP, SHIFT_JIS, UTF_8};

use crate::ast::GhostJson;

pub(crate) fn load_setting_file<P>(path: P) -> Result<String, std::io::Error>
where
    P: AsRef<Path>,
{
    let mut fs = File::open(path)?;
    let mut buffer_raw = Vec::new();
    fs.read_to_end(&mut buffer_raw)?;

    let mut charset = SHIFT_JIS;
    let temp_contents = String::from_utf8_lossy(&buffer_raw);

    for line in temp_contents.lines() {
        let line_nocase = line.to_ascii_lowercase();

        if let Some(body) = line_nocase.strip_prefix("charset,") {
            charset = match body.trim() {
                "shift_jis" | "shift-jis" => SHIFT_JIS,
                "iso-2022-jp" => ISO_2022_JP,
                "euc-jp" => EUC_JP,
                "utf-8" => UTF_8,
                _ => SHIFT_JIS,
            };
            break;
        }
    }

    let (cow, _encoding_used, had_erros) = charset.decode(&buffer_raw);

    if had_erros {
        Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    } else {
        Ok(cow.to_string())
    }
}

pub(crate) fn load_json<P>(path: P) -> Result<GhostJson, std::io::Error>
where
    P: AsRef<Path>,
{
    let mut fs = File::open(path)?;
    let mut contents = String::new();

    fs.read_to_string(&mut contents)?;

    serde_json::from_str::<GhostJson>(&contents).map_err(|e| {
        if let Some(error_kind) = e.io_error_kind() {
            std::io::Error::from(error_kind)
        } else {
            std::io::Error::from(std::io::ErrorKind::InvalidData)
        }
    })
}

pub(crate) fn write_json<P>(path: P, json: &GhostJson) -> Result<(), std::io::Error>
where
    P: AsRef<Path>,
{
    let contents = serde_json::to_string(json).map_err(|e| {
        if let Some(error_kind) = e.io_error_kind() {
            std::io::Error::from(error_kind)
        } else {
            std::io::Error::from(std::io::ErrorKind::InvalidData)
        }
    })?;

    let mut fs = File::create(path)?;
    fs.write_all(contents.as_bytes())?;
    fs.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod load_setting_file {
        use std::path::PathBuf;

        use super::*;

        #[test]
        fn success_when_valid_file() {
            let base_path =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_target/descript/valid/");

            let case = base_path.clone().join("utf8.txt");
            let result = load_setting_file(case).unwrap();
            assert!(result.contains("テストsakura"));

            let case = base_path.clone().join("shiftjis.txt");
            let result = load_setting_file(case).unwrap();
            assert!(result.contains("テストsakura"));

            let case = base_path.clone().join("eucjp.txt");
            let result = load_setting_file(case).unwrap();
            assert!(result.contains("テストsakura"));
        }

        #[test]
        fn failed_when_invalid_file() {
            let base_path =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_target/descript/invalid/");

            let case = base_path.clone().join("utf8_but-other-note.txt");
            assert!(load_setting_file(case).is_err());

            let case = base_path.clone().join("shiftjis_but-other-note.txt");
            assert!(load_setting_file(case).is_err());

            let case = base_path.clone().join("eucjp_but-other-note.txt");
            let result = load_setting_file(case).unwrap();
            assert!(!result.contains("テストsakura"));
        }
    }

    mod load_json {
        use std::path::PathBuf;

        use crate::ast::GhostData;

        use super::*;

        #[test]
        fn success_when_valid_file() {
            let base_path =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_target/json/valid/");

            let case = base_path.clone().join("Konuka.json");
            let result = load_json(case).unwrap();
            assert_eq!(result.update(), "2023-12-18T07:18:31.808Z");
            assert_eq!(
                result.ghost_list(),
                &vec![
                    GhostData::new("100th_year".to_string(), "霊".to_string(), "".to_string(),),
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
        fn failed_when_invalid_file() {
            let base_path =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_target/json/invalid/");

            let case = base_path.clone().join("no-update.json");
            assert!(load_json(case).is_err());

            let case = base_path.clone().join("no-ghostList.json");
            assert!(load_json(case).is_err());
        }
    }

    mod write_json {
        use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
        use tempfile::tempdir;

        use crate::ast::GhostData;

        use super::*;

        #[test]
        fn success_when_valid_path() {
            let out_dir = tempdir().unwrap();

            let path = out_dir.path().join("test.json");
            let d = NaiveDate::from_ymd_opt(2023, 12, 18).unwrap();
            let t = NaiveTime::from_hms_milli_opt(7, 18, 31, 808).unwrap();
            let update = NaiveDateTime::new(d, t).and_utc();
            let json = GhostJson::new(
                update,
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

            write_json(&path, &json).unwrap();

            let mut fs = File::open(path).unwrap();
            let mut buf = String::new();

            fs.read_to_string(&mut buf).unwrap();
            assert_eq!(
                buf,
                r#"{"update":"2023-12-18T07:18:31.808Z","ghostList":[{"directory":"100th_year","sakuraName":"霊","keroName":""},{"directory":"FoxTheory","sakuraName":"リサ","keroName":"book"},{"directory":"tanumki","sakuraName":"きつね","keroName":"たぬき"},{"directory":"tcidelam","sakuraName":"シデラム","keroName":""}]}"#
            );

            out_dir.close().unwrap();
        }
    }
}
