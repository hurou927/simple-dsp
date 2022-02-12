use crate::{
    app_conf::{ImpCondition, ResResource},
    rtb_model::{Imp, Request},
};

#[derive(Debug)]
pub struct ImpInfo {
    pub imp_id: String,
}

impl From<&Imp> for ImpInfo {
    fn from(imp: &Imp) -> Self {
        ImpInfo {
            imp_id: imp.id.clone(),
        }
    }
}

fn find_imp(imp_condition: &ImpCondition, request: &Request) -> Option<ImpInfo> {
    match imp_condition {
        ImpCondition::NativeVideo => request
            .imp
            .iter()
            .find(|imp| is_native_video(imp))
            .map(|imp| ImpInfo::from(imp)),

        ImpCondition::NativeImage => request
            .imp
            .iter()
            .find(|imp| is_native_image(imp))
            .map(|imp| ImpInfo::from(imp)),

        ImpCondition::Video => request
            .imp
            .iter()
            .find(|imp| is_video(imp))
            .map(|imp| ImpInfo::from(imp)),
    }
}

fn is_video(imp: &Imp) -> bool {
    imp.video.is_some()
}

fn is_native_video(imp: &Imp) -> bool {
    match &imp.native {
        Some(native) => native.request.assets.iter().any(|x| x.video.is_some()),
        None => false,
    }
}

fn is_native_image(imp: &Imp) -> bool {
    match &imp.native {
        Some(native) => native
            .request
            .assets
            .iter()
            .any(|x| x.img.iter().any(|img| img.img_type == 3)),
        None => false,
    }
}

fn replace_macro(content: &str, imp_info: &ImpInfo) -> String {
    content.replace("$[XX_IMP_ID]", &imp_info.imp_id)
}

pub fn select_resource_with_replacing_macro(
    resource: &ResResource,
    request: &Request,
) -> Option<String> {
    find_imp(&resource.imp_condition, request).map(|imp_info| {
        tracing::info!("detected imp_info. {:?}", imp_info);
        replace_macro(&resource.content, &imp_info)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace() {
        let imp_info = ImpInfo {
            imp_id: String::from("imp_id"),
        };
        assert_eq!(
            replace_macro(r#"{"id": "$[XX_IMP_ID]", "id": "$[XX_IMP_ID]"}"#, &imp_info),
            r#"{"id": "imp_id", "id": "imp_id"}"#
        );
    }
}
