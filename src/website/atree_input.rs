use std::rc::Rc;

use yew::{prelude::*};
use gloo::timers::callback::Timeout;

use super::AutocompleteInput;
use crate::wynn_data::{items, items::{WynnItem, Type, Category, Tier}, Class, atree::{AtreeBuild}};


#[derive(Properties, PartialEq)]
pub struct AtreeInputProps{
    #[prop_or_default]
    pub class: Option<Class>,
    #[prop_or_default]
    /// Callback to retrieve items when this component loses focus
    pub on_leave: Callback<Option<WynnItem>>,
}
pub enum AtreeInputMsg{
    OnFocus,
    InputChanged(Option<usize>, String),
    OnBlur,
    OnLeave
}
pub struct AtreeInput{
    focused: bool,
    selection: Option<WynnItem>,
    items: Vec<WynnItem>,
    item_names: Rc<Vec<String>>,
    item_rarities: Rc<Vec<String>>,
    unfocus_handle: Option<Timeout>
}
impl Component for AtreeInput{
    type Message = AtreeInputMsg;

    type Properties = AtreeInputProps;

    fn create(ctx: &Context<Self>) -> Self {
        let items: Vec<WynnItem> = items::iter().filter(|itm| itm.get_category()==Category::Weapon).collect();
        // items.sort_by(|a, b| a.name().cmp(b.name()));
        AtreeInput{focused: false, unfocus_handle: None, selection: None, item_names: items.iter().map(|itm| itm.name().to_string()).collect::<Vec<String>>().into(), item_rarities: items.iter().map(|itm| itm.get_tier().to_string()).collect::<Vec<String>>().into(), items}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            AtreeInputMsg::OnFocus => {
                self.focused=true;
            },
            AtreeInputMsg::InputChanged(option_idx, s) => {
                match option_idx{
                    Some(id) => {
                        if self.items[id].is_null(){
                            self.selection=None
                        }else{
                            self.selection=Some(self.items[id]);
                        }
                    },
                    None => {
                        self.selection=None;
                    }
                }
            },
            AtreeInputMsg::OnBlur => {
                // onblur always gets called when nested `<input/>`'s lose focus, even if the focus was redirected to another nested input type. 
                // i want OnLeave to *only* get called when all the nested components lose focus (none of the nested components have focus)
                // to prevent this, i set a timeout to delay OnLeave from being called until after OnFocus gets an opportunity to get called again,
                // preventing false 'onblur' calls
                if self.focused{
                    self.focused=false;
                    let link = ctx.link().clone();
                    self.unfocus_handle = Some(Timeout::new(0, move || link.send_message(AtreeInputMsg::OnLeave)));    
                }else{
                    return false
                }
            },
            AtreeInputMsg::OnLeave => {
                if !self.focused{
                    // emit callback using the selection
                    ctx.props().on_leave.emit(self.selection.clone());
                }else{
                    return false
                }
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let item_type = match self.selection{Some(itm) => itm.get_type(), None => Type::Spear};
        let item_tier = match self.selection{Some(itm) => itm.get_tier(), None => Tier::Common};

        let div_content = html!{
            <>
                <h3>{"Atree (work in progress)"}</h3>
                <img src={format!("images/wynn-{}.png",item_type.to_string().to_lowercase())}/>
                <div class="atree-selection-area">
                    <AutocompleteInput class={format!("atree-input {}",item_tier)} placeholder="Disabled" disabled=true options = {&self.item_names} on_leave = {link.callback(move |(op, s)| AtreeInputMsg::InputChanged(op, s))} on_select = {link.callback(move |(idx,s)| AtreeInputMsg::InputChanged(Some(idx),s))} options_classes ={&self.item_rarities}/>
                </div>
            </>
        };

        html!{
            <div class={format!("atree-input-wrapper")} onfocus={link.callback(|_| AtreeInputMsg::OnFocus)} onblur={link.callback(|_| AtreeInputMsg::OnBlur)}>
                {div_content}
            </div>
        }
    }
}