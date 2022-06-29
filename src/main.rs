mod app;
mod components;
mod functions;
mod models;
mod pages;

use app::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
