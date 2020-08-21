use super::Page;
use yew::prelude::*;

pub struct Nav {
    pub link: ComponentLink<Self>,
    pub props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub page: Page,
    pub nav_to: Callback<Page>,
}

#[derive(Copy, Clone)]
pub struct NavMsg(Page);

impl Component for Nav {
    type Message = NavMsg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.props.nav_to.emit(msg.0);
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
        let page = self.props.page;
        let prev = page.prev();
        let next = page.next();
        html! {
            <div id="nav">
                <div class="center">
                    <button
                        class="bigbutton"
                        onclick={
                            self.link
                                .callback(
                                    move |_| NavMsg(prev))}>
                        { prev }
                    </button>
                </div>
                <div class="center">
                <button
                    class="bigbutton"
                    onclick={
                        self.link
                            .callback(
                                move |_| NavMsg(next))}>
                        { next }
                    </button>
                </div>
            </div>
        }
    }
}
