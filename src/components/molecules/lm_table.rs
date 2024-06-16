use gloo::console::log;
use stylist::{yew::styled_component, Style};
use yew::{html, Callback, Html, InputEvent, Properties, TargetCast};
use web_sys::HtmlInputElement;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct LmTableProps{
    pub selected_lms: Vec<(String, u16)>,
    pub on_table_update: Callback<TableUpdate>,
}

pub enum TableUpdate{
    Update(String, u16),
    Remove(String),
}



#[styled_component(LmTable)]
pub fn view(props: &LmTableProps) -> Html {
    let table_style = include_str!("../../styles/table.css");
    let style = Style::new(table_style).unwrap();

    html! {
        <div class={style}>
    <table class="styled-table">
    <thead>
        <tr>
            <th>{"Lebensmittel"}</th>
            <th>{"Menge in g"}</th>
            <th></th>
            <th></th>
        </tr>
    </thead>
    <tbody>
        {for props.selected_lms.iter().map(|(name, menge)| {
            let table_callback_it = props.on_table_update.clone();
            let name_clone = name.clone();
            
            let oninput = table_callback_it.reform(move |event: InputEvent| {
                let input: HtmlInputElement = event.target_unchecked_into();
                TableUpdate::Update(name_clone.clone(), input.value_as_number() as u16)
            });
            let table_callback_it = props.on_table_update.clone();
            let name_clone = name.clone();
            html!{
                        <tr>
                            <td>{name}</td>
                            <td>{menge}</td>
                            <td>
                                <input type="range"
                                value={menge.to_string()}
                                class="slider__input"
                                min={0} max={300} step={5}
                                {oninput}
                            />
                            </td>
                            <td><button onclick={
                                Callback::from(move |_| {
                                    table_callback_it.emit(TableUpdate::Remove(name_clone.clone()));
                                })
                            }>{"Entfernen"}</button></td>
                        </tr>
                    
            }
        }
    )}
    </tbody>
    </table>
    <table class="styled-table">
    <thead>
        <tr>
            <th>{"Inhalt"}</th>
            <th>{"Menge in mg"}</th>
            <th>{"% des Tagesbedarfs"}</th>
        </tr>
    </thead>
    </table>
    </div>
    }
}
