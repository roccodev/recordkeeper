use std::borrow::Cow;

use game_data::{
    item::Item,
    lang::{Filterable, Nameable},
    LanguageData,
};
use recordkeeper::{dlc::CRAFTED_ITEM_ID, item::ItemType};
use yew::prelude::*;

use crate::{
    data::Data,
    lang::{LangManager, Text},
};

use super::select::HtmlName;

pub mod edit;

#[derive(Clone, PartialEq, Copy)]
pub struct HtmlItem(pub Item);

#[derive(Properties, PartialEq)]
pub struct ItemDisplayProps {
    pub item: Item,
}

#[function_component]
pub fn ItemDisplay(props: &ItemDisplayProps) -> Html {
    let data = use_context::<Data>().unwrap();

    html! {
        <>
            <span><small>{props.item.id}{". "}</small></span>
            <span>
                {if props.item.item_type.0 != ItemType::Accessory || props.item.id != u32::from(CRAFTED_ITEM_ID) {
                    html!{
                        <>
                            {props.item.get_name_str(data.lang())
                                .map(Html::from)
                                .unwrap_or_else(|| html!(<Text path="item_unnamed" />))}
                        </>
                    }
                } else {
                    html!(<b><Text path="item_masha" /></b>)
                }}
            </span>
            <span><small>{" ("}<Text path={format!("item_rarity_{}", props.item.rarity.lang_id())} />{")"}</small></span>
        </>
    }
}

impl HtmlItem {
    fn unnamed_name(&self, lang: &LangManager) -> String {
        if self.0.item_type.0 == ItemType::Accessory && self.0.id == u32::from(CRAFTED_ITEM_ID) {
            lang.translate("item_masha").to_string()
        } else {
            lang.translate("item_unnamed").to_string()
        }
    }
}

impl HtmlName for HtmlItem {
    fn get_name_html(&self, _: &LanguageData) -> Html {
        html!(<ItemDisplay item={self.0} />)
    }

    fn get_search_query<'a, 'b: 'a>(
        &'a self,
        language: &'b LanguageData,
        lang: &'b LangManager,
    ) -> Option<Cow<'a, str>> {
        Some(
            self.0
                .get_name(language)
                .map(|t| t.text().into())
                .unwrap_or_else(|| self.unnamed_name(lang).into()),
        )
    }

    fn get_name_for_filter<'a, 'b: 'a>(
        &'a self,
        language: &'b LanguageData,
        lang: &'b LangManager,
    ) -> Option<Cow<'a, str>> {
        Some(
            self.0
                .get_filter(language)
                .map(|t| t.text_lower().into())
                .unwrap_or_else(|| self.unnamed_name(lang).to_lowercase().into()),
        )
    }
}
