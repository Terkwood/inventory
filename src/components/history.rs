use crate::model::history::*;
use crate::model::*;
use crate::time::js_local_offset;

use yew::prelude::*;

pub struct HistoryView {
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub history: History,
}
#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub resolve_item: Callback<u64>,
    pub inventory: Inventory,
}
pub enum Msg {
    Resolve(Item),
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
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Resolve(item) => self.props.resolve_item.emit(item.epoch_millis_utc),
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
                    .map(view_day)
                    .collect::<Html>()
            }
            </div>
        }
    }
}
fn view_day(day: &Day) -> Html {
    html! { <div>{ "a day "}</div> }
}
