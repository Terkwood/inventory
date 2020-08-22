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
            Msg::Resolve(epoch) => self.props.resolve_item.emit(epoch),
            Msg::EnterResolveMode(epoch) => self.mode = Mode::Resolve(epoch),
        }

        false
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
        let style = format!(
            "grid-template: repeat({}, 1fr) / repeat(1, 1fr);",
            self.history.days.len()
        );

        html! {
            <div id="history" style=style>
            {
                self.history.days.iter()
                    .map(|day|self.view_day(day))
                    .collect::<Html>()
            }
            </div>
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
