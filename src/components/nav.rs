use super::Page;
use yew::prelude::*;

pub struct Nav;
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub page: Page,
}

impl Component for Nav {
    type Message = ();
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
