use yew::Html;

mod app;
pub mod components;
mod data;
pub mod dialog;
pub mod lang;
pub mod routes;
pub mod save;

pub const BRAND_NAME: &str = "Recordkeeper";
pub const BRAND_DISPLAY: &str = concat!("Recordkeeper", " v. ", env!("CARGO_PKG_VERSION"));

pub const GITHUB_URL: &str = "https://github.com/RoccoDev/recordkeeper";
pub const LICENSE_URL: &str = "https://github.com/RoccoDev/recordkeeper/blob/master/COPYING";

pub trait ToHtml {
    fn to_html(&self) -> Html;
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    app::render_app();
}
