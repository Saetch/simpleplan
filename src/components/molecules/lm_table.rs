use _LmTableProps::ingreds;
use gloo::console::log;
use stylist::{yew::styled_component, Style};
use yew::{html, Callback, Html, InputEvent, Properties, TargetCast};
use web_sys::HtmlInputElement;

use crate::components::organisms::appview::{Daten, LMInformation, TagesBedarfInMg};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct LmTableProps{
    pub selected_lms: Vec<(String, u16)>,
    pub ingreds: Vec<TagesBedarfInMg>,
    pub on_table_update: Callback<TableUpdate>,
    pub data: Daten,
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
                                min={0} max={800} step={5}
                                {oninput}
                            />
                            </td>
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
            <th>{"Tagesbedarf in mg"}</th>
            <th>{"% des Tagesbedarfs"}</th>
        </tr>
    </thead>
    <tbody>
        {for props.ingreds.iter().map(|tagesbedarf| {
            let table_callback_it = props.on_table_update.clone();
            let name_clone = tagesbedarf.name.clone();
            
            let oninput = table_callback_it.reform(move |event: InputEvent| {
                let input: HtmlInputElement = event.target_unchecked_into();
                TableUpdate::Update(name_clone.clone(), input.value_as_number() as u16)
            });
            let mut menge = 0.0;
            let mut per_mg = 0.0;
            let mut curr_lm = "";
            let bedarf_in_mg = tagesbedarf.wert.clone();
            for (lm, info) in props.data.lebensmittel.iter(){
                for (ingred, menge_ingred) in info.iter(){
                    if ingred == &tagesbedarf.name{
                        per_mg = menge_ingred.clone();
                        curr_lm = lm;
                    }
                }
                if per_mg == 0.0{
                    continue;
                }
                for (lm, menge_lm) in props.selected_lms.iter(){
                    if curr_lm == lm{
                        menge += *menge_lm as f64 * per_mg;
                    }
                }
            }
            menge = menge * 1000.0;
            menge = menge.round();
            menge = menge / 1000.0;
            html!{
                        <tr>
                            <td>{tagesbedarf.name.clone()}</td>
                            <td>{menge}</td>
                            <td>{bedarf_in_mg}</td>
                            <td>{if menge > 0.0 { (menge / bedarf_in_mg) * 100.0 } else { 0.0 }} </td>
                        </tr>
                    
            }
        }
    )}
    </tbody>
    </table>
    </div>
    }
}
