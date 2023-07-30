use recordkeeper_macros::SaveBin;

mod character;

#[derive(SaveBin)]
pub struct SaveData {

}

#[derive(SaveBin)]
pub struct PlayTime {
    raw: u32
}