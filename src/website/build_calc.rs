use std::rc::Rc;

use web_sys::{console, Event, HtmlInputElement};
use yew::{prelude::*, virtual_dom::AttrValue};
use gloo::timers::callback::Timeout;

use crate::{wynn_data::{atree::AtreeBuild, builder::WynnBuild, items::{self, Category, Tier, Type, WynnItem, Atrs}, Class}};
use crate::best_build_calc::{BestBuildCalc,CalcOrd};

use super::{build_disp::BuildDisp, build_reqs_input::StatReq};

#[derive(Properties, PartialEq)]
pub struct BuildCalcProps{
    pub items: [Vec<WynnItem>; 8],
    pub weapon: WynnItem,
    pub atree: Rc<AtreeBuild>,
    pub min_reqs: Vec<StatReq>,
    pub max_reqs: Vec<StatReq>,
    pub optimizing_stat: CalcOrd,
    #[prop_or_default]
    pub on_finish: Callback<Vec<WynnBuild>>
}
pub enum CalcMsg{
    ReqStart,
    StartCalc,
    ContinueCalc,
    CalcDone,
    PeekCalc
}
pub struct BuildCalcComponent {
    calc: BestBuildCalc,
    res_builds: Vec<WynnBuild>,
    peek_counter: u32, // used to save data and optionally display the best found builds every few seconds
    progress: f64,
    handle: Option<Timeout>
}
impl Component for BuildCalcComponent{
    type Message = CalcMsg;

    type Properties = BuildCalcProps;

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let handle = Some(Timeout::new(0, move || link.send_message(CalcMsg::StartCalc)));
        BuildCalcComponent{calc: Default::default(), res_builds: Default::default(), handle, peek_counter: 0, progress: 0.0}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CalcMsg::ReqStart => {

                true
            },
            CalcMsg::StartCalc => {

                self.calc = BestBuildCalc::make(ctx.props().weapon, ctx.props().items.clone(), Default::default(), 
                ctx.props().optimizing_stat.ord_fn());

                let mut min_stats = Vec::new();
                for s in ctx.props().min_reqs.iter(){
                    match s{
                        StatReq::Stat(atr, v) => min_stats.push((*atr,*v)),
                        StatReq::EhpReq(v) => self.calc.set_min_ehp(*v),
                        StatReq::Sp1PerSec(_) => (),
                        StatReq::Sp2PerSec(_) => (),
                        StatReq::Sp3PerSec(_) => (),
                        StatReq::Sp4PerSec(_) => (),
                    }
                }
                
                let link = ctx.link().clone();
                self.handle = Some(Timeout::new(0, move || link.send_message(CalcMsg::ContinueCalc)));
                false
            },
            CalcMsg::ContinueCalc => {
                self.peek_counter+=1;
                self.progress=self.calc.progress();
                if self.peek_counter>100 {
                    ctx.link().send_message(CalcMsg::PeekCalc);
                    self.peek_counter=0;
                }
                if self.calc.calc_best_build(1000){ // num builds to calculate before website is allowed to update
                    ctx.link().send_message(CalcMsg::CalcDone);
                }else{
                    let link = ctx.link().clone();
                    self.handle = Some(Timeout::new(0, move || link.send_message(CalcMsg::ContinueCalc)));
                }
                true
            },
            CalcMsg::CalcDone => {
                self.res_builds = self.calc.curr_bests.iter().map(|(ord,bld)| bld.clone()).collect::<Vec<WynnBuild>>();
                ctx.props().on_finish.emit(self.res_builds.clone());
                self.progress = 1.0;
                true
            },
            CalcMsg::PeekCalc => {
                // let get_cookies = document().unchecked_into::<HtmlDocument>().cookie().unwrap_or(String::from("None"));
                self.res_builds = self.calc.curr_bests.iter().map(|(ord,bld)| bld.clone()).collect::<Vec<WynnBuild>>();
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            <div class="build-calc">
                <div class="calc-loading">
                    <h3>{format!("{:.2}",self.progress*100.0)}</h3>
                </div>
                <div class="display-builds">
                {
                    self.res_builds.iter().map(|bld|
                        html!{
                            <BuildDisp build={bld.clone()}/>
                        }
                    ).collect::<Html>()
                }
                </div>
            </div>
        }
    }
}