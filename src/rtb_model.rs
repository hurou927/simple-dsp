use serde::{de, Deserialize};

#[derive(Deserialize, PartialEq, Debug)]
pub struct Video {}

#[derive(Deserialize, PartialEq, Debug)]
pub struct NativeVideo {}

#[derive(Deserialize, PartialEq, Debug)]
pub struct NativeImage {
    #[serde(rename = "type")]
    pub img_type: i32, // should be optional????
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct NativeAsset {
    pub id: i32,
    pub img: Option<NativeImage>,
    pub video: Option<NativeVideo>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct NativeRequest {
    pub ver: String,
    pub assets: Vec<NativeAsset>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Native {
    #[serde(deserialize_with = "nested_json_or_struct")]
    pub request: NativeRequest,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ImpExt {}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Imp {
    pub id: String,
    pub video: Option<Video>,
    pub native: Option<Native>,
    pub ext: Option<ImpExt>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Request {
    pub id: String,
    #[serde(default)]
    pub imp: Vec<Imp>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(untagged)]
enum NativeRequestType {
    #[serde(with = "serde_with::json::nested")]
    AsNestedJson(NativeRequest),
    AsStruct(NativeRequest),
}

fn nested_json_or_struct<'de, D>(deserializer: D) -> Result<NativeRequest, D::Error>
where
    D: de::Deserializer<'de>,
{
    match NativeRequestType::deserialize(deserializer)? {
        NativeRequestType::AsStruct(value) => Ok(value),
        NativeRequestType::AsNestedJson(value) => Ok(value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(doc: &str) -> Request {
        serde_json::from_str(doc).unwrap()
    }

    #[test]
    fn empty_imp() {
        let expected = Request {
            id: "req_id".to_owned(),
            imp: Vec::new(),
        };
        assert!(parse(r#"{"id":"req_id", "imp":[]}"#) == expected);
    }
    #[test]
    fn null_imp() {
        let expected = Request {
            id: "req_id".to_owned(),
            imp: Vec::new(),
        };
        assert!(parse(r#"{"id":"req_id"}"#) == expected);
    }
    #[test]
    fn native_as_struct() {
        let native_req = NativeRequest {
            ver: "1.2".to_owned(),
            assets: vec![
                NativeAsset {
                    id: 1,
                    img: None,
                    video: Some(NativeVideo {}),
                },
                NativeAsset {
                    id: 2,
                    img: Some(NativeImage { img_type: 3 }),
                    video: None,
                },
            ],
        };

        let expected = Request {
            id: "req_id".to_owned(),
            imp: vec![Imp {
                id: "imp_id".to_owned(),
                video: None,
                native: Some(Native {
                    request: native_req,
                }),
                ext: None,
            }],
        };
        assert!(
            parse(
                r#"
{
  "id": "req_id",
  "imp": [
    {
      "id": "imp_id",
      "native": {
        "request": {
          "ver": "1.2",
          "assets": [
            {
              "id": 1,
              "video": {}
            },
            {
              "id": 2,
              "img": {
                "type": 3
              }
            }
          ]
        }
      }
    }
  ]
}
"#
            ) == expected
        );
    }
    #[test]
    fn native_as_string() {
        let native_req = NativeRequest {
            ver: "1.2".to_owned(),
            assets: vec![
                NativeAsset {
                    id: 1,
                    img: None,
                    video: Some(NativeVideo {}),
                },
                NativeAsset {
                    id: 2,
                    img: Some(NativeImage { img_type: 3 }),
                    video: None,
                },
            ],
        };

        let expected = Request {
            id: "req_id".to_owned(),
            imp: vec![Imp {
                id: "imp_id".to_owned(),
                video: None,
                native: Some(Native {
                    request: native_req,
                }),
                ext: None,
            }],
        };
        //  pbpaste |  jq '.imp[0] | .native.request' -rc | tr -d "\n" | jq -Rs
        assert!(
            parse(
                r#"
{
  "id": "req_id",
  "imp": [
    {
      "id": "imp_id",
      "native": {
        "request": "{\"ver\":\"1.2\",\"assets\":[{\"id\":1,\"video\":{}},{\"id\":2,\"img\":{\"type\":3}}]}"
      }
    }
  ]
}
"#
            ) == expected
        );
    }
}
