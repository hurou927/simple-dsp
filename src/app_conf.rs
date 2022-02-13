use std::{error::Error, fs::File, io::BufReader, path::PathBuf};

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
struct RawAppConf {
    resources: Vec<RawResource>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
struct RawResource {
    uri: String,
    cond: ImpCondition,
    path: String,
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

impl ResResource {
    fn from(ra: &RawResource) -> Result<Self, Box<dyn Error>> {
        let content = std::fs::read_to_string(&ra.path)?;
        Ok(ResResource {
            uri: ra.uri.clone(),
            imp_condition: ra.cond.clone(),
            content,
        })
    }
}

impl AppConf {
    fn from(ra: &RawAppConf) -> Result<Self, Box<dyn Error>> {
        ra.resources
            .iter()
            .map(|raw_resource| ResResource::from(raw_resource))
            .collect::<Result<Vec<ResResource>, Box<dyn Error>>>()
            .map(|resources| AppConf { resources })
    }
}

pub fn read_app_conf(path: &PathBuf) -> Result<AppConf, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let raw_app_conf: RawAppConf = serde_yaml::from_reader(reader)?;
    AppConf::from(&raw_app_conf)
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
