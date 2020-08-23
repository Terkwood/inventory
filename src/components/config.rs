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
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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
            <div id="config">
                { self.view_export() }
                { self.view_inventory_buttons() }
                { self.view_about() }
            </div>
        }
    }
}

impl Config {
    fn view_inventory_buttons(&self) -> Html {
        todo!()
    }
    fn view_about(&self) -> Html {
        todo!()
    }
    fn view_export(&self) -> Html {
        todo!()
    }
}
