use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub(crate) struct GhostJson {
    update: String,
    #[serde(rename = "ghostList")]
    ghost_list: Vec<GhostData>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub(crate) struct GhostData {
    directory: String,
    #[serde(rename = "sakuraName")]
    sakura_name: String,
    #[serde(rename = "keroName")]
    kero_name: String,
}

impl GhostJson {
    pub fn new(update: DateTime<Utc>, ghost_list: Vec<GhostData>) -> GhostJson {
        let update = update.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

        GhostJson { update, ghost_list }
    }

    #[cfg(test)]
    pub fn update(&self) -> &String {
        &self.update
    }

    pub fn ghost_list(&self) -> &Vec<GhostData> {
        &self.ghost_list
    }
}

impl GhostData {
    pub fn new(directory: String, sakura_name: String, kero_name: String) -> GhostData {
        GhostData {
            directory,
            sakura_name,
            kero_name,
        }
    }

    pub fn directory(&self) -> &String {
        &self.directory
    }

    pub fn sakura_name(&self) -> &String {
        &self.sakura_name
    }

    pub fn kero_name(&self) -> &String {
        &self.kero_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod ghost_json {
        use super::*;

        mod new {
            use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

            use super::*;

            #[test]
            fn checking_value() {
                let d = NaiveDate::from_ymd_opt(2023, 12, 18).unwrap();
                let t = NaiveTime::from_hms_milli_opt(7, 18, 31, 808).unwrap();
                let update = NaiveDateTime::new(d, t).and_utc();
                let ghost_list = vec![
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
                ];
                let result = GhostJson::new(update, ghost_list.clone());
                assert_eq!(result.update(), "2023-12-18T07:18:31.808Z");
                assert_eq!(result.ghost_list(), &ghost_list);
            }
        }

        mod deserialize {
            use super::*;

            #[test]
            fn success_when_valid_str() {
                let case = r#"{"update":"2023-12-18T07:18:31.808Z","ghostList":[{"directory":"100th_year","sakuraName":"霊","keroName":""},{"directory":"FoxTheory","sakuraName":"リサ","keroName":"book"},{"directory":"tanumki","sakuraName":"きつね","keroName":"たぬき"},{"directory":"tcidelam","sakuraName":"シデラム","keroName":""}]}"#;
                let result: GhostJson = serde_json::from_str(case).unwrap();
                assert_eq!(result.update, "2023-12-18T07:18:31.808Z".to_string());
                assert_eq!(
                    result.ghost_list,
                    vec![
                        GhostData {
                            directory: "100th_year".to_string(),
                            sakura_name: "霊".to_string(),
                            kero_name: "".to_string(),
                        },
                        GhostData {
                            directory: "FoxTheory".to_string(),
                            sakura_name: "リサ".to_string(),
                            kero_name: "book".to_string(),
                        },
                        GhostData {
                            directory: "tanumki".to_string(),
                            sakura_name: "きつね".to_string(),
                            kero_name: "たぬき".to_string(),
                        },
                        GhostData {
                            directory: "tcidelam".to_string(),
                            sakura_name: "シデラム".to_string(),
                            kero_name: "".to_string(),
                        },
                    ]
                );

                let case = r#"{"update":"2023-12-18T07:18:31.808Z","ghostList":[]}"#;
                let result: GhostJson = serde_json::from_str(case).unwrap();
                assert_eq!(result.update, "2023-12-18T07:18:31.808Z".to_string());
                assert_eq!(result.ghost_list.len(), 0);
            }

            #[test]
            fn failed_when_invalid_str() {
                let case = r#"{"ghostList":[{"directory":"100th_year","sakuraName":"霊","keroName":""},{"directory":"FoxTheory","sakuraName":"リサ","keroName":"book"},{"directory":"tanumki","sakuraName":"きつね","keroName":"たぬき"},{"directory":"tcidelam","sakuraName":"シデラム","keroName":""}]}"#;
                assert!(serde_json::from_str::<GhostJson>(case).is_err());

                let case = r#"{"update":"2023-12-18T07:18:31.808Z"}"#;
                assert!(serde_json::from_str::<GhostJson>(case).is_err());
            }
        }

        mod serialize {
            use super::*;

            #[test]
            fn checking_value() {
                let case = GhostJson {
                    update: "2023-12-18T07:18:31.808Z".to_string(),
                    ghost_list: vec![
                        GhostData {
                            directory: "100th_year".to_string(),
                            sakura_name: "霊".to_string(),
                            kero_name: "".to_string(),
                        },
                        GhostData {
                            directory: "FoxTheory".to_string(),
                            sakura_name: "リサ".to_string(),
                            kero_name: "book".to_string(),
                        },
                        GhostData {
                            directory: "tanumki".to_string(),
                            sakura_name: "きつね".to_string(),
                            kero_name: "たぬき".to_string(),
                        },
                        GhostData {
                            directory: "tcidelam".to_string(),
                            sakura_name: "シデラム".to_string(),
                            kero_name: "".to_string(),
                        },
                    ],
                };

                let result = serde_json::to_string(&case).unwrap();
                assert_eq!(
                    result,
                    r#"{"update":"2023-12-18T07:18:31.808Z","ghostList":[{"directory":"100th_year","sakuraName":"霊","keroName":""},{"directory":"FoxTheory","sakuraName":"リサ","keroName":"book"},{"directory":"tanumki","sakuraName":"きつね","keroName":"たぬき"},{"directory":"tcidelam","sakuraName":"シデラム","keroName":""}]}"#
                );
            }
        }
    }
}
