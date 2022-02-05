use serde::{de, Deserialize};

#[derive(Deserialize, PartialEq, Debug)]
pub struct Video {}

#[derive(Deserialize, PartialEq, Debug)]
pub struct NativeRequest {
    ver: String,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Native {
    #[serde(deserialize_with = "nested_json_or_struct")]
    request: NativeRequest,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ImpExt {}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Imp {
    id: String,
    video: Option<Video>,
    native: Option<Native>,
    ext: Option<ImpExt>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Request {
    id: String,
    #[serde(default)]
    imp: Vec<Imp>,
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
            parse(r#"{"id":"req_id","imp":[{"id":"imp_id","native":{"request":{"ver":"1.2"}}}]}"#)
                == expected
        );
    }
    #[test]
    fn native_as_string() {
        let native_req = NativeRequest {
            ver: "1.2".to_owned(),
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
                r#"{"id":"req_id","imp":[{"id":"imp_id","native":{"request":"{\"ver\":\"1.2\"}"}}]}"#
            ) == expected
        );
    }
}
