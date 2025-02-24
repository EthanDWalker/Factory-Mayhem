use crate::{circle, factory_crate, sprite};

pub struct SpriteList {
    pub background: Vec<sprite::Sprite>,
    pub player: Option<crate::player::Player>,
    pub factory_belts: Vec<sprite::Sprite>,
    pub factory_crates: Vec<factory_crate::FactoryCrate>,
    pub spotlights: Vec<circle::Circle>,
    pub other_outline: Vec<sprite::Sprite>,
    pub explosion: Option<sprite::Sprite>,
    pub gui: Vec<sprite::Sprite>,
}
