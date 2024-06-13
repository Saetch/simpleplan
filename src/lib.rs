use stylist::{Style};
use yew::{function_component, html, Html};
use gloo::console::log;
mod components;
use components::atoms::main_title::MainTitle;
const STYLE_FILE: &str = include_str!("styles/main.css");




#[function_component(App)]
pub fn app() -> Html {
    log!(String::from_utf8(Vec::from(STYLE_FILE)).unwrap());
    let stri = STYLE_FILE;
    log!("Hello, World!");
    html! {
    <>
        <div>
            <MainTitle title="Planen oder Anschauen"/>
        </div>
    </>

    }
}