use recordkeeper_macros::SaveBin;

/// Nul-terminated string with fixed storage and maximum length.
///
/// Extra bytes are not guaranteed to be nulls.
#[derive(SaveBin, Debug)]
pub struct FixStr<const MAX: usize> {
    buf: [u8; MAX],
}
