use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::rtb_model::{Imp, Request};

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

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ImpCondition {
    NativeVideo = 1,
    NativeImage = 2,
    Video = 3,
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

#[derive(Deserialize, PartialEq, Debug)]
pub struct Resource {
    pub path: String,
    pub imp_cond: ImpCondition,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Conf {
    pub resources: Vec<Resource>,
}