use game_data::lang::Nameable;
use recordkeeper::dlc::{ChallengeDifficulty, ChallengeRank};
use yew::prelude::*;

use crate::{
    components::edit::{editor, CheckboxInput, EnumInput, NumberInput, StringInput},
    data::Data,
    util::FiniteF32,
};

use super::challenge::Time;

editor!(
    ClearCountEditor,
    u32,
    get |editor, save| save.challenge_battle.gauntlet(editor.id).clear_count,
    set |editor, save, new| save.challenge_battle.gauntlet_mut(editor.id).clear_count = new,
    capture id: usize
);

editor!(
    RankEditor,
    ChallengeRank,
    get |editor, save| save.challenge_battle.gauntlet(editor.id).get_rank(editor.difficulty),
    set |editor, save, new| save.challenge_battle.gauntlet_mut(editor.id).set_rank(editor.difficulty, new),
    capture id: usize, difficulty: ChallengeDifficulty
);

editor!(
    PlayCountEditor,
    u32,
    get |editor, save| save.challenge_battle.gauntlet(editor.id).get_play_count(editor.difficulty),
    set |editor, save, new| save.challenge_battle.gauntlet_mut(editor.id).set_play_count(editor.difficulty, new),
    capture id: usize, difficulty: ChallengeDifficulty
);

editor!(
    TimeEditor,
    Time,
    get |editor, save| Time(FiniteF32::try_from(save.challenge_battle.gauntlet(editor.id).get_best_time(editor.difficulty)).unwrap()),
    set |editor, save, new| save.challenge_battle.gauntlet_mut(editor.id).set_best_time(editor.difficulty, new.0.into()),
    capture id: usize, difficulty: ChallengeDifficulty
);

editor!(
    ScoreEditor,
    u32,
    get |editor, save| save.challenge_battle.gauntlet(editor.id).get_high_score(editor.difficulty),
    set |editor, save, new| save.challenge_battle.gauntlet_mut(editor.id).set_high_score(editor.difficulty, new),
    capture id: usize, difficulty: ChallengeDifficulty
);

editor!(
    StageEditor,
    u32,
    get |editor, save| save.challenge_battle.gauntlet(editor.id).get_stage_reached(editor.difficulty),
    set |editor, save, new| save.challenge_battle.gauntlet_mut(editor.id).set_stage_reached(editor.difficulty, new),
    capture id: usize, difficulty: ChallengeDifficulty
);

editor!(
    BonusEditor,
    bool,
    get |editor, save| save.challenge_battle.gauntlet(editor.id).has_bonus,
    set |editor, save, new| save.challenge_battle.gauntlet_mut(editor.id).has_bonus = new,
    capture id: usize
);

editor!(
    ClearEditor,
    bool,
    get |editor, save| save.challenge_battle.gauntlet(editor.id).cleared,
    set |editor, save, new| save.challenge_battle.gauntlet_mut(editor.id).cleared = new,
    capture id: usize
);

editor!(
    NewEditor,
    bool,
    get |editor, save| save.challenge_battle.gauntlet(editor.id).new,
    set |editor, save, new| save.challenge_battle.gauntlet_mut(editor.id).new = new,
    capture id: usize
);

editor!(
    RewardBEditor,
    bool,
    get |editor, save| save.challenge_battle.gauntlet(editor.id).reward_b,
    set |editor, save, new| save.challenge_battle.gauntlet_mut(editor.id).reward_b = new,
    capture id: usize
);

editor!(
    RewardAEditor,
    bool,
    get |editor, save| save.challenge_battle.gauntlet(editor.id).reward_a,
    set |editor, save, new| save.challenge_battle.gauntlet_mut(editor.id).reward_a = new,
    capture id: usize
);

#[derive(Properties, PartialEq, Clone)]
pub struct GauntletProps {
    pub id: usize,
    pub difficulty: ChallengeDifficulty,
}

#[function_component]
pub fn GauntletRow(props: &GauntletProps) -> Html {
    let data = use_context::<Data>().unwrap();

    let id = props.id;
    let challenge = data
        .game()
        .dlc
        .challenge
        .get_gauntlet(id)
        .expect("gauntlet not found");
    let difficulty = props.difficulty;

    html! {
        <>
            <tr>
                <th>{props.id.to_string()}</th>
                <td>{challenge.get_name_str(data.lang())}</td>
                <td>
                    <EnumInput<RankEditor> editor={RankEditor { id, difficulty }} />
                </td>
                <td>
                    <NumberInput<ScoreEditor> editor={ScoreEditor { id, difficulty }} />
                </td>
                <td>
                    <NumberInput<StageEditor> editor={StageEditor { id, difficulty }} />
                </td>
                <td>
                    <StringInput<Time, TimeEditor> editor={TimeEditor { id, difficulty }} />
                </td>
                <td>
                    <NumberInput<PlayCountEditor> editor={PlayCountEditor { id, difficulty }} />
                </td>
                <td>
                    <NumberInput<ClearCountEditor> editor={ClearCountEditor { id }} />
                </td>
                <td><CheckboxInput<ClearEditor> editor={ClearEditor { id }} /></td>
                <td><CheckboxInput<NewEditor> editor={NewEditor { id }} /> </td>
                <td><CheckboxInput<BonusEditor> editor={BonusEditor { id }} /></td>
                <td><CheckboxInput<RewardBEditor> editor={RewardBEditor { id }} /></td>
                <td><CheckboxInput<RewardAEditor> editor={RewardAEditor { id }} /></td>
            </tr>
        </>
    }
}
