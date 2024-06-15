use fxhash::FxHashMap;
use gloo::console::log;
use serde::Deserialize;
use stylist::yew::styled_component;
use yew::{html, Html};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub(crate) struct State {
    // Example stuff:
    pub(crate) label: String,

    pub(crate) value: f32,
    pub(crate) data: Daten,
    pub(crate) selected_lms: Vec<(String, u16)>,
    pub(crate) new_selection: Option<String>,
    pub(crate) view: View,
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

#[styled_component(AppView)]
pub(crate) fn app_view() -> Html {
    let data = load_data();
    let str = format!("{:?}", data);
    log!(&str);

    html! {
        <div>
            { 
                data.lebensmittel.iter().map(|(key, value)| {
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
        </div>
    }
}





#[derive(Clone, Copy, Debug, PartialEq)]
pub (crate) enum View {
    View1,
    View2,
    View3,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TagesBedarfInMg {
    pub(crate) name: String,
    pub(crate) wert: f64,
}

pub(crate) type LMInformation = FxHashMap<String, LMConcentration>;
pub(crate) type LM = FxHashMap<String, LMInformation>;
pub(crate) type LMConcentration = f64;

#[derive(Deserialize, Debug)]
pub(crate) struct Daten {
    pub(crate) lebensmittel: FxHashMap<String, LMInformation>,
    pub(crate) tagesbedarf: Vec<TagesBedarfInMg>,
}


fn fix_data(mut data: Daten) -> Daten{
    let key_value = data.lebensmittel.get("Möhregegartgegart").unwrap().clone();
    let (key, value) = key_value.iter().next().unwrap();
    let key = key.clone();
    data.lebensmittel.remove("Möhregegartgegart");
    data.lebensmittel.get_mut("Möhregegart").unwrap().insert(key.clone(), value.clone());
    return data;
}