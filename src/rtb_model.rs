use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum NativeRequestEnum {
    #[serde(with = "serde_with::json::nested")]
    AsString(NativeRequest),
    AsStruct(NativeRequest),
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Video {}

#[derive(Deserialize, PartialEq, Debug)]
pub struct NativeRequest {
    ver: String
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Native {
    request: NativeRequestEnum,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(doc: &str) ->  Request {
        serde_json::from_str(doc).unwrap()
    }

    #[test]
    fn empty_imp() {
        let expected = Request{
            id: "req_id".to_owned(),
            imp: Vec::new()
        };
        assert!(parse(r#"{"id":"req_id", "imp":[]}"#) ==  expected );
    }
    #[test]
    fn null_imp() {
        let expected = Request{
            id: "req_id".to_owned(),
            imp: Vec::new()
        };
        assert!(parse(r#"{"id":"req_id"}"#) ==  expected );
    }
    #[test]
    fn native_as_struct() {

        let native_req = NativeRequestEnum::AsStruct(NativeRequest{
           ver: "1.2".to_owned()
        });

        let expected = Request{
            id: "req_id".to_owned(),
            imp: vec![
                Imp {
                    id: "imp_id".to_owned(),
                    video: None,
                    native: Some(
                        Native{
                            request: native_req
                    }),
                    ext: None
                }
            ]
        };
        assert!(parse(r#"{"id":"req_id","imp":[{"id":"imp_id","native":{"request":{"ver":"1.2"}}}]}"#) ==  expected );
    }
    #[test]
    fn native_as_string() {

        let native_req = NativeRequestEnum::AsString(NativeRequest{
           ver: "1.2".to_owned()
        });

        let expected = Request{
            id: "req_id".to_owned(),
            imp: vec![
                Imp {
                    id: "imp_id".to_owned(),
                    video: None,
                    native: Some(
                        Native{
                            request: native_req
                    }),
                    ext: None
                }
            ]
        };
        assert!(parse(r#"{"id":"req_id","imp":[{"id":"imp_id","native":{"request":"{\"ver\":\"1.2\"}"}}]}"#) ==  expected );
    }
}

