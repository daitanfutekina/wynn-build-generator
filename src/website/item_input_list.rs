use std::{collections::HashSet, rc::Rc};
use yew::{prelude::*, virtual_dom::AttrValue};
use gloo::timers::callback::Timeout;
use super::AutocompleteInput;
use crate::wynn_data::{items, items::{WynnItem, Type}};

#[derive(Properties, PartialEq)]
pub struct ItemInputProps{
    #[prop_or_default]
    /// Applies the `class` html tag to the wrapper div
    pub class: &'static str,
    #[prop_or("".into())]
    /// Applies the `id` html tag to the wrapper div
    pub id: AttrValue,
    /// What type (category) of wynncraft item this input is used for
    pub item_type: Type,
    #[prop_or(1)]
    /// Minimum number of item input this component can have
    pub min_inputs: usize,
    #[prop_or_default]
    /// Callback to retrieve items when this component loses focus
    pub on_leave: Callback<(Type, Vec<WynnItem>)>,
    #[prop_or_default]
    pub start_value: Vec<WynnItem>,
}
pub enum ItemInputMsg{
    OnFocus,
    InputChanged(usize, (Option<usize>, String)),
    OnBlur,
    OnLeave
}
pub struct ItemInput{
    focused: bool,
    selection: Vec<WynnItem>,
    items: Vec<WynnItem>,
    item_names: Rc<Vec<String>>,
    item_rarities: Rc<Vec<String>>,
    unfocus_handle: Option<Timeout>
}
impl Component for ItemInput{
    type Message = ItemInputMsg;

    type Properties = ItemInputProps;

    fn create(ctx: &Context<Self>) -> Self {
        let items: Vec<WynnItem> = items::iter().filter(|itm| itm.get_type()==ctx.props().item_type).collect();
        // items.sort_by(|a, b| a.name().cmp(b.name()));

        ItemInput{focused: false, unfocus_handle: None, 
            selection: if ctx.props().start_value.is_empty(){vec![WynnItem::NULL; ctx.props().min_inputs]} else {let mut temp = ctx.props().start_value.clone(); temp.push(WynnItem::NULL); temp}, 
            item_names: items.iter().map(|itm| itm.name().to_string()).collect::<Vec<String>>().into(), 
            item_rarities: items.iter().map(|itm| itm.get_tier().to_string()).collect::<Vec<String>>().into(), items}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            ItemInputMsg::OnFocus => {
                self.focused=true;
            },
            ItemInputMsg::InputChanged(input_idx, (option_idx, _)) => {
                match option_idx{
                    Some(id) => {
                        self.selection[input_idx]=self.items[id];
                        if input_idx==self.selection.len()-1{
                            self.selection.push(WynnItem::NULL);
                        }
                    },
                    None => {
                        self.selection[input_idx]=WynnItem::NULL;
                    }
                }
            },
            ItemInputMsg::OnBlur => {
                // onblur always gets called when nested `<input/>`'s lose focus, even if the focus was redirected to another nested input type. 
                // i want OnLeave to *only* get called when all the nested components lose focus (none of the nested components have focus)
                // to prevent this, i set a timeout to delay OnLeave from being called until after OnFocus gets an opportunity to get called again,
                // preventing false 'onblur' calls
                if self.focused{
                    self.focused=false;
                    let link = ctx.link().clone();
                    self.unfocus_handle = Some(Timeout::new(0, move || link.send_message(ItemInputMsg::OnLeave)));    
                }else{
                    return false
                }
            },
            ItemInputMsg::OnLeave => {
                if !self.focused{
                    // filter out 'null' items and remove them from the selection, in addition to removing duplicates
                    let mut found: HashSet<u32> = HashSet::new();
                    self.selection.retain(|itm| !itm.is_null() && found.insert(itm.id()));
                    // emit callback using the selection
                    ctx.props().on_leave.emit((ctx.props().item_type,self.selection.clone()));
                    self.selection.resize(ctx.props().min_inputs.max(self.selection.len()+1), WynnItem::NULL);
                }else{
                    return false
                }
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let div_content = html!{
            <>
                <h3>{ctx.props().item_type}</h3>
                <img src={format!("images/wynn-{}.png",ctx.props().item_type.to_string().to_lowercase())}/>
                <div class="item-input-list">
                    {self.selection.iter().enumerate().map(|(i,v)|{
                        html!{
                            <AutocompleteInput class={format!("item-list-input")} stick_to_input=true placeholder={format!("Insert {}",ctx.props().item_type)} value={if v.is_null(){String::new()}else{v.name().to_string()}} unfocus_delay=250 options = {&self.item_names} on_leave = {link.callback(move |v| ItemInputMsg::InputChanged(i, v))} on_select = {link.callback(move |(idx,s)| ItemInputMsg::InputChanged(i, (Some(idx),s)))} options_classes ={&self.item_rarities}/>
                        }
                    }).collect::<Html>()}
                </div>
            </>
        };

        if ctx.props().id.is_empty(){
            html!{
                <div class={format!("item-input-wrapper {}",ctx.props().class)} onfocus={link.callback(|_| ItemInputMsg::OnFocus)} onblur={link.callback(|_| ItemInputMsg::OnBlur)}>
                    {div_content}
                </div>
            }
        }else{
            html!{
                <div id={ctx.props().id.clone()} class={format!("item-input-wrapper {}",ctx.props().class)} onfocus={link.callback(|_| ItemInputMsg::OnFocus)} onblur={link.callback(|_| ItemInputMsg::OnBlur)}>
                    {div_content}
                </div>
            }
        }
    }
}