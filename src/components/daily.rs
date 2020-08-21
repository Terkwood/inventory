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
    FocusInput,
    EnterResolveMode(u64),
    Resolve(u64),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub inventory: Inventory,
    pub add_item: Callback<Item>,
    pub resolve_item: Callback<u64>,
}

#[derive(PartialEq)]
pub enum Mode {
    Default,
    Resolve(u64),
    Input,
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
                self.mode = Mode::Default;
                if !self.text_area.is_empty() {
                    self.props
                        .add_item
                        .emit(Item::new(item_type, self.text_area.clone()));
                    self.text_area.clear();

                    false
                } else {
                    // just get out of the input area
                    true
                }
            }
            Msg::TextAreaUpdated(text) => {
                self.text_area = text;
                true
            }
            Msg::FocusInput => {
                self.mode = Mode::Input;
                true
            }
            Msg::EnterResolveMode(item_epoch_ms_utc) => {
                if self.mode == Mode::Resolve(item_epoch_ms_utc) {
                    self.mode = Mode::Default
                } else {
                    self.mode = Mode::Resolve(item_epoch_ms_utc)
                }

                true
            }
            Msg::Resolve(item_epoch_millis_utc) => {
                self.props.resolve_item.emit(item_epoch_millis_utc);
                false
            }
        }
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
        let input_grid_id = if self.mode == Mode::Input {
            "inputgridfocus"
        } else {
            "inputgridwaiting"
        };
        html! {
            <div id=input_grid_id>
                <div id="bigtextgrid">
                    <textarea
                        value=&self.text_area
                        onfocus=self.link.callback(|_| Msg::FocusInput)
                        oninput=self.link.callback(|e: InputData| Msg::TextAreaUpdated(e.value))
                        placeholder="Please take inventory.">
                    </textarea>
                </div>
                <div class="center">
                    <button
                        class="bigbutton"
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
                        class="bigbutton"
                        onclick=
                            self.link
                                .callback(
                                    |_| Msg::SubmitItem(
                                        DefaultItemType::Fear.instance()
                                    ))>
                        { "Fear 😱" }
                    </button>
                </div>
            </div>
        }
    }
    fn view_todays_inventory(&self) -> Html {
        html! {
            <div id="inventory">
                <ul>
                    { self.props.inventory.items.iter().map(|item| self.view_item(item.clone())).collect::<Html>() }
                </ul>
            </div>
        }
    }
    fn view_item(&self, item: Item) -> Html {
        let epoch_ms = item.epoch_millis_utc;
        html! {
            <li class="inventoryitem" onclick={self.link.callback(move |_| Msg::EnterResolveMode(epoch_ms))}>
                { format!("{} {} " , item.item_type.emoji, item.text) }
                {
                    if self.mode == Mode::Resolve(item.epoch_millis_utc) {
                        html! {
                            <button
                                class="resolve"
                                onclick=
                                    self.link
                                        .callback(
                                            move |_| Msg::Resolve(item.epoch_millis_utc)
                                        )>
                                { "✅"}
                            </button>
                        }
                    } else {
                        html! { <></> }
                    }
                }
            </li>
        }
    }
}
