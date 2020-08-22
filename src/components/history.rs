use crate::model::history::*;
use crate::model::*;

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
        let history = History::from(&props.inventory, todo!());
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
            self.history = History::from(&props.inventory, todo!());
            self.props = props;
            true
        } else {
            false
        }
    }
    fn view(&self) -> Html {
        todo!()
    }
}
