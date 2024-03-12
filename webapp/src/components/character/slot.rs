use game_data::{item::ItemDetails, LanguageData};
use recordkeeper::{
    character::class::ClassAccessory,
    character::slot::{EmptySlotMut, Slot, SlotMut},
    item::ItemType,
    SaveData,
};
use ybc::{Button, Control, Field, Icon};
use yew::prelude::*;
use yew_feather::X;

use crate::{
    components::{
        edit::Editor,
        item::HtmlItem,
        select::{HtmlName, Options, SearchSelect},
    },
    data::Data,
    lang::LangManager,
    save::SaveContext,
};

use super::ClassAccessor;

#[derive(Properties, PartialEq)]
pub struct SlotProps<E: Editor + PartialEq, I: PartialEq + 'static>
where
    E::Target: PartialEq,
{
    pub editor: E,
    pub possible_values: &'static [I],
    pub id_mapper: Callback<&'static I, E::Target>,
}

#[derive(Properties, PartialEq, Clone, Copy)]
pub struct AccessorySlotProps {
    pub char: ClassAccessor,
    pub slot_idx: AccessorySlotIndex,
}

#[derive(PartialEq, Clone, Copy)]
pub enum AccessorySlotIndex {
    Regular(usize),
    /// Future Redeemed manual-only slot
    Manual,
}

#[derive(Clone)]
struct Accessory {
    item: HtmlItem,
    slot_index: u16,
}

#[function_component]
pub fn SlotInput<E, I, N>(props: &SlotProps<E, I>) -> Html
where
    N: PartialEq + 'static,
    E: Editor<Target = Option<N>> + PartialEq,
    I: PartialEq + 'static + Clone + HtmlName,
{
    let save_context = use_context::<SaveContext>().unwrap();
    let data = use_context::<Data>().unwrap();
    let lang = data.to_lang();

    let current = props.editor.get(save_context.get().get_save());
    // TODO: get_by_id callback
    let current = props
        .possible_values
        .iter()
        .enumerate()
        .find(|(_, v)| props.id_mapper.emit(v) == current)
        .map(|(i, _)| i);

    let options: Options<_> = props.possible_values.into();

    let on_type_select = {
        let editor = props.editor;
        let save_context = save_context.clone();
        let id_mapper = props.id_mapper.clone();
        let values = props.possible_values;
        Callback::from(move |idx| {
            let item = &values[idx];
            let id = id_mapper.emit(item);
            save_context.edit(move |save| editor.set(save, id))
        })
    };

    let clear_callback = {
        let editor = props.editor;
        let save_context = save_context.clone();
        Callback::from(move |_: MouseEvent| save_context.edit(move |save| editor.set(save, None)))
    };

    html! {
        <Field classes={classes!("has-addons")}>
            <Control>
                <SearchSelect<I>
                    current={current}
                    options={options}
                    on_select={on_type_select}
                    lang={lang}
                />
            </Control>
            <Control>
                <Button onclick={clear_callback} disabled={current.is_none()}>
                    <Icon><X /></Icon>
                </Button>
            </Control>
        </Field>
    }
}

#[function_component]
pub fn AccessoryInput(props: &AccessorySlotProps) -> Html {
    let save_context = use_context::<SaveContext>().unwrap();
    let save = save_context.get();

    let data = use_context::<Data>().unwrap();
    let lang = data.to_lang();

    let manuals = matches!(props.slot_idx, AccessorySlotIndex::Manual);

    // Accessory slot uses inventory slot index
    let inventory = save.get_save().inventory.slots(ItemType::Accessory);
    let inventory: Options<_> = inventory
        .iter()
        .filter_map(|slot| {
            slot.is_valid()
                .then(|| slot.item_id())
                .and_then(|id| data.game().items.get_item(ItemType::Accessory, id as u32))
                .and_then(|item| {
                    let Some(ItemDetails::Accessory { is_manual }) = item.details else {
                        return Some(item);
                    };
                    // Only enable manuals for manual slots, and enable only manuals
                    // in manual slots
                    (is_manual == manuals).then_some(item)
                })
                .map(|&item| Accessory {
                    item: HtmlItem(item),
                    slot_index: slot.index(),
                })
        })
        .collect();

    let current = props
        .save_slot(save.get_save())
        .get()
        .map(|acc| acc.slot_index() as usize);

    let on_select = {
        let save_context = save_context.clone();
        let props = *props;
        Callback::from(move |idx: usize| {
            save_context.edit(move |save| {
                let slot = save.inventory.slots(ItemType::Accessory)[idx];
                props.save_slot_mut(save).set_from_inventory(&slot)
            })
        })
    };

    let clear_callback = {
        let save_context = save_context.clone();
        let props = *props;
        Callback::from(move |_: MouseEvent| {
            save_context.edit(move |save| props.save_slot_mut(save).set_empty())
        })
    };

    html! {
        <Field classes={classes!("has-addons")}>
            <Control>
                <SearchSelect<Accessory>
                    current={current}
                    options={inventory}
                    on_select={on_select}
                    lang={lang}
                />
            </Control>
            <Control>
                <Button onclick={clear_callback} disabled={current.is_none()}>
                    <Icon><X /></Icon>
                </Button>
            </Control>
        </Field>
    }
}

impl AccessorySlotProps {
    fn save_slot(&self, save: &SaveData) -> Slot<ClassAccessory> {
        match self.slot_idx {
            AccessorySlotIndex::Regular(i) => self.char.class_data(save).accessory_slot(i),
            AccessorySlotIndex::Manual => {
                let ClassAccessor::Character { char, class } = self.char else {
                    panic!("must be a character accessor")
                };
                save.dlc4
                    .battle_manual_slot((char + 1).try_into().unwrap(), class.try_into().unwrap())
            }
        }
    }

    fn save_slot_mut<'a>(&self, save: &'a mut SaveData) -> SlotMut<'a, ClassAccessory> {
        match self.slot_idx {
            AccessorySlotIndex::Regular(i) => self.char.class_data_mut(save).accessory_slot_mut(i),
            AccessorySlotIndex::Manual => {
                let ClassAccessor::Character { char, class } = self.char else {
                    panic!("must be a character accessor")
                };
                save.dlc4.battle_manual_slot_mut(
                    (char + 1).try_into().unwrap(),
                    class.try_into().unwrap(),
                )
            }
        }
    }
}

impl HtmlName for Accessory {
    fn get_name_html(&self, language: &LanguageData) -> Html {
        html! {
            <>
                <span><small>{"["}{self.slot_index}{"] "}</small></span>
                <span>
                    {self.item.get_name_html(language)}
                </span>
            </>
        }
    }

    fn get_search_query<'a, 'b: 'a>(
        &'a self,
        language: &'b LanguageData,
        ui_lang: &'b LangManager,
    ) -> Option<std::borrow::Cow<'a, str>> {
        self.item.get_search_query(language, ui_lang)
    }

    fn get_name_for_filter<'a, 'b: 'a>(
        &'a self,
        language: &'b LanguageData,
        ui_lang: &'b LangManager,
    ) -> Option<std::borrow::Cow<'a, str>> {
        self.item.get_name_for_filter(language, ui_lang)
    }
}
