use game_data::lang::Nameable;
use recordkeeper::enemy::Difficulty;
use yew::prelude::*;

use crate::data::Data;

#[derive(Properties, PartialEq, Clone)]
pub struct UniqueMonsterProps {
    pub id: usize,
    pub difficulty: Difficulty,
}

#[function_component]
pub fn UniqueMonsterRow(props: &UniqueMonsterProps) -> Html {
    let data = use_context::<Data>().unwrap();
    let lang = data.to_lang();

    let enemy = &data.game().enemies.unique_monsters[props.id.checked_sub(1).unwrap()];

    html! {
        <>
            <tr>
                <th>{props.id.to_string()}</th>
                <td>{enemy.get_name_str(&lang)}</td>
                <td>
                </td>
                <td>
                </td>
                <td>
                </td>
                <td>
                </td>
            </tr>
        </>
    }
}
