use fxhash::FxHashMap;
use gloo::console::log;
use serde::Deserialize;
use stylist::yew::styled_component;
use yew::{html, use_state, Html};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub(crate) struct State {
    pub(crate) data: Daten,
    pub(crate) selected_lms: Vec<(String, u16)>,
    pub(crate) view: View,
    pub(crate) viewing_contents : bool
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
    //let state = use_state(|| State::initial());
    let data = load_data();
    let str = format!("{:?}", data);
    log!(&str);

    html! {
        <div>
            { 
                if true {
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
                else {
                    html! {
                        <p>{"View 2"}</p>
                    }
                
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