use super::daily::Daily;
use crate::inventory::*;
use crate::repo::Repo;
use yew::prelude::*;

pub struct App {
    _link: ComponentLink<Self>,
    mode: Mode,
    repo: Repo,
    inventory: Inventory,
    add_item: Option<Callback<Item>>,
    resolve_item: Option<Callback<Item>>,
}

pub enum Msg {
    AddItem(Item),
    ResolveItem(Item),
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
        let resolve_item = Some(link.callback(|item| Msg::ResolveItem(item)));
        let repo = Repo::new();
        let inventory = repo.read_inventory();
        let mode = Mode::default();

        Self {
            _link: link,
            add_item,
            resolve_item,
            mode,
            repo,
            inventory,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddItem(item) => {
                self.inventory.add(item);
                self.repo.save_inventory(&self.inventory)
            }
            Msg::ResolveItem(item) => {
                self.inventory.resolve(item);
                self.repo.save_inventory(&self.inventory)
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
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
            <Daily
                inventory={self.inventory.today()}
                add_item={self.add_item.as_ref().expect("add item cb")}
                resolve_item={self.resolve_item.as_ref().expect("resolve item cb")}
            />
        }
    }
}
