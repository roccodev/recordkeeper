use recordkeeper::SaveFlag;

use super::edit::editor;

pub mod env;
pub mod location;
pub mod player;

#[rustfmt::skip]
editor!(
    pub MetaFlagEditor,
    bool,
    get |editor, save| save.is_flag_set(editor.flag),
    set |editor, save, new| save.set_flag(editor.flag, new),
    capture flag: SaveFlag
);
