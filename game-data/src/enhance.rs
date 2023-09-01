use std::borrow::Cow;

use crate::{
    lang::{Nameable, TextEntry, TextTable},
    GameData, LanguageData,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct EnhanceRegistry {
    instances: Vec<Enhance>,
    effects: Vec<EnhanceEffect>,
}

#[derive(Serialize, Deserialize)]
pub struct EnhanceLang {
    captions: TextTable,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Enhance {
    pub instance_id: u32,
    pub effect_id: u32,
    pub caption_id: u32,
    pub param_1: f32,
    pub param_2: f32,
}

#[derive(Serialize, Deserialize)]
pub struct EnhanceEffect {
    pub id: u32,
    pub param: u16,
}

impl EnhanceRegistry {
    pub fn get_instance(&self, id: u32) -> Option<&Enhance> {
        self.instances
            .binary_search_by_key(&id, |i| i.instance_id)
            .map(|i| &self.instances[i])
            .ok()
    }

    pub fn register_instance(&mut self, instance: Enhance) {
        let index = self
            .instances
            .binary_search_by_key(&instance.instance_id, |i| i.instance_id)
            .expect_err("duplicate enhance instance");
        self.instances.insert(index, instance);
    }

    pub fn register_effect(&mut self, effect: EnhanceEffect) {
        let index = self
            .effects
            .binary_search_by_key(&effect.id, |e| e.id)
            .expect_err("duplicate enhance instance");
        self.effects.insert(index, effect);
    }
}

impl EnhanceLang {
    pub fn new(captions: TextTable) -> Self {
        Self { captions }
    }
}

impl Enhance {
    pub fn format<'l>(&self, game: &GameData, lang: &'l LanguageData) -> Option<Cow<'l, str>> {
        let caption = self.get_name_str(lang)?;
        if caption.contains('[') {
            // TODO: format
        }
        Some(caption.into())
    }
}

impl Nameable for Enhance {
    fn get_name<'l>(&self, language: &'l LanguageData) -> Option<&'l TextEntry> {
        language.enhance.captions.get(self.caption_id as usize)
    }
}
