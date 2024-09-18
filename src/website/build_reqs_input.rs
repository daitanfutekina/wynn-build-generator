use std::{cmp::Ordering, rc::Rc};

use web_sys::HtmlInputElement;
use yew::prelude::*;
use gloo::timers::callback::Timeout;

use crate::best_build_search::helper_enums::SearchReq;
use super::AutocompleteInput;

#[derive(Properties, PartialEq)]
pub struct BuildReqsProps{
    #[prop_or_default]
    /// Callback to retrieve stat requirements when this component loses focus
    pub on_leave: Callback<Vec<(Ordering,SearchReq)>>,
    #[prop_or_default]
    pub start_value: (Vec<SearchReq>,Vec<SearchReq>),
}
pub enum BuildReqsMsg{
    OnFocus,
    AddReq,
    RemoveReq(usize),
    StatInput(Option<usize>),
    NumInput(String),
    OrdToggle,
    OnBlur,
    OnLeave,
    None
}
pub struct BuildReqsInput{
    focused: bool,
    selection: Vec<(Ordering,SearchReq)>,
    stat_names: Rc<Vec<String>>,
    stat_colors: Rc<Vec<String>>,
    curr_stat_input: Option<usize>,
    curr_num_input: String,
    curr_ord_input: Ordering,
    reset_input: bool,
    unfocus_handle: Option<Timeout>
}

impl Component for BuildReqsInput{
    type Message = BuildReqsMsg;

    type Properties = BuildReqsProps;

    fn create(ctx: &Context<Self>) -> Self {
        let stat_names = super::STAT_RENAMES.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let stat_colors = super::STAT_COLOR_CLASSES.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let mut start_value = ctx.props().start_value.0.iter().map(|r| (Ordering::Greater, r.clone())).collect::<Vec<(Ordering, SearchReq)>>();
        start_value.extend(ctx.props().start_value.1.iter().map(|r| (Ordering::Less, r.clone())));
        BuildReqsInput{focused: false, unfocus_handle: None, selection: start_value, stat_names: stat_names.into(), stat_colors: stat_colors.into(), curr_stat_input: None, curr_num_input: String::new(), reset_input: false, curr_ord_input: Ordering::Greater}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            BuildReqsMsg::OnFocus => {
                self.focused=true;
            },
            BuildReqsMsg::StatInput(s) => {
                self.reset_input = false;
                self.curr_stat_input = s;
            },
            BuildReqsMsg::NumInput(n) => {
                self.unfocus_handle=None;
                self.curr_num_input=n;
                self.reset_input = false;
            },
            BuildReqsMsg::OrdToggle => {
                self.unfocus_handle=None;
                if self.curr_ord_input==Ordering::Less{
                    self.curr_ord_input=Ordering::Greater
                }else{
                    self.curr_ord_input=Ordering::Less
                }
                self.reset_input = false;
            },
            BuildReqsMsg::AddReq => {
                match self.curr_stat_input{
                    Some(id) => match self.curr_num_input.parse::<f32>() {
                        Ok(num_val) => {
                            let temp = SearchReq::from_usize_and_f32(id, num_val);
                            let find = self.selection.iter().position(|a| a.0==self.curr_ord_input && a.1.stat_eq(&temp));
                            match find{
                                Some(i) => {
                                    self.selection[i] = (self.curr_ord_input,temp);
                                },
                                None => {
                                    self.selection.push((self.curr_ord_input,temp));
                                }
                            }
                            self.curr_stat_input=None;
                            self.curr_num_input=String::new();
                            self.curr_ord_input=Ordering::Greater;
                            self.reset_input=true;
                            let link = ctx.link().clone();
                            self.unfocus_handle = Some(Timeout::new(0, move || link.send_message(BuildReqsMsg::NumInput(String::new()))));    
            
                        },
                        Err(_) => ()
                    }
                    None => ()
                }
            },
            BuildReqsMsg::RemoveReq(n) => {
                self.selection.remove(n);
            },
            BuildReqsMsg::OnBlur => {
                // onblur always gets called when nested `<input/>`'s lose focus, even if the focus was redirected to another nested input type. 
                // i want OnLeave to *only* get called when all the nested components lose focus (none of the nested components have focus)
                // to prevent this, i set a timeout to delay OnLeave from being called until after OnFocus gets an opportunity to get called again,
                // preventing false 'onblur' calls
                if self.focused{
                    self.focused=false;
                    let link = ctx.link().clone();
                    self.unfocus_handle = Some(Timeout::new(0, move || link.send_message(BuildReqsMsg::OnLeave)));    
                }else{
                    return false
                }
            },
            BuildReqsMsg::OnLeave => {
                if !self.focused{
                    // emit callback using the selection
                    ctx.props().on_leave.emit(self.selection.clone());
                }else{
                    return false
                }
            },
            BuildReqsMsg::None => {
                return false
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let div_content = html!{
            <>
                <h3>{"Stat Requirements"}</h3>
                <div class="stat-reqs-area">
                    {self.selection.iter().enumerate().map(|(idx,(ord,stat_req))| {
                        let temp = stat_req.debug_name_and_val();
                        html!{
                            <button class = "stat-req" onclick={link.callback(move |_| BuildReqsMsg::RemoveReq(idx))}>
                            <div class = {format!("stat-type {}", self.stat_colors[stat_req.usize_id()])}>
                            {temp.0}
                            </div>
                            <div class = "stat-comparator">
                            {match ord{Ordering::Greater => ">", Ordering::Less => "<", Ordering::Equal => "="}}
                            </div>
                            <div class = "stat-value">
                            {temp.1}
                            </div>
                            </button>
                        }}
                    ).collect::<Html>()}
                    <br/>
                    <div class="stat-input-wrapper" onkeypress={link.callback(|key:KeyboardEvent| {if key.char_code()==13 {BuildReqsMsg::AddReq} else {BuildReqsMsg::None}})} onblur={link.callback(|_| BuildReqsMsg::AddReq)}>
                    <AutocompleteInput class={format!("stat-input")} placeholder="Insert a Stat" reset={self.reset_input} options = {&self.stat_names} on_leave = {link.callback(move |(op, _)| BuildReqsMsg::StatInput(op))} options_classes ={&self.stat_colors}/>
                    <button class="comparator" onclick = {link.callback(|_| BuildReqsMsg::OrdToggle)}>{match self.curr_ord_input{Ordering::Greater => ">", Ordering::Less => "<", Ordering::Equal => "="}}</button>
                    <input class="num-input" placeholder="Number" oninput={link.callback(|event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); BuildReqsMsg::NumInput(input.value())})} value = {self.curr_num_input.clone()}/>
                    </div>
                </div>
            </>
        };
        
        html!{
            <div class={format!("stat-reqs-wrapper")} onfocus={link.callback(|_| BuildReqsMsg::OnFocus)} onblur={link.callback(|_| BuildReqsMsg::OnBlur)}>
                {div_content}
            </div>
        }
    }
}