use recordkeeper::enemy::Difficulty;

use ybc::{Control, Field, Tile, Title};
use yew::prelude::*;

use crate::components::edit::{CheckboxInput, Editor, EnumInput, FlagEditor, ToBool};
use crate::data::Data;
use crate::lang::Text;
use crate::save::SaveContext;
use crate::ToHtml;

#[derive(Clone, Copy, PartialEq)]
struct DifficultyEditor(FlagEditor);

#[function_component]
pub fn Settings() -> Html {
    let data = use_context::<Data>().unwrap();
    let save = use_context::<SaveContext>().unwrap();
    let flags = &data.game().manual.flags;
    let dlc4 = save.get().get().save().is_dlc4();

    let ngp_editor = ToBool(flags.new_game_plus.into());
    let game_clear_editor = ToBool(flags.game_clear.into());
    let difficulty_editor = DifficultyEditor(flags.difficulty.into());
    let fr_complete_editor = ToBool(flags.fr_complete.into());

    html! {
        <Tile classes={classes!("is-child", "notification")}>
            <Title><Text path="meta_settings" /></Title>

            <Field>
                <label class="label"><Text path="difficulty" /></label>
                <Control>
                    <EnumInput<DifficultyEditor> editor={difficulty_editor} />
                </Control>
            </Field>

            <Field>
                <Control>
                    <CheckboxInput<ToBool<FlagEditor>> editor={ngp_editor}>
                        {" "}<Text path="meta_ngp" />
                    </CheckboxInput<ToBool<FlagEditor>>>
                </Control>
            </Field>

            <Field>
                <Control>
                    <CheckboxInput<ToBool<FlagEditor>> editor={game_clear_editor}>
                        {" "}<Text path="meta_clear" />
                    </CheckboxInput<ToBool<FlagEditor>>>
                </Control>
            </Field>

            {(!dlc4).then(|| html! {
                <Field>
                    <Control>
                        <CheckboxInput<ToBool<FlagEditor>> editor={fr_complete_editor}>
                            {" "}<Text path="meta_fr_complete" />
                        </CheckboxInput<ToBool<FlagEditor>>>
                    </Control>
                </Field>
            })}
        </Tile>
    }
}

impl Editor for DifficultyEditor {
    type Target = Difficulty;

    fn get(&self, save: &recordkeeper::SaveData) -> Self::Target {
        Difficulty::from_repr(self.0.get(save)).expect("unknown difficulty")
    }

    fn set(&self, save: &mut recordkeeper::SaveData, new: Self::Target) {
        self.0.set(save, new as u32);
    }
}

impl ToHtml for Difficulty {
    fn to_html(&self) -> Html {
        let id = match self {
            Difficulty::Easy => "easy",
            Difficulty::Normal => "normal",
            Difficulty::Hard => "hard",
            Difficulty::VeryHard => "veryhard",
        };
        html!(<Text path={format!("difficulty_{id}")} />)
    }
}
