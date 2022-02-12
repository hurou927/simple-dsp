use std::fs::File;

use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum ImpCondition {
    NativeVideo = 1,
    NativeImage = 2,
    Video = 3,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct RawAppConf {
    pub resources: Vec<RawResource>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct RawResource {
    pub uri: String,
    pub cond: ImpCondition,
    pub path: String,
}

#[derive(Clone, Debug)]
pub struct AppConf {
    pub resources: Vec<ResResource>,
}

#[derive(Clone, Debug)]
pub struct ResResource {
    pub uri: String,
    pub imp_condition: ImpCondition,
    pub content: String,
}

impl From<&RawResource> for ResResource {
    fn from(ra: &RawResource) -> Self {
        let content =
            std::fs::read_to_string(&ra.path).expect(&format!("no such file. path: {}", ra.path));
        ResResource {
            uri: ra.uri.clone(),
            imp_condition: ra.cond.clone(),
            content,
        }
    }
}

impl From<&RawAppConf> for AppConf {
    fn from(ra: &RawAppConf) -> Self {
        AppConf {
            resources: ra.resources.iter().map(|r| ResResource::from(r)).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(doc: &str) -> RawAppConf {
        serde_yaml::from_str(doc).unwrap()
    }

    #[test]
    fn parse_yml() {
        let expected = RawAppConf {
            resources: vec![
                RawResource {
                    uri: String::from("/hoge/fuga"),
                    path: String::from("./aa/bb.json"),
                    cond: ImpCondition::NativeVideo,
                },
                RawResource {
                    uri: String::from("/fuga/hoge"),
                    path: String::from("./cc/dd.json"),
                    cond: ImpCondition::NativeImage,
                },
                RawResource {
                    uri: String::from("/hoge/hoge"),
                    path: String::from("./ee/ff.json"),
                    cond: ImpCondition::Video,
                },
            ],
        };
        assert_eq!(
            parse(
                r#"
resources:
    - uri: /hoge/fuga
      path: ./aa/bb.json
      cond: 1
    - uri: /fuga/hoge
      path: ./cc/dd.json
      cond: 2
    - uri: /hoge/hoge
      path: ./ee/ff.json
      cond: 3
"#
            ),
            expected
        )
    }
}
