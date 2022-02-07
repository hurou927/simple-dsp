
use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ImpCondition {
    NativeVideo = 1,
    NativeImage = 2,
    Video = 3,
}




#[derive(Deserialize, PartialEq, Debug)]
struct RawAppConf {
    resources: Vec<RawResource>
}

#[derive(Deserialize, PartialEq, Debug)]
struct RawResource {
    path: String,
    cond: ImpCondition,
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
            resources: vec! [
                RawResource {
                    path: String::from("./aa/bb.json"),
                    cond: ImpCondition::NativeVideo
                },
                RawResource {
                    path: String::from("./cc/dd.json"),
                    cond: ImpCondition::NativeImage
                },
                RawResource {
                    path: String::from("./ee/ff.json"),
                    cond: ImpCondition::Video
                }
            ]
        };
        assert_eq!(
            parse(r#"
resources:
    - path: ./aa/bb.json
      cond: 1
    - path: ./cc/dd.json
      cond: 2
    - path: ./ee/ff.json
      cond: 3
"#),
            expected
            )
    }
}
