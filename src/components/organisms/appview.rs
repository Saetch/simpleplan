use _State::selected_lms;
use fxhash::FxHashMap;
use gloo::console::log;
use serde::Deserialize;
use stylist::{yew::styled_component, Style};
use yew::{html, use_state, Callback, Html, MouseEvent, Properties, UseStateHandle};
use crate::components::molecules::lm_table::{LmTable, TableUpdate, _LmTableProps::on_table_update};

#[derive(Debug, Clone, Properties, PartialEq)]
pub(crate) struct State {
    pub(crate) data: Daten,
    pub selected_lms: Vec<(String, u16)>,
}



fn load_data() -> Daten{
    
    let bytes = include_bytes!("../../ressources/daten.json");
    let data: Daten = serde_json::from_slice(bytes).unwrap();
    for (key, value) in data.lebensmittel.iter(){
        println!("\n{}: ", key);
        for (key, value) in value.iter(){
            println!("{}: {}", key, value);
        }
    }
    for tagesbedarf in data.tagesbedarf.iter(){
        println!("{}: {}", tagesbedarf.name, tagesbedarf.wert);
    }
    
    fix_data(data)
}

impl State {
    pub(crate) fn initial() -> Self {
        Self {
            data: load_data(),
            selected_lms: Vec::new(),
        }
    }
}

#[styled_component(AppView)]
pub(crate) fn app_view() -> Html {
    let view = use_state(|| View::View1);
    let mut state = use_state(|| State::initial());
    let str = use_state(||String::from("Inhalt darstellen")).clone();
    let onclick: Callback<MouseEvent> = {
        let view = view.clone();
        let str = str.clone();
        Callback::from(move |_| {
            let new_view = match *view {
                View::View1 => View::View2,
                View::View2 => View::View1,
                View::View3 => View::View1,
            };
            let new_str = if *str == "Inhalt darstellen"{
                log!("Inhalt darstellen wurde geändert");
                "Zum Planen wechseln"
            }else{
                log!("Zum Planen wechseln wurde geändert");
                "Inhalt darstellen"
            };
            str.set(new_str.to_string());
            view.set(new_view);
        })
    };
    log!("Add Lebensmittel!");
    let mut lms = state.selected_lms.clone();
    if lms.is_empty(){
        for (key, _) in state.data.lebensmittel.iter(){
            lms.push((key.clone(), 0));
        }
        state.set ( State { selected_lms: lms, ..(*state).clone() });
    }
    let lms = state.selected_lms.clone();
    let state_clone = state.clone();
    let ingreds = state.data.tagesbedarf.clone();
    let table_update_callback = Callback::from(move |table_update: TableUpdate| {
        match table_update {
            TableUpdate::Update(name, menge) => {
                let state = state_clone.clone();
                let mut lms = state.selected_lms.clone();
                let index = lms.iter().position(|(lm_name, _)| lm_name == &name);
                if let Some(index) = index {
                    lms[index] = (name, menge);
                    state.set(State { selected_lms: lms, ..(*state).clone() });
                }
            }
            TableUpdate::Remove(name) => {
                let state = state_clone.clone();
                let mut lms = state.selected_lms.clone();
                let index = lms.iter().position(|(lm_name, _)| lm_name == &name);
                if let Some(index) = index {
                    lms.remove(index);
                    state.set(State { selected_lms: lms, ..(*state).clone() })
                }
            }
        }
    });

    html! {
        <div>
            <button {onclick}>{ (*str).clone()  }</button>
            { 
                if *view == View::View1 {
                    let bool = state.selected_lms.is_empty();
                    if bool{
                        html!{
                            <div>
                                <p>{"Keine Lebensmittel ausgewählt"}</p>
                            </div>
                        }
                    }else{
                        html!{
                            <LmTable selected_lms={state.selected_lms.clone()} on_table_update={table_update_callback} ingreds={ingreds} data={state.data.clone()}/>
                        }

                    }
                }
                else {

                    state.data.lebensmittel.iter().map(|(key, value)| {
                        html! {
                            <div>
                                <p>{key}</p>
                                <p>
                                {
                                    format!("{:?}", value)
                                }
                                </p>
                            </div>
                        }
                    }).collect::<Html>()
                }
            }
        </div>
    }
}





#[derive(Clone, Copy, Debug, PartialEq)]
pub (crate) enum View {
    View1,
    View2,
    View3,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub(crate) struct TagesBedarfInMg {
    pub(crate) name: String,
    pub(crate) wert: f64,
}

pub(crate) type LMInformation = FxHashMap<String, LMConcentration>;
pub(crate) type LM = FxHashMap<String, LMInformation>;
pub(crate) type LMConcentration = f64;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct Daten {
    pub(crate) lebensmittel: FxHashMap<String, LMInformation>,
    pub(crate) tagesbedarf: Vec<TagesBedarfInMg>,
}


fn fix_data(mut data: Daten) -> Daten{
    let key_value = data.lebensmittel.get("Möhregegartgegart").unwrap().clone();
    let (key, value) = key_value.iter().next().unwrap();
    let key = key.clone();
    data.lebensmittel.remove("Möhregegartgegart");
    data.lebensmittel.get_mut("Möhregegart").unwrap().insert(key, *value);



    let special_words = ["gebraten", "gekocht", "gegart", "gegarts"];
    let mut vec_to_update = Vec::new();
    for (key, _) in data.lebensmittel.iter(){
        for word in special_words.iter(){
            if key.ends_with(word){
                vec_to_update.push(key.clone());
            }
        }
    }
    
    for to_update_key in vec_to_update.iter(){
        let mut splits = to_update_key.split("ge").map(|s| s.to_string()).collect::<Vec<String>>();
        for split in splits[1..].iter_mut(){
            split.insert_str(0, "ge");
        }
        splits.insert(splits.len()-1, " ".to_owned());
        let new_key = splits.join("");
        let hash_map = data.lebensmittel.get(to_update_key).unwrap().clone();
        data.lebensmittel.remove(to_update_key);
        data.lebensmittel.insert(new_key, hash_map);

    }

    let fixable_key = "Vollkornrei gegarts";
    let key_value = data.lebensmittel.get(fixable_key).unwrap().clone();
    data.lebensmittel.remove(fixable_key);
    let mut lm = data.lebensmittel.get_mut("Vollkornreis gegart");
    for (key, value) in key_value.iter(){
        lm.as_mut().unwrap().insert(key.clone(), *value);
    }

    let kkkey= "geröstetegesalzeneKürbiskerne";
    let key_value = data.lebensmittel.get(kkkey).unwrap().clone();
    data.lebensmittel.remove(kkkey);
    data.lebensmittel.insert("geröstete gesalzene Kürbiskerne".to_owned(), key_value);


    let bkey = "Butter";
    let key_value = data.lebensmittel.get(bkey).unwrap().clone();
    data.lebensmittel.remove(bkey);
    let mut lm = data.lebensmittel.get_mut("Süßrahmbutter");
    for (key, value) in key_value.iter(){
        lm.as_mut().unwrap().insert(key.clone(), *value);
    }

    return data;
}