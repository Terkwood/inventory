use crate::model::*;
use yew::prelude::*;

pub struct InventoryButtons {
    pub link: ComponentLink<Self>,
    pub props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub inventory_buttons: InventoryButtonCollection,
}

impl Component for InventoryButtons {
    type Message = ();

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        todo!()
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
        let free = self.props.inventory_buttons.free_user_buttons();
        html! {
         <div class="configsection">
            <h1>{ "Configure Inventory Buttons"}</h1>
            <h2>{ "Current buttons:" }</h2>
            <div> { self.props.inventory_buttons.all().iter().map(view_item_type).collect::<Html>() } </div>
            <h2>{ "Add a button" }</h2>
            <div> {
                if free > 0 {
                    format!("You may add {} more buttons.", free)
                } else {
                    "You need to delete a button before you can add another.".to_string()
                }
            } </div>
            {
                if free > 0 {
                    self.view_button_selections()
                } else {
                    html! { <></> }
                }
            }
         </div>
        }
    }
}

impl InventoryButtons {
    fn view_button_selections(&self) -> Html {
        InventoryButtonCollection::allowed_emojis()
            .iter()
            .map(view_button)
            .collect::<Html>()
    }
}

fn view_button(s: &String) -> Html {
    todo!()
}

fn view_item_type(item_type: &ItemType) -> Html {
    let emoji = &item_type.emoji;
    let name = &item_type.name;
    html! { <div>{ format!("{} {}", emoji, name) }</div> }
}
