use super::daily::Daily;
use super::nav::Nav;
use super::Page;
use crate::inventory::*;
use crate::repo::Repo;
use yew::prelude::*;

pub struct App {
    _link: ComponentLink<Self>,
    page: Page,
    repo: Repo,
    inventory: Inventory,
    add_item: Option<Callback<Item>>,
    resolve_item: Option<Callback<u64>>,
}

pub enum Msg {
    AddItem(Item),
    ResolveItem(u64),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let add_item = Some(link.callback(|item| Msg::AddItem(item)));
        let resolve_item =
            Some(link.callback(|epoch_millis_utc| Msg::ResolveItem(epoch_millis_utc)));
        let repo = Repo::new();
        let inventory = repo.read_inventory();
        let mode = Page::default();

        Self {
            _link: link,
            add_item,
            resolve_item,
            page: mode,
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
            Msg::ResolveItem(utc) => {
                self.inventory.resolve(utc);
                self.repo.save_inventory(&self.inventory)
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            {
                match self.page {
                    Page::Daily => self.view_daily(),
                    Page::History => self.view_history(),
                    Page::Config => self.view_config(),
                }
            }
            { self.view_nav() }
            </>
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

    fn view_history(&self) -> Html {
        todo!()
    }

    fn view_config(&self) -> Html {
        todo!()
    }

    fn view_nav(&self) -> Html {
        html! {
            <Nav page={self.page}/>
        }
    }
}
