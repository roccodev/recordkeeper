use recordkeeper_macros::SaveBin;

const COLONY_MAX: usize = 32;

#[derive(SaveBin, Debug)]
pub struct ColonyFlameClocks {
    values: [u32; COLONY_MAX],
}
