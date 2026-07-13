#![allow(clippy::collapsible_if, clippy::unnecessary_map_or)]

mod api;
mod app;
mod components;
mod i18n;
mod types;
mod whois_and_navigation_helpers;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
