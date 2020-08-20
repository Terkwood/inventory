use crate::inventory::*;
use yew::prelude::*;

pub struct Daily {
    link: ComponentLink<Self>,
    props: Props,
    text_area: String,
    mode: Mode,
}

pub enum Msg {
    SubmitItem(ItemType),
    TextAreaUpdated(String),
    HideInventory,
    ShowInventory,
    ToggleResolveMode,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub inventory: Inventory,
    pub add_item: Callback<Item>,
}

#[derive(PartialEq)]
pub enum Mode {
    Default,
    Resolve,
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
            mode: Mode::Default,
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
            Msg::HideInventory => (),
            Msg::ShowInventory => (),
            Msg::ToggleResolveMode => {
                if self.mode == Mode::Resolve {
                    self.mode = Mode::Default
                } else {
                    self.mode = Mode::Resolve
                }
            }
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
            <>
                { self.view_input() }
                { self.view_todays_inventory() }
            </>
        }
    }
}

impl Daily {
    pub fn view_input(&self) -> Html {
        html! {
            <div id="inputgrid">
                <div id="bigtextgrid">
                    <textarea
                        value=&self.text_area
                        onfocus=self.link.callback(|_| Msg::HideInventory)
                        onchange=self.link.callback(|_| Msg::ShowInventory)
                        oninput=self.link.callback(|e: InputData| Msg::TextAreaUpdated(e.value))
                        placeholder="Please take inventory.">
                    </textarea>
                </div>
                <div class="center">
                    <button
                        class="itembutton"
                        onclick=
                            self.link
                                .callback(
                                    |_| Msg::SubmitItem(
                                        DefaultItemType::Resentment.instance()
                                    ))>
                        { "Resentment 😠" }
                    </button>
                </div>
                <div class="center">
                    <button
                        class="itembutton"
                        onclick=
                            self.link
                                .callback(
                                    |_| Msg::SubmitItem(
                                        DefaultItemType::Fear.instance()
                                    ))>
                        { "Fear 😱" }
                    </button>
                </div>
                <div class="center">
                    <button
                        class="itembutton"
                        onclick=
                            self.link
                                .callback(
                                    |_| Msg::ToggleResolveMode
                                )>
                        { "Resolve ✅"}
                    </button>
                </div>
            </div>
        }
    }
    pub fn view_todays_inventory(&self) -> Html {
        html! {
            <div id="inventory">
                { if self.mode == Mode::Resolve { html!{<>{"RESOLVE"}</>} } else { html!{<></>} }}
                <ul>
                    { self.props.inventory.items.iter().map(view_item).collect::<Html>() }
                </ul>
            </div>
        }
    }
}
fn view_item(item: &Item) -> Html {
    html! {
        <li class="inventoryitem">
            { format!("{} {}" , item.item_type.emoji, item.text) }
        </li>
    }
}
