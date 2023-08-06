mod app;
pub mod components;

pub const BRAND_NAME: &str = "Recordkeeper";
pub const BRAND_DISPLAY: &str = concat!("Recordkeeper", " v. ", env!("CARGO_PKG_VERSION"));

fn main() {
    app::render_app();
}
