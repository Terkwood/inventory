use crate::model::*;
use yew::prelude::*;

pub struct InventoryButtons {
    pub link: ComponentLink<Self>,
    pub props: Props,
    mode: Mode,
    name_input: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub inventory_buttons: InventoryButtonCollection,
    pub add_inventory_button: Callback<ItemType>,
}

pub enum Msg {
    EmojiSelected(String),
    AddButton(ItemType),
    InputUpdated(String),
    NothingHappened,
}

enum Mode {
    SelectEmoji,
    EnterName { emoji: String },
}

impl Component for InventoryButtons {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            mode: Mode::SelectEmoji,
            name_input: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::EmojiSelected(emoji) => {
                self.mode = Mode::EnterName { emoji };
                self.name_input.clear();
                true
            }
            Msg::AddButton(item_type) => {
                self.mode = Mode::SelectEmoji;
                self.name_input.clear();
                self.props.add_inventory_button.emit(item_type);
                true
            }
            Msg::InputUpdated(name_input) => {
                self.name_input = name_input;
                true
            }
            Msg::NothingHappened => false,
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
        match &self.mode {
            Mode::SelectEmoji => self.view_emoji_selection(),
            Mode::EnterName { emoji } => self.view_name_input(emoji.clone()),
        }
    }
}

impl InventoryButtons {
    fn view_emoji_selection(&self) -> Html {
        let free = self.props.inventory_buttons.free_user_buttons();
        html! {
            <div class="configsection">
                { self.view_intro() }
                <div> {
                    if free > 0 {
                        format!("You may add {} more buttons.", free)
                    } else {
                        "You need to delete a button before you can add another.".to_string()
                    }
                } </div>
                {
                    if free > 0 {
                        self.view_emoji_selection_buttons()
                    } else {
                        html! { <></> }
                    }
                }
            </div>
        }
    }

    fn view_name_input(&self, emoji: String) -> Html {
        let em_c = emoji.clone();
        let ni_c = self.name_input.clone();
        let on_enter_key = self.link.callback(move |e: web_sys::KeyboardEvent| {
            if e.key_code() == 13 {
                Msg::AddButton(ItemType {
                    emoji: em_c.clone(),
                    name: ni_c.clone(),
                })
            } else {
                Msg::NothingHappened
            }
        });
        let ni_c_c = self.name_input.clone();
        let on_click = self.link.callback(move |_| {
            Msg::AddButton(ItemType {
                emoji: emoji.clone(),
                name: ni_c_c.clone(),
            })
        });
        html! {
            <div class="configsection">
                { self.view_intro() }
                <div>{ "Input the name" }</div>
                <input
                    oninput={self.link.callback(|e: InputData| Msg::InputUpdated(e.value))}
                    onkeyup={on_enter_key.clone()}
                />
                <button onkeyup={on_enter_key} onclick={on_click}>{ "ADD" }</button>
            </div>
        }
    }

    fn view_emoji_selection_buttons(&self) -> Html {
        InventoryButtonCollection::allowed_emojis()
            .iter()
            .map(|emoji|{ let e = emoji.clone();
            html! {
                <button onclick={self.link.callback(move |_| Msg::EmojiSelected(e.clone()))}>{ emoji }</button>
            }})
            .collect::<Html>()
    }

    fn view_intro(&self) -> Html {
        html! {
            <>
                <h1>{ "Configure Inventory Buttons"}</h1>
                <h2>{ "Current buttons:" }</h2>
                <div> { self.props.inventory_buttons.all().iter().map(view_item_type).collect::<Html>() } </div>
                <h2>{ "Add a button" }</h2>
            </>
        }
    }
}

fn view_item_type(item_type: &ItemType) -> Html {
    let emoji = &item_type.emoji;
    let name = &item_type.name;
    html! { <div>{ format!("{} {}", emoji, name) }</div> }
}
