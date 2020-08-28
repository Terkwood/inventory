use super::*;
use crate::model::*;
use crate::repo::Repo;
use crate::time::{js_local_offset, js_utc_now};
use yew::prelude::*;

pub struct App {
    page: Page,
    repo: Repo,
    inventory: Inventory,
    nav_state: NavState,
    add_item: Option<Callback<Item>>,
    resolve_item: Option<Callback<UtcMillis>>,
    nav_to: Option<Callback<Page>>,
    hide_nav: Option<Callback<()>>,
    show_nav: Option<Callback<()>>,
    add_inventory_button: Option<Callback<ItemType>>,
}

#[derive(PartialEq)]
pub enum NavState {
    Visible,
    Hidden,
}
pub enum Msg {
    AddItem(Item),
    ResolveItem(UtcMillis),
    NavigateTo(Page),
    HideNav,
    ShowNav,
    AddInventoryButton(ItemType),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let add_item = Some(link.callback(|item| Msg::AddItem(item)));
        let resolve_item =
            Some(link.callback(|epoch_millis_utc| Msg::ResolveItem(epoch_millis_utc)));
        let nav_to = Some(link.callback(|page| Msg::NavigateTo(page)));
        let hide_nav = Some(link.callback(|_| Msg::HideNav));
        let show_nav = Some(link.callback(|_| Msg::ShowNav));
        let add_inventory_button =
            Some(link.callback(|item_type| Msg::AddInventoryButton(item_type)));

        let repo = Repo::new();
        let inventory = repo.read_inventory();
        let page = Page::default();

        Self {
            page,
            repo,
            inventory,
            add_item,
            resolve_item,
            nav_state: NavState::Visible,
            nav_to,
            hide_nav,
            show_nav,
            add_inventory_button,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddItem(item) => {
                self.inventory.add(item);
                self.repo.save_inventory(&self.inventory)
            }
            Msg::ResolveItem(utc) => {
                self.inventory.resolve(utc.0);
                self.repo.save_inventory(&self.inventory)
            }
            Msg::NavigateTo(page) => self.page = page,
            Msg::HideNav => self.nav_state = NavState::Hidden,
            Msg::ShowNav => self.nav_state = NavState::Visible,
            Msg::AddInventoryButton(_item_type) => todo!(),
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
                inventory={self.inventory.today(js_utc_now(), js_local_offset())}
                add_item={self.add_item.as_ref().expect("add item cb")}
                resolve_item={self.resolve_item.as_ref().expect("resolve item cb")}
                hide_nav={self.hide_nav.as_ref().expect("hide nav cb")}
                show_nav={self.show_nav.as_ref().expect("show nav cb")}
            />
        }
    }

    fn view_history(&self) -> Html {
        html! {
            <HistoryView
                resolve_item={self.resolve_item.as_ref().expect("resolve item cb")}
                inventory={self.inventory.clone()}
            />
        }
    }

    // TODO LOAD FROM DISK
    // TODO LOAD FROM DISK
    // TODO LOAD FROM DISK
    // TODO LOAD FROM DISK
    // TODO LOAD FROM DISK
    // TODO LOAD FROM DISK
    // TODO LOAD FROM DISK
    fn view_config(&self) -> Html {
        html! {
            <Config
                inventory_buttons={
                    InventoryButtonCollection { user_item_types: vec![]}
                }
                inventory={self.inventory.clone()}
                add_inventory_button={self.add_inventory_button.as_ref().expect("add inv button cb")}
            />
        }
    }

    fn view_nav(&self) -> Html {
        if self.nav_state == NavState::Visible {
            html! {
                <Nav
                    page={self.page}
                    nav_to={self.nav_to.as_ref().expect("nav cb")}
                />
            }
        } else {
            html! { <></> }
        }
    }
}
