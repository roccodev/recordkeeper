use game_data::{
    character::Character,
    dlc::challenge::{ChallengeData, GauntletMap, Whimsy},
};
use recordkeeper::dlc::ChallengeDifficulty;
use ybc::{Button, Control, Field, Icon, Tile};
use yew::prelude::*;
use yew_feather::X;

use crate::{
    components::{
        edit::{editor, Editor, EnumInput, NumberInput, StringInput},
        select::{EditorSelector, Options, SearchSelect},
    },
    data::Data,
    lang::Text,
    save::SaveContext,
    util::FiniteF32,
};

#[rustfmt::skip]
macro_rules! edit_num {
    ($name:ident, $field:ident) => {
        edit_num!($name, $field, u32);
    };
    ($name:ident, $field:ident, FF32) => {
        editor!(
            $name,
            FiniteF32,
            get |_, save| FiniteF32::try_from(save.challenge_battle.gauntlet_save().$field).unwrap(),
            set |_, save, new| save.challenge_battle.gauntlet_save_mut().$field = new.into()
        );
    };
    ($name:ident, $field:ident, $ty:ty) => {
        editor!(
            $name,
            $ty,
            get |_, save| save.challenge_battle.gauntlet_save().$field as $ty,
            set |_, save, new| save.challenge_battle.gauntlet_save_mut().$field = new as _
        );
    };
}

edit_num!(IdEditor, gauntlet_id);

edit_num!(WeatherEditor, game_weather);
edit_num!(StageEditor, last_stage);
edit_num!(ScoreEditor, current_score);
edit_num!(ShuffleEditor, shuffle_tickets);
edit_num!(NoKoStreakEditor, no_ko_streak);
edit_num!(NoponStoneEditor, nopon_stone_reward);
edit_num!(HeroCountEditor, hero_buy_count);
edit_num!(EmblemCountEditor, emblems_bought);
edit_num!(WatchCountEditor, nopwatch_buy_count);
edit_num!(ScoreGainedEditor, score_gained);

edit_num!(ChainGaugeEditor, chain_gauge, FF32);
edit_num!(LaunchGaugeEditor, launch_charge, FF32);
edit_num!(WatchGaugeEditor, nopwatch_gauge, FF32);

#[rustfmt::skip]
editor!(
    LeadEditor,
    u32,
    get |_, save| save.challenge_battle.gauntlet_save().get_lead_character(),
    set |_, save, new| save.challenge_battle.gauntlet_save_mut().set_lead_character(new)
);

#[rustfmt::skip]
editor!(
    MapIdEditor,
    u32,
    get |_, save| save.challenge_battle.gauntlet_save().map_id.checked_add(75).unwrap(),
    set |_, save, new| save.challenge_battle.gauntlet_save_mut().map_id = new.checked_sub(75).unwrap()
);

#[rustfmt::skip]
editor!(
    WhimsyEditor,
    u32,
    get |editor, save| save.challenge_battle.gauntlet_save().whimsy[editor.index],
    set |editor, save, new| save.challenge_battle.gauntlet_save_mut().whimsy[editor.index] = new,
    capture index: usize
);

#[rustfmt::skip]
editor!(
    DifficultyEditor,
    ChallengeDifficulty,
    get |_, save| save.challenge_battle.gauntlet_save().get_challenge_difficulty(),
    set |_, save, new| save.challenge_battle.gauntlet_save_mut().set_challenge_difficulty(new)
);

#[derive(Properties, PartialEq)]
struct EntryProps {
    pub label: AttrValue,
    pub children: Children,
}

#[function_component]
pub fn GauntletSaveState() -> Html {
    let data = use_context::<Data>().unwrap();

    html! {
        <>
            <Tile>
                // ID, difficulty
                <Field classes="is-grouped">
                    <Entry label="gauntlet_save_challenge">
                        <EditorSelector<IdEditor, ChallengeData> editor={IdEditor {}} values={data.game().dlc.challenge.gauntlets.as_ref()} />
                    </Entry>

                    <Entry label="difficulty">
                        <EnumInput<DifficultyEditor> editor={DifficultyEditor {}} />
                    </Entry>

                    <Entry label="gauntlet_save_lead">
                        <EditorSelector<LeadEditor, Character> editor={LeadEditor {}} values={data.game().characters.characters()} />
                    </Entry>
                </Field>

                // Party editor
            </Tile>

            <Tile>
                // Current stage, map ID,

                <Field classes="is-grouped">
                    <Entry label="gauntlet_save_stage">
                        <NumberInput<StageEditor> editor={StageEditor {}} />
                    </Entry>

                    <Entry label="gauntlet_save_score">
                        <NumberInput<ScoreEditor> editor={ScoreEditor {}} />
                    </Entry>

                    <Entry label="gauntlet_save_stone">
                        <NumberInput<NoponStoneEditor> editor={NoponStoneEditor {}} />
                    </Entry>

                    <Entry label="gauntlet_save_no_ko">
                        <NumberInput<NoKoStreakEditor> editor={NoKoStreakEditor {}} />
                    </Entry>

                    <Entry label="gauntlet_save_map">
                        <EditorSelector<MapIdEditor, GauntletMap> editor={MapIdEditor {}} values={data.game().dlc.challenge.gauntlet_maps.as_ref()} />
                    </Entry>
                </Field>
            </Tile>

            <Tile>
                // Gauges

                <Field classes="is-grouped">
                    <Entry label="gauntlet_save_chain">
                        <StringInput<FiniteF32, ChainGaugeEditor> editor={ChainGaugeEditor {}} />
                    </Entry>

                    <Entry label="gauntlet_save_launch">
                        <StringInput<FiniteF32, LaunchGaugeEditor> editor={LaunchGaugeEditor {}} />
                    </Entry>

                    <Entry label="gauntlet_save_watch">
                        <StringInput<FiniteF32, WatchGaugeEditor> editor={WatchGaugeEditor {}} />
                    </Entry>

                    <Entry label="gauntlet_save_shuffle">
                        <NumberInput<ShuffleEditor> editor={ShuffleEditor {}} />
                    </Entry>
                </Field>
            </Tile>

            <Tile>
                // Emblem shop, prices

                <Field classes="is-grouped">
                    <Entry label="gauntlet_save_purchase_hero">
                        <NumberInput<HeroCountEditor> editor={HeroCountEditor {}} />
                    </Entry>

                    <Entry label="gauntlet_save_purchase_watch">
                        <NumberInput<WatchCountEditor> editor={WatchCountEditor {}} />
                    </Entry>

                    <Entry label="gauntlet_save_purchase_emblem">
                        <NumberInput<EmblemCountEditor> editor={EmblemCountEditor {}} />
                    </Entry>

                    <Entry label="gauntlet_save_total_score">
                        <NumberInput<ScoreGainedEditor> editor={ScoreGainedEditor {}} />
                    </Entry>
                </Field>
            </Tile>

            <Tile>
                <WhimsySelect />
            </Tile>
        </>
    }
}

#[function_component]
fn WhimsySelect() -> Html {
    let data = use_context::<Data>().unwrap();
    let save_context = use_context::<SaveContext>().unwrap();
    let lang = data.to_lang();

    let whimsy = Options::from(data.game().dlc.challenge.whimsy.as_ref());
    let get_whimsy = |i| {
        let editor = WhimsyEditor { index: i };
        editor.get(save_context.get().get_save())
    };
    let update_whimsy = |i| {
        let save_context = save_context.clone();
        Callback::from(move |whimsy: usize| {
            let editor = WhimsyEditor { index: i };
            save_context
                .edit(move |save| editor.set(save, whimsy.wrapping_add(1).try_into().unwrap()))
        })
    };

    html! {
        <Field>
            <label class="label"><Text path="gauntlet_save_whimsy" /></label>
            <Control>
                <Field classes="is-grouped">
                    {for (0..=1).map(|i| {
                        let value = get_whimsy(i);
                        html! {
                            <Control>
                                <Field classes={classes!("has-addons")}>
                                    <SearchSelect<Whimsy>
                                        current={value.checked_sub(1).map(|i| i as usize)}
                                        options={whimsy.clone()}
                                        on_select={update_whimsy(i)}
                                        lang={lang.clone()}
                                    />
                                    <Control>
                                        <Button onclick={update_whimsy(i).reform(|_| usize::MAX)} disabled={value == 0}>
                                            <Icon><X /></Icon>
                                        </Button>
                                    </Control>
                                </Field>
                            </Control>
                        }
                    })}
                </Field>
            </Control>
        </Field>
    }
}

#[function_component]
fn Entry(props: &EntryProps) -> Html {
    html! {
        <Control>
            <Field>
                <label class="label"><Text path={&props.label} /></label>
                <Control>
                    {for props.children.clone()}
                </Control>
            </Field>
        </Control>
    }
}
