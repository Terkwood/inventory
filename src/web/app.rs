use super::daily::Daily;
use crate::inventory::*;
use crate::repo::Repo;
use yew::prelude::*;

pub struct App {
    link: ComponentLink<Self>,
    mode: Mode,
    repo: Repo,
    inventory: Inventory,
    add_item: Option<Callback<Item>>,
}

pub enum Msg {
    AddItem(Item),
}

enum Mode {
    Daily,
}
impl Default for Mode {
    fn default() -> Self {
        Mode::Daily
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let add_item = Some(link.callback(|item| Msg::AddItem(item)));
        let repo = Repo::new();
        let inventory = repo.read_inventory().expect("read inventory");
        let mode = Mode::default();

        Self {
            link,
            add_item,
            mode,
            repo,
            inventory,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        match self.mode {
            Mode::Daily => self.view_daily(),
        }
    }
}

impl App {
    fn view_daily(&self) -> Html {
        html! {
            <Daily inventory={self.inventory.clone()} add_item={self.add_item.as_ref().expect("add item cb")} />
        }
    }
}
