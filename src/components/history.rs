use crate::model::history::*;
use crate::model::*;

use yew::prelude::*;

pub struct HistoryView {
    link: ComponentLink<Self>,
}
#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    resolve_item: Callback<Item>,
}
pub struct Msg;

impl Component for HistoryView {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        todo!()
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        todo!()
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }
    fn view(&self) -> Html {
        todo!()
    }
}
