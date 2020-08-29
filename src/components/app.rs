use super::*;
use crate::model::*;
use crate::repo::{ButtonsRepo, InventoryRepo};
use crate::time::{js_local_offset, js_utc_now};
use yew::prelude::*;

pub struct App {
    page: Page,
    inventory_repo: InventoryRepo,
    buttons_repo: ButtonsRepo,
    inventory: Inventory,
    buttons: InventoryButtonCollection,
    nav_state: NavState,
    add_item: Option<Callback<Item>>,
    resolve_item: Option<Callback<UtcMillis>>,
    nav_to: Option<Callback<Page>>,
    show_nav: Option<Callback<bool>>,
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
    ShowNav(bool),
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
        let show_nav = Some(link.callback(|b| Msg::ShowNav(b)));
        let add_inventory_button =
            Some(link.callback(|item_type| Msg::AddInventoryButton(item_type)));

        let inventory_repo = InventoryRepo::new();
        let inventory = inventory_repo.read_inventory();

        let buttons_repo = ButtonsRepo::new();
        let buttons = buttons_repo.read_buttons();

        let page = Page::default();

        Self {
            page,
            inventory_repo,
            buttons_repo,
            inventory,
            buttons,
            add_item,
            resolve_item,
            nav_state: NavState::Visible,
            nav_to,
            show_nav,
            add_inventory_button,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddItem(item) => {
                self.inventory.add(item);
                self.inventory_repo.save_inventory(&self.inventory)
            }
            Msg::ResolveItem(utc) => {
                self.inventory.resolve(utc.0);
                self.inventory_repo.save_inventory(&self.inventory)
            }
            Msg::NavigateTo(page) => self.page = page,
            Msg::ShowNav(b) => {
                if b {
                    self.nav_state = NavState::Visible
                } else {
                    self.nav_state = NavState::Hidden
                }
            }
            Msg::AddInventoryButton(item_type) => {
                self.buttons.add(item_type);
                self.buttons_repo.save_buttons(&self.buttons)
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
                inventory={self.inventory.today(js_utc_now(), js_local_offset())}
                add_item={self.add_item.as_ref().expect("add item cb")}
                resolve_item={self.resolve_item.as_ref().expect("resolve item cb")}
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

    fn view_config(&self) -> Html {
        html! {
            <Config
                inventory_buttons={self.buttons.clone()}
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
