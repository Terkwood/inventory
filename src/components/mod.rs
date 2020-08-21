mod app;
mod daily;
mod nav;

pub use app::App;

#[derive(Copy, Clone, PartialEq)]
pub enum Page {
    Daily,
    History,
    Config,
}

impl Default for Page {
    fn default() -> Self {
        Page::Daily
    }
}

impl Page {
    pub fn order() -> Vec<Page> {
        vec![Page::Daily, Page::History, Page::Config]
    }

    fn next(self) -> Page {
        todo!() //Mode::order().iter().find(self))
    }

    fn prev(self) -> Page {
        todo!()
    }
}
