use std::rc::Rc;
use yew::prelude::*;
use gloo::timers::callback::Timeout;
use super::AutocompleteInput;
use crate::best_build_calc::CalcOrd;

#[derive(Properties, PartialEq)]
pub struct BuildOrdProps{
    #[prop_or_default]
    /// Callback to retrieve items when this component loses focus
    pub on_leave: Callback<CalcOrd>,
}
pub enum BuildOrdMsg{
    OnFocus,
    InputChanged(usize),
    OnBlur,
    OnLeave
}
pub struct OptimizingStatInput{
    focused: bool,
    selection: CalcOrd,
    options: Vec<CalcOrd>,
    option_names: Rc<Vec<String>>,
    unfocus_handle: Option<Timeout>
}
impl Component for OptimizingStatInput{
    type Message = BuildOrdMsg;

    type Properties = BuildOrdProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let options: Vec<CalcOrd> = CalcOrd::VARIENTS.to_vec();
        // items.sort_by(|a, b| a.name().cmp(b.name()));
        OptimizingStatInput{focused: false, unfocus_handle: None, selection: CalcOrd::MeleeHit, option_names: options.iter().map(|op| op.to_string()).collect::<Vec<String>>().into(), options}
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
                    <AutocompleteInput class={format!("build-ordering-input")} force=true limit={usize::MAX} editable=false options = {&self.option_names} on_select = {link.callback(move |(idx,_)| BuildOrdMsg::InputChanged(idx))}/>
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