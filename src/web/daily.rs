use crate::inventory::Item;
use yew::prelude::*;

pub struct Daily {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
    AddItem(Item),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub items: Vec<Item>,
    pub add_item: Callback<Item>,
}

impl Component for Daily {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddItem(item) => self.props.add_item.emit(item),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>

            </div>
        }
    }
}
