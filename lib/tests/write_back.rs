use recordkeeper::SaveFile;

static SRC: &[u8] = include_bytes!("res/save-ch5-v10.sav");

#[test]
pub fn write_back_no_changes() {
    let mut save = SaveFile::from_bytes(SRC).unwrap();
    save.write().unwrap();
    assert_eq!(SRC, save.bytes());
}
