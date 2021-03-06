use crate::model::history::*;
use crate::model::*;
use crate::time::js_local_offset;

use yew::prelude::*;

pub struct HistoryView {
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub history: History,
    pub mode: Mode,
}
#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub resolve_item: Callback<UtcMillis>,
    pub inventory: Inventory,
}
pub enum Msg {
    Resolve(UtcMillis),
    EnterResolveMode(UtcMillis),
}

#[derive(PartialEq)]
pub enum Mode {
    View,
    Resolve(UtcMillis),
}

const EMPTY_MSG: &str = "This is the history of all your entries. You haven't written anything down yet, so there's nothing to show.";
impl Component for HistoryView {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let history = History::from(&props.inventory, js_local_offset());
        Self {
            props,
            link,
            history,
            mode: Mode::View,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Resolve(epoch) => {
                self.props.resolve_item.emit(epoch);
                false
            }
            Msg::EnterResolveMode(epoch) => {
                self.mode = Mode::Resolve(epoch);
                true
            }
        }
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.history = History::from(&props.inventory, js_local_offset());
            self.props = props;
            true
        } else {
            false
        }
    }
    fn view(&self) -> Html {
        let payload = &self.history.days;
        if payload.is_empty() {
            html! { <div id="history"> <p> { EMPTY_MSG } </p> </div> }
        } else {
            html! {
                <div id="history">
                {
                    payload.iter()
                    .map(|day| self.view_day(day))
                    .collect::<Html>()
                }
                </div>
            }
        }
    }
}

impl HistoryView {
    fn view_day(&self, day: &Day) -> Html {
        html! {
            <div>
                <h1>{ day.date.format("%A, %b %e, %Y") }</h1>
                <ul>{ day.inventory.items.iter().map(|item| self.view_item(item)).collect::<Html>() }</ul>
            </div>
        }
    }

    fn view_item(&self, item: &Item) -> Html {
        let epoch = UtcMillis(item.epoch_millis_utc);
        html! {
            <li class="inventoryitem" onclick={self.link.callback(move |_| Msg::EnterResolveMode(epoch))}>
                { format!("{} {} " , item.item_type.emoji, item.text) }
                {
                    if self.mode == Mode::Resolve(epoch) {
                        html! {
                            <button
                                class="resolve"
                                onclick=
                                    self.link
                                        .callback(
                                            move |_| Msg::Resolve(epoch)
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
