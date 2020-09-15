use super::inventory_buttons::InventoryButtons;
use crate::model::*;
use crate::time::js_utc_now;
use yew::prelude::*;
use yew_export_button::*;

pub struct Config {
    pub link: ComponentLink<Self>,
    pub props: Props,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub inventory_buttons: InventoryButtonCollection,
    pub inventory: Inventory,
    pub add_inventory_button: Callback<ItemType>,
    pub del_inventory_button: Callback<ItemType>,
    pub show_nav: Callback<bool>,
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
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

impl Config {
    fn view_export(&self) -> Html {
        let button = export_button(
            &self.props.inventory,
            ButtonOpts {
                utc_millis: js_utc_now().0,
                a_class: "download",
                button_id: "exportbutton",
                file_prefix: "inventory",
            },
        );
        html! {
            <div class="configsection">
                <h1>{ "Export Data"}</h1>
                <p>{ "Use this button to save your data to a file on your device." }</p>
                <div>{ button }</div>
            </div>
        }
    }

    fn view_inventory_buttons(&self) -> Html {
        html! {
            <InventoryButtons
                inventory_buttons=self.props.inventory_buttons.clone()
                add_inventory_button=self.props.add_inventory_button.clone()
                del_inventory_button=self.props.del_inventory_button.clone()
                show_nav=self.props.show_nav.clone()
            />
        }
    }

    fn view_about(&self) -> Html {
        html! {
            <div class="configsection">
                <h1>{ "About" }</h1>
                <p>{ "INVENTORY helps you track Fourth and Tenth Step inventories used in Twelve Step programs." }</p>
                <p>{ "INVENTORY is designed with privacy in mind.  Your data will never be transmitted to a third party.  Data is kept in browser local storage, unencrypted.  KEEP YOUR DATA SAFE: make sure there is no malware on your system!" }</p>
                <p>{ format!("This is version {}.", VERSION) }</p>
                <h2>{ "Source Code" }</h2>
                <p>{ "The source code is available under MIT license." }</p>
                <p><a href=REPO_URL>{ REPO_URL }</a></p>
            </div>
        }
    }
}
