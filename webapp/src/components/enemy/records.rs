use game_data::lang::Nameable;
use recordkeeper::enemy::Difficulty;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

use crate::{
    components::edit::{editor, CheckboxInput, Editor, NumberInput},
    data::Data,
    save::SaveContext,
};

editor!(
    SeenEditor,
    bool,
    get |editor, save| save.enemy_tombstones[editor.idx].seen,
    set |editor, save, new| save.enemy_tombstones[editor.idx].seen = new,
    capture idx: usize
);

editor!(
    DefeatedEditor,
    bool,
    get |editor, save| save.enemy_tombstones[editor.idx].defeated,
    set |editor, save, new| save.enemy_tombstones[editor.idx].defeated = new,
    capture idx: usize
);

editor!(
    RematchEditor,
    u8,
    get |editor, save| save.enemy_tombstones[editor.idx].get_highest_rematch(editor.difficulty),
    set |editor, save, new| save.enemy_tombstones[editor.idx].set_highest_rematch(editor.difficulty, new),
    capture idx: usize, difficulty: Difficulty
);

editor!(
    TimeEditor,
    u16,
    get |editor, save| {
        let record = save.enemy_tombstones[editor.idx].time_record(editor.difficulty);
        if editor.rematch { record.best_time_highest_level } else { record.best_time }
    },
    set |editor, save, new| {
        let record = save.enemy_tombstones[editor.idx].time_record_mut(editor.difficulty);
        let time = if editor.rematch { &mut record.best_time_highest_level } else { &mut record.best_time };
        *time = new;
    },
    capture idx: usize, difficulty: Difficulty, rematch: bool
);

#[derive(Properties, PartialEq, Clone)]
pub struct UniqueMonsterProps {
    pub id: usize,
    pub difficulty: Difficulty,
}

#[derive(Properties, PartialEq, Clone)]
struct TimeInputProps {
    editor: TimeEditor,
}

#[function_component]
pub fn UniqueMonsterRow(props: &UniqueMonsterProps) -> Html {
    let data = use_context::<Data>().unwrap();
    let lang = data.to_lang();

    let idx = props.id.checked_sub(1).unwrap();
    let enemy = &data.game().enemies.unique_monsters[idx];

    html! {
        <>
            <tr>
                <th>{props.id.to_string()}</th>
                <td>{enemy.get_name_str(&lang)}</td>
                <td>
                    <CheckboxInput<SeenEditor> editor={SeenEditor { idx }} />
                </td>
                <td>
                    <CheckboxInput<DefeatedEditor> editor={DefeatedEditor { idx }} />
                </td>
                <td>
                    <NumberInput<RematchEditor> editor={RematchEditor { idx, difficulty: props.difficulty }} max={15} />
                </td>
                <td>
                    <TimeInput editor={TimeEditor { idx, difficulty: props.difficulty, rematch: false }} />
                </td>
                <td>
                    <TimeInput editor={TimeEditor { idx, difficulty: props.difficulty, rematch: true }} />
                </td>
            </tr>
        </>
    }
}

#[function_component]
fn TimeInput(props: &TimeInputProps) -> Html {
    let save_context = use_context::<SaveContext>().unwrap();

    let seconds = props.editor.get(save_context.get().get_save());
    let iso = format!("00:{:02}:{:02}", seconds / 60, seconds % 60);

    let update = {
        let editor = props.editor;
        let save_context = save_context.clone();
        Callback::from(move |event: InputEvent| {
            let new = seconds_from_event(event).unwrap_or(seconds);
            save_context.edit(move |save| editor.set(save, new))
        })
    };

    html! {
        <input class="input" type="time" value={iso} oninput={update} step={1} />
    }
}

fn seconds_from_event(event: InputEvent) -> Option<u16> {
    let target: Option<EventTarget> = event.target();
    let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok())?;
    let value = input.value();
    let mut split = value.split(':').skip(1);
    let mins = split.next()?.parse::<u16>().ok()?;
    let secs = split.next()?.parse::<u16>().ok()?;

    let res = secs + mins * 60;
    (res < 6000).then_some(res)
}
