use recordkeeper::{DataFile, SystemFile};

static SRC: &[u8] = include_bytes!("res/system.sav");

#[test]
pub fn auto_detect() {
    assert!(DataFile::from_bytes(SRC).unwrap().is_system());
}

#[test]
pub fn write_back_no_changes() {
    let mut save = SystemFile::from_bytes(SRC).unwrap();
    save.write().unwrap();
    assert_eq!(SRC, save.bytes());
}
