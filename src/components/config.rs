use crate::model::*;
use yew::prelude::*;

pub struct Config {
    pub link: ComponentLink<Self>,
    pub props: Props,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub inventory_buttons: InventoryButtonCollection,
}

impl Component for Config {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }
    fn view(&self) -> Html {
        html! {
            <div>
                { self.view_export() }
                { self.view_inventory_buttons() }
                { self.view_about() }
            </div>
        }
    }
}

const REPO_URL: &str = "https://github.com/Terkwood/inventory";
impl Config {
    fn view_inventory_buttons(&self) -> Html {
        html! {
            <div class="configsection">
                <h1>{ "Configure Inventory Buttons"}</h1>
                <div>{ "🏗 Coming Soon 🏗" }</div>
                <p>{ "Your current inventory buttons:" }</p>
                <div> { self.props.inventory_buttons.all().iter().map(view_item_type).collect::<Html>() } </div>
            </div>
        }
    }
    fn view_about(&self) -> Html {
        html! {
            <div class="configsection">
                <h1>{ "About" }</h1>
                <p>{ "INVENTORY helps you track Fourth and Tenth Step inventories used in Twelve Step programs." }</p>
                <p>{ "INVENTORY is designed with privacy in mind.  Your data will never be transmitted to a third party.  Data is kept in browser local storage, unencypted.  KEEP YOUR DATA SAFE: make sure there is no malware on your system!" }</p>
                <h2>{ "Source Code" }</h2>
                <p>{ "The source code is available under MIT license." }</p>
                <p><a href=REPO_URL>{ REPO_URL }</a></p>
            </div>
        }
    }
    fn view_export(&self) -> Html {
        html! {
            <div class="configsection">
                <h1>{ "Export Data"}</h1>
                <div>{ "🏗 Coming Soon 🏗" }</div>
            </div>
        }
    }
}

fn view_item_type(item_type: &ItemType) -> Html {
    let emoji = &item_type.emoji;
    let name = &item_type.name;
    html! { <div>{ format!("{} {}", emoji, name) }</div> }
}
