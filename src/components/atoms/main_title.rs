use stylist::yew::styled_component;
use yew::{ html, Html, Properties};

#[derive(PartialEq, Properties)]
pub(crate) struct Props{
    pub(crate) title: String,

}

#[styled_component(MainTitle)]
pub(crate) fn main_title(props: &Props) -> Html {
    
    let style = r#"

        
        .ok{
            color: green;
                    background-color: darkgrey;
        color:black;
        font-size: 2rem;
        color: #333;
        text-align: center
        }
        .err{
            color: red;
        }"#;
    let stylesheet = stylist::Style::new(style).unwrap();
    html! {
    <div class={stylesheet}>
        <h1 class="ok">{&props.title}</h1>
    </div>
    }
}