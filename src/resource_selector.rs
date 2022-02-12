use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    app_conf::ImpCondition,
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

impl ImpCondition {
    pub fn apply(&self, request: &Request) -> Option<ImpInfo> {
        match self {
            Self::NativeVideo => request
                .imp
                .iter()
                .find(|imp| is_native_video(imp))
                .map(|imp| ImpInfo::from(imp)),

            Self::NativeImage => request
                .imp
                .iter()
                .find(|imp| is_native_image(imp))
                .map(|imp| ImpInfo::from(imp)),

            Self::Video => request
                .imp
                .iter()
                .find(|imp| is_video(imp))
                .map(|imp| ImpInfo::from(imp)),
        }
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
    resource.imp_condition.apply(request).map(|imp_info| {
        tracing::info!("detected imp_info. {:?}", imp_info);
        replace_macro(&resource.content, &imp_info)
    })
}
