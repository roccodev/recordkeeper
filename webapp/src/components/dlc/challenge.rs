use std::{fmt::Display, str::FromStr};

use game_data::lang::Nameable;
use recordkeeper::dlc::{ChallengeDifficulty, ChallengeRank};
use yew::prelude::*;

use crate::{
    components::edit::{editor, CheckboxInput, EnumInput, NumberInput, StringInput},
    data::Data,
    lang::Text,
    util::FiniteF32,
    ToHtml,
};

editor!(
    ClearCountEditor,
    u32,
    get |editor, save| save.challenge_battle.challenge(editor.id).clear_count,
    set |editor, save, new| save.challenge_battle.challenge_mut(editor.id).clear_count = new,
    capture id: usize
);

editor!(
    BonusEditor,
    bool,
    get |editor, save| save.challenge_battle.challenge(editor.id).has_bonus,
    set |editor, save, new| save.challenge_battle.challenge_mut(editor.id).has_bonus = new,
    capture id: usize
);

editor!(
    ClearEditor,
    bool,
    get |editor, save| save.challenge_battle.challenge(editor.id).cleared,
    set |editor, save, new| save.challenge_battle.challenge_mut(editor.id).cleared = new,
    capture id: usize
);

editor!(
    NewEditor,
    bool,
    get |editor, save| save.challenge_battle.challenge(editor.id).new,
    set |editor, save, new| save.challenge_battle.challenge_mut(editor.id).new = new,
    capture id: usize
);

editor!(
    RewardEditor,
    bool,
    get |editor, save| save.challenge_battle.challenge(editor.id).claimed_reward,
    set |editor, save, new| save.challenge_battle.challenge_mut(editor.id).claimed_reward = new,
    capture id: usize
);

editor!(
    RankEditor,
    ChallengeRank,
    get |editor, save| save.challenge_battle.challenge(editor.id).get_rank(editor.difficulty),
    set |editor, save, new| save.challenge_battle.challenge_mut(editor.id).set_rank(editor.difficulty, new),
    capture id: usize, difficulty: ChallengeDifficulty
);

editor!(
    pub TimeEditor,
    Time,
    get |editor, save| Time(FiniteF32::try_from(save.challenge_battle.challenge(editor.id).get_best_time(editor.difficulty)).unwrap()),
    set |editor, save, new| save.challenge_battle.challenge_mut(editor.id).set_best_time(editor.difficulty, new.0.into()),
    capture id: usize, difficulty: ChallengeDifficulty
);

#[derive(Properties, PartialEq, Clone)]
pub struct ChallengeProps {
    pub id: usize,
    pub difficulty: ChallengeDifficulty,
}

#[derive(Properties, PartialEq, Clone)]
pub struct TimeInputProps {
    pub editor: TimeEditor,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Time(pub FiniteF32);

#[function_component]
pub fn ChallengeRow(props: &ChallengeProps) -> Html {
    let data = use_context::<Data>().unwrap();

    let id = props.id;
    let challenge = data
        .game()
        .dlc
        .challenge
        .get_challenge(id)
        .expect("challenge not found");

    html! {
        <>
            <tr>
                <th>{props.id.to_string()}</th>
                <td>{challenge.get_name_str(data.lang())}</td>
                <td>
                    <EnumInput<RankEditor> editor={RankEditor { id, difficulty: props.difficulty }} />
                </td>
                <td>
                    <StringInput<Time, TimeEditor> editor={TimeEditor { id, difficulty: props.difficulty }} />
                </td>
                <td>
                    <NumberInput<ClearCountEditor> editor={ClearCountEditor { id }} />
                </td>
                <td><CheckboxInput<ClearEditor> editor={ClearEditor { id }} /></td>
                <td><CheckboxInput<NewEditor> editor={NewEditor { id }} /> </td>
                <td><CheckboxInput<BonusEditor> editor={BonusEditor { id }} /></td>
                <td><CheckboxInput<RewardEditor> editor={RewardEditor { id }} /></td>
            </tr>
        </>
    }
}

fn parse_time(value: &str) -> Option<f32> {
    let mut split = value.split(':');
    let mins = split.next()?.parse::<u32>().ok()?;
    let mut ss_ms = split.next()?.split('.');
    let secs = ss_ms.next()?.parse::<u32>().ok()?;
    let millis = ss_ms.next()?.parse::<u32>().ok()?;

    if millis > 999 || secs > 59 {
        return None;
    }

    let res = millis as f64 / 1000.0 + secs as f64 + mins as f64 * 60.0;
    Some(res as f32)
}

impl ToHtml for ChallengeRank {
    fn to_html(&self) -> Html {
        let key = match self {
            ChallengeRank::None => "none",
            ChallengeRank::S => "s",
            ChallengeRank::A => "a",
            ChallengeRank::B => "b",
            ChallengeRank::C => "c",
        };
        html!(<Text path={format!("challenge_rank_{key}")} />)
    }
}

impl FromStr for Time {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        parse_time(value)
            .and_then(|t| FiniteF32::try_from(t).ok())
            .map(Time)
            .ok_or(())
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let seconds = f32::from(self.0) as f64;
        let ms = seconds.fract();
        write!(
            f,
            "{:02.0}:{:02.0}.{:03.0}",
            (seconds / 60.0).trunc(),
            (seconds % 60.0).trunc(),
            ms * 1000.0
        )
    }
}
