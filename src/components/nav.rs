use super::Page;
use yew::prelude::*;

pub struct Nav {
    pub link: ComponentLink<Self>,
    pub props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub page: Page,
}

impl Component for Nav {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
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
            <div id="nav">
                <div class="center">
                    <button>{ "prev" }</button>
                </div>
                <div class="center">
                    <button>{ "next" }</button>
                </div>
            </div>
        }
    }
}
