use super::daily::Daily;
use super::nav::Nav;
use super::Page;
use crate::inventory::*;
use crate::repo::Repo;
use yew::prelude::*;

pub struct App {
    page: Page,
    repo: Repo,
    inventory: Inventory,
    add_item: Option<Callback<Item>>,
    resolve_item: Option<Callback<u64>>,
    navigate_to: Option<Callback<Page>>,
}

pub enum Msg {
    AddItem(Item),
    ResolveItem(u64),
    NavigateTo(Page),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let add_item = Some(link.callback(|item| Msg::AddItem(item)));
        let resolve_item =
            Some(link.callback(|epoch_millis_utc| Msg::ResolveItem(epoch_millis_utc)));
        let navigate_to = Some(link.callback(|page| Msg::NavigateTo(page)));

        let repo = Repo::new();
        let inventory = repo.read_inventory();
        let mode = Page::default();

        Self {
            page: mode,
            repo,
            inventory,
            add_item,
            resolve_item,
            navigate_to,
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
            Msg::NavigateTo(page) => self.page = page,
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
        html! {<div>{ " HELLO HISTORY " }</div>}
    }

    fn view_config(&self) -> Html {
        html! {<div>{ "🚧 Coming soon: config 🚧" }</div>}
    }

    fn view_nav(&self) -> Html {
        html! {
            <Nav
                page={self.page}
                nav_to={self.navigate_to.as_ref().expect("nav cb")}
            />
        }
    }
}
