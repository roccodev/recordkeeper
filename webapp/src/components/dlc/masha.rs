use std::borrow::Cow;

use game_data::{dlc::masha::CraftTypeText, enhance::Enhance, GameData, LanguageData};
use recordkeeper::{
    dlc::{Statistic, MASHA_STAT_BOOSTS_MAX},
    item::{edit::ItemEditor, ItemType},
};
use ybc::{Field, Modal, Table};
use yew::prelude::*;

use crate::{
    components::{
        edit::{editor, Editor, EnumInput, NumberInput},
        select::{HtmlName, Options, SearchSelect},
    },
    data::Data,
    lang::{LangManager, Text},
    save::SaveContext,
    ToHtml,
};

#[derive(Properties, PartialEq)]
pub struct MashaModalProps {
    pub item_slot: Option<usize>,
    #[prop_or_default]
    pub close_callback: Callback<()>,
}

#[derive(Properties, PartialEq)]
struct StatBoostProps {
    pub item_slot: usize,
    pub index: usize,
}

#[derive(Clone)]
struct DisplayType(CraftTypeText);

#[derive(Clone, Copy)]
struct DisplayEnhance {
    enhance: &'static Enhance,
    game: &'static GameData,
}

editor!(
    pub LevelEditor,
    u8,
    get |editor, save| {
        save.inventory.slots(ItemType::Accessory)[editor.item_slot]
            .craft_data(save)
            .unwrap()
            .level
    },
    set |editor, save, new_value| {
        ItemEditor::new(save, ItemType::Accessory, editor.item_slot)
            .craft_data_mut()
            .unwrap()
            .level = new_value
    },
    capture item_slot: usize
);

editor!(
    pub BoostStatEditor,
    Statistic,
    get |editor, save| {
        Statistic::from_repr(save.inventory.slots(ItemType::Accessory)[editor.item_slot]
            .craft_data(save)
            .unwrap()
            .stat_boosts[editor.boost]
            .stat).expect("unknown stat")
    },
    set |editor, save, new_value| {
        ItemEditor::new(save, ItemType::Accessory, editor.item_slot)
            .craft_data_mut()
            .unwrap()
            .stat_boosts[editor.boost]
            .stat = new_value as u16
    },
    capture item_slot: usize, boost: usize
);

editor!(
    pub BoostValueEditor,
    u16,
    get |editor, save| {
        save.inventory.slots(ItemType::Accessory)[editor.item_slot]
            .craft_data(save)
            .unwrap()
            .stat_boosts[editor.boost]
            .amount
    },
    set |editor, save, new_value| {
        ItemEditor::new(save, ItemType::Accessory, editor.item_slot)
            .craft_data_mut()
            .unwrap()
            .stat_boosts[editor.boost]
            .amount = new_value
    },
    capture item_slot: usize, boost: usize
);

#[function_component]
pub fn MashaModal(props: &MashaModalProps) -> Html {
    let save_context = use_context::<SaveContext>().unwrap();
    let save = save_context.get();
    let save = save.get_save();
    let data = use_context::<Data>().unwrap();
    let lang = data.to_lang();

    let slot = match props.item_slot {
        Some(slot) => slot,
        None => return html!(),
    };

    let Some(craft_data) = save.inventory.slots(ItemType::Accessory)[slot].craft_data(save) else {
        return html!();
    };

    let level_editor = LevelEditor {
        item_slot: props.item_slot.unwrap(),
    };
    let level = level_editor.get(save);

    let type_options: Options<DisplayType> = data
        .lang()
        .dlc
        .masha
        .type_names
        .iter()
        .map(|t| DisplayType(t.clone()))
        .collect();

    let enhance_options: Options<DisplayEnhance> = data
        .game()
        .dlc
        .masha
        .enhances
        .iter()
        .map(|t| DisplayEnhance {
            game: data.game(),
            enhance: t.get_enhance_for_level(data.game(), level as u32).unwrap(),
        })
        .collect();

    let selected_type = data
        .lang()
        .dlc
        .masha
        .index_of(craft_data.display_id.into())
        .unwrap_or_default();
    let on_type_select = {
        let save_context = save_context.clone();
        let options = type_options.clone();
        Callback::from(move |selected| {
            let options = options.clone();
            save_context.edit(move |save| {
                ItemEditor::new(save, ItemType::Accessory, slot)
                    .craft_data_mut()
                    .unwrap()
                    .display_id = options.get(selected).0.id as u16
            })
        })
    };

    let selected_enhance = craft_data.enhance_id as usize - 1;
    let save_context = save_context.clone();
    let on_enhance_select = Callback::from(move |selected| {
        save_context.edit(move |save| {
            ItemEditor::new(save, ItemType::Accessory, slot)
                .craft_data_mut()
                .unwrap()
                .enhance_id = (selected + 1) as u16
        })
    });

    let close_fn = props.close_callback.clone();
    let close_callback = Callback::from(move |_: MouseEvent| {
        close_fn.emit(());
    });

    html! {
        <Modal id="card" classes={classes!("is-active")}>
            <div class="modal-card">
                <header class="modal-card-head">
                    <p class="modal-card-title"><Text path="menu_dlc_masha" /></p>
                    <button class="delete" aria-label="close" onclick={close_callback}></button>
                </header>
                <section class="modal-card-body">
                    <Field>
                        <label class="label"><Text path="masha_item_type" /></label>
                        <SearchSelect<DisplayType>
                            current={Some(selected_type)}
                            options={type_options}
                            on_select={on_type_select}
                            lang={lang.clone()}
                        />
                    </Field>
                    <Field>
                        <label class="label"><Text path="masha_level" /></label>
                        <NumberInput<LevelEditor> editor={level_editor} min={1} max={5} />
                    </Field>
                    <Field>
                        <label class="label"><Text path="masha_enhance" /></label>
                        <SearchSelect<DisplayEnhance>
                            current={Some(selected_enhance)}
                            options={enhance_options}
                            on_select={on_enhance_select}
                            lang={lang.clone()}
                        />
                    </Field>
                    <StatBoosts item_slot={props.item_slot} />
                </section>
            </div>
        </Modal>
    }
}

#[function_component]
fn StatBoosts(props: &MashaModalProps) -> Html {
    html! {
        <Table classes={classes!("is-fullwidth")}>
            <thead>
                <tr>
                    <th><Text path="masha_boost_stat" /></th>
                    <th><Text path="masha_boost_value" /></th>
                </tr>
            </thead>

            <tbody>
                {for (0..MASHA_STAT_BOOSTS_MAX).map(|index| {
                    html!(<StatBoost item_slot={props.item_slot.unwrap()} index={index} />)
                })}
            </tbody>
        </Table>
    }
}

#[function_component]
fn StatBoost(props: &StatBoostProps) -> Html {
    let value_editor = BoostValueEditor {
        item_slot: props.item_slot,
        boost: props.index,
    };

    let stat_editor = BoostStatEditor {
        item_slot: props.item_slot,
        boost: props.index,
    };

    html! {
        <tr>
            <td>
                <EnumInput<BoostStatEditor> editor={stat_editor} />
            </td>
            <td>
                <NumberInput<BoostValueEditor> editor={value_editor} />
            </td>
        </tr>
    }
}

impl HtmlName for DisplayType {
    fn get_name_html(&self, _: &LanguageData) -> Html {
        html!(<>{self.0.text.as_ref().map(|t| t.text())}</>)
    }

    fn get_search_query<'a, 'b: 'a>(
        &'a self,
        _: &'b LanguageData,
        _: &'b LangManager,
    ) -> Option<Cow<'a, str>> {
        self.0.text.as_ref().map(|t| t.text().into())
    }

    fn get_name_for_filter<'a, 'b: 'a>(
        &'a self,
        _: &'b LanguageData,
        _: &'b LangManager,
    ) -> Option<Cow<'a, str>> {
        self.0.text.as_ref().map(|t| t.text_lower().into())
    }
}

impl HtmlName for DisplayEnhance {
    fn get_name_html(&self, lang: &LanguageData) -> Html {
        html!(<>{self.enhance.format(self.game, lang)}</>)
    }

    fn get_search_query<'a, 'b: 'a>(
        &'a self,
        lang: &'b LanguageData,
        _: &'b LangManager,
    ) -> Option<Cow<'a, str>> {
        self.enhance.format(self.game, lang)
    }

    fn get_name_for_filter<'a, 'b: 'a>(
        &'a self,
        lang: &'b LanguageData,
        _: &'b LangManager,
    ) -> Option<Cow<'a, str>> {
        self.enhance.format(self.game, lang)
    }
}

impl ToHtml for Statistic {
    fn to_html(&self) -> Html {
        let id = match self {
            Statistic::HP => "hp",
            Statistic::Strength => "str",
            Statistic::Heal => "heal",
            Statistic::Dexterity => "dex",
            Statistic::Agility => "agi",
            Statistic::CritRate => "crit",
            Statistic::BlockRate => "block",
        };
        html!(<Text path={format!("masha_stat_{id}")} />)
    }
}
