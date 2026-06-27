#![allow(clippy::collapsible_if, clippy::unnecessary_map_or)]

mod api;
mod app;
mod components;
mod footer;
mod header;
mod i18n;
mod storage;
mod types;
mod utils;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
