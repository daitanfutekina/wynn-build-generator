use std::rc::Rc;
use yew::prelude::*;
use gloo::timers::callback::Timeout;
use super::AutocompleteInput;
use crate::best_build_search::helper_enums::{CalcStat, SearchParam};

#[derive(Properties, PartialEq)]
pub struct BuildOrdProps{
    #[prop_or_default]
    /// Callback to retrieve items when this component loses focus
    pub on_leave: Callback<SearchParam>,
    #[prop_or_default]
    pub start_value: Option<SearchParam>
}
pub enum BuildOrdMsg{
    OnFocus,
    InputChanged(usize),
    OnBlur,
    OnLeave
}
pub struct OptimizingStatInput{
    focused: bool,
    selection: SearchParam,
    options: Vec<SearchParam>,
    option_names: Rc<Vec<String>>,
    option_colors: Rc<Vec<String>>,
    unfocus_handle: Option<Timeout>
}
impl Component for OptimizingStatInput{
    type Message = BuildOrdMsg;

    type Properties = BuildOrdProps;

    fn create(ctx: &Context<Self>) -> Self {
        let options: Vec<SearchParam> = SearchParam::all_varients();
        let option_names = super::STAT_RENAMES.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let option_colors = super::STAT_COLOR_CLASSES.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        // items.sort_by(|a, b| a.name().cmp(b.name()));
        OptimizingStatInput{focused: false, unfocus_handle: None, selection: match ctx.props().start_value{Some(v) => v, None => SearchParam::Calc(CalcStat::MeleeHit)}, option_names: option_names.into(), option_colors: option_colors.into(),options}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            BuildOrdMsg::OnFocus => {
                self.focused=true;
            },
            BuildOrdMsg::InputChanged(option_idx) => {
                self.focused=true;
                self.selection=self.options[option_idx];
            },
            BuildOrdMsg::OnBlur => {
                // onblur always gets called when nested `<input/>`'s lose focus, even if the focus was redirected to another nested input type. 
                // i want OnLeave to *only* get called when all the nested components lose focus (none of the nested components have focus)
                // to prevent this, i set a timeout to delay OnLeave from being called until after OnFocus gets an opportunity to get called again,
                // preventing false 'onblur' calls
                if self.focused{
                    self.focused=false;
                    let link = ctx.link().clone();
                    self.unfocus_handle = Some(Timeout::new(0, move || link.send_message(BuildOrdMsg::OnLeave)));    
                }else{
                    return false
                }
            },
            BuildOrdMsg::OnLeave => {
                if !self.focused{
                    // emit callback using the selection
                    ctx.props().on_leave.emit(self.selection);
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
                <h3>{"Maximizing Stat"}</h3>
                <div class="build-ord-area">
                    <AutocompleteInput class={format!("build-ordering-input {}",self.option_colors[self.selection.usize_id()])} char_req=0 force=true options = {&self.option_names} on_select = {link.callback(move |(idx,_)| BuildOrdMsg::InputChanged(idx))} options_classes={&self.option_colors} value={self.option_names[self.selection.usize_id()].clone()}/>
                </div>
            </>
        };

        html!{
            <div class={format!("build-ord-wrapper")} onfocus={link.callback(|_| BuildOrdMsg::OnFocus)} onblur={link.callback(|_| BuildOrdMsg::OnBlur)}>
                {div_content}
            </div>
        }
    }
}