use std::rc::Rc;

use yew::{prelude::*};
use gloo::timers::callback::Timeout;

use super::AutocompleteInput;
use crate::{wynn_data::{atree::{archer, assassin, mage, shaman, warrior, AtreeBuild}, items::{self, Category, Tier, Type, WynnItem}, Class}, WynnEnum};


#[derive(Properties, PartialEq)]
pub struct AtreeInputProps{
    #[prop_or_default]
    /// Starting AtreeBuild
    pub start_value: Rc<AtreeBuild>,
    #[prop_or_default]
    pub class: Option<Class>,
    #[prop_or_default]
    /// Callback to retrieve atree when this component loses focus
    pub on_leave: Callback<AtreeBuild>,
}
pub enum AtreeInputMsg{
    OnFocus,
    UpdateClass,
    AddAtreeItem(Option<usize>, String),
    RemoveItem(usize),
    OnBlur,
    OnLeave
}
pub struct AtreeInput{
    focused: bool,
    selection: Vec<usize>,
    atree_item_names: Rc<Vec<String>>,
    item_colors: Rc<Vec<String>>,
    unfocus_handle: Option<Timeout>,
    curr_class: Option<Class>
}
impl Component for AtreeInput{
    type Message = AtreeInputMsg;

    type Properties = AtreeInputProps;

    fn create(ctx: &Context<Self>) -> Self {
        let items: Vec<WynnItem> = items::iter().filter(|itm| itm.get_category()==Category::Weapon).collect();


        let atree_item_names = atree_item_names_from_class(ctx.props().class);
        let start_value_data = ctx.props().start_value.get_atree_items_ids();
        let selection = match ctx.props().class{Some(c) => if start_value_data.0==c{start_value_data.1.iter().map(|v| *v as usize).collect()}else{Vec::new()}, None => Vec::new()};
        // items.sort_by(|a, b| a.name().cmp(b.name()));
        AtreeInput{focused: false, unfocus_handle: None, selection, atree_item_names: atree_item_names.into(), item_colors: items.iter().map(|itm| itm.get_tier().to_string()).collect::<Vec<String>>().into(), curr_class: ctx.props().class}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            AtreeInputMsg::OnFocus => {
                self.focused=true;
                if self.curr_class!=ctx.props().class{
                    ctx.link().send_message(AtreeInputMsg::UpdateClass);
                }
            },
            AtreeInputMsg::UpdateClass => {
                self.curr_class=ctx.props().class;
                self.atree_item_names=atree_item_names_from_class(ctx.props().class).into();
                self.selection=Vec::new();
                ctx.props().on_leave.emit(Default::default());
            },
            AtreeInputMsg::AddAtreeItem(option_idx, s) => {
                match option_idx{
                    Some(id) => {
                        match self.selection.binary_search(&id){
                            Ok(_) => (),
                            Err(idx) => self.selection.insert(idx, id)
                        }
                    },
                    None => ()
                }
            },
            AtreeInputMsg::RemoveItem(idx) => {
                self.selection.remove(idx);
            }
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
                    match ctx.props().class{
                        Some(c) => ctx.props().on_leave.emit(match c{
                            Class::Archer => self.selection.iter().map(|id| archer::AtreeItems::VARIENTS[*id]).collect::<AtreeBuild>(),
                            Class::Warrior => self.selection.iter().map(|id| warrior::AtreeItems::VARIENTS[*id]).collect::<AtreeBuild>(),
                            Class::Mage => self.selection.iter().map(|id| mage::AtreeItems::VARIENTS[*id]).collect::<AtreeBuild>(),
                            Class::Assassin => self.selection.iter().map(|id| assassin::AtreeItems::VARIENTS[*id]).collect::<AtreeBuild>(),
                            Class::Shaman => self.selection.iter().map(|id| shaman::AtreeItems::VARIENTS[*id]).collect::<AtreeBuild>(),
                        }),
                        None => return false,
                    }
                    // ctx.props().on_leave.emit(self.selection.clone());
                }else{
                    return false
                }
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        if self.curr_class!=ctx.props().class{
            link.send_message(AtreeInputMsg::UpdateClass);
        }

        let div_content = html!{
            <>
                <h3>{"Atree (work in progress)"}</h3>
                <img src={"images/wynn-atree-icon.png"}/>
                <div class="atree-selection-area">
                {self.selection.iter().enumerate().map(|(idx,item_id)| {
                    html!{
                        <button class = "atree-item-button" onclick={link.callback(move |_| AtreeInputMsg::RemoveItem(idx))}>
                        <div class = {format!("atree-item {}", self.item_colors[*item_id])}>
                        {&self.atree_item_names[*item_id]}
                        </div>
                        </button>
                    }}
                ).collect::<Html>()}
                    <AutocompleteInput class={format!("atree-input")} reset=true placeholder={match ctx.props().class{Some(c) => format!("Type an {} ability",c), None => "Select a weapon".to_string()}} disabled={ctx.props().class.is_none()} options = {&self.atree_item_names} on_leave = {link.callback(move |(op, s)| AtreeInputMsg::AddAtreeItem(op, s))} options_classes ={&self.item_colors}/>
                </div>
            </>
        };

        html!{
            <div class={format!("atree-input-wrapper")} title="report errors to @yellowly" onfocus={link.callback(|_| AtreeInputMsg::OnFocus)} onblur={link.callback(|_| AtreeInputMsg::OnBlur)}>
                {div_content}
            </div>
        }
    }
}

fn atree_item_names_from_class(class: Option<Class>) -> Vec<String>{
    match class{Some(c) => 
        match c{
            Class::Archer => archer::AtreeItems::iter().map(|v| v.to_string()).collect::<Vec<String>>(),
            Class::Warrior => warrior::AtreeItems::iter().map(|v| v.to_string()).collect::<Vec<String>>(),
            Class::Mage => mage::AtreeItems::iter().map(|v| v.to_string()).collect::<Vec<String>>(),
            Class::Assassin => assassin::AtreeItems::iter().map(|v| v.to_string()).collect::<Vec<String>>(),
            Class::Shaman => shaman::AtreeItems::iter().map(|v| v.to_string()).collect::<Vec<String>>(),
        }, 
        None => Vec::new()
    }
}