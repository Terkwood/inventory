use crate::inventory::*;
use yew::prelude::*;

pub struct Daily {
    link: ComponentLink<Self>,
    props: Props,
    text_area: String,
}

pub enum Msg {
    SubmitItem(ItemType),
    TextAreaUpdated(String),
    HideInventory,
    ShowInventory,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub inventory: Inventory,
    pub add_item: Callback<Item>,
}

impl Component for Daily {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let text_area = String::new();
        Self {
            link,
            props,
            text_area,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SubmitItem(item_type) => {
                self.props
                    .add_item
                    .emit(Item::new(item_type, self.text_area.clone()));
                self.text_area.clear()
            }
            Msg::TextAreaUpdated(text) => self.text_area = text,
            Msg::HideInventory => todo!(),
            Msg::ShowInventory => todo!(),
        }
        true
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
            <div id="controlgrid">
                <div id="bigtextgrid">
                    <textarea
                        rows=6
                        value=&self.text_area
                        onfocus=self.link.callback(|_| Msg::HideInventory)
                        onchange=self.link.callback(|_| Msg::ShowInventory)
                        oninput=self.link.callback(|e: InputData| Msg::TextAreaUpdated(e.value))
                        placeholder="Please take inventory.">
                    </textarea>
                </div>
                <div class="center">
                    <button
                        class="expandheight"
                        onclick=self.link
                                    .callback(|_| Msg::SubmitItem(DefaultItemType::Resentment.instance()))>
                        { "Resentment 😠" }
                    </button>
                </div>
                <div class="center">
                    <button class="expandheight" onclick=self.link.callback(|_| Msg::SubmitItem(DefaultItemType::Fear.instance()))>{ "Fear 😱" }</button>
                </div>
            </div>
        }
    }
}
