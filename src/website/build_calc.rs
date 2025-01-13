use std::rc::Rc;

use wasm_bindgen::JsCast;
use web_sys::HtmlDocument;
use yew::prelude::*;
use gloo::{timers::callback::Timeout, utils::document};

use crate::wynn_data::{atree::AtreeBuild, builder::WynnBuild, items::{WynnItem, Atrs}};
use crate::best_build_search::{BestBuildSearch,helper_enums::{SearchReq,SearchParam,CalcStat}};

use super::build_disp::BuildDisp;

#[derive(Properties, PartialEq)]
pub struct BuildCalcProps{
    pub items: [Vec<WynnItem>; 8],
    pub weapon: WynnItem,
    pub atree: Rc<AtreeBuild>,
    pub min_reqs: Vec<SearchReq>,
    pub max_reqs: Vec<SearchReq>,
    pub optimizing_stat: SearchParam,
    #[prop_or_default]
    pub on_finish: Callback<Vec<WynnBuild>>,
    #[prop_or_default]
    pub start_value: (u64,Vec<WynnBuild>)
}
pub enum CalcMsg{
    StartCalc,
    ContinueCalc,
    CalcDone,
    PeekCalc
}
pub struct BuildCalcComponent {
    calc: BestBuildSearch,
    res_builds: Vec<WynnBuild>,
    peek_counter: u32, // used to save data and optionally display the best found builds every few seconds
    progress: (u64,u64),
    handle: Option<Timeout>,

    disp_stats: Rc<Vec<SearchParam>>
}
impl Component for BuildCalcComponent{
    type Message = CalcMsg;

    type Properties = BuildCalcProps;

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let handle = Some(Timeout::new(0, move || link.send_message(CalcMsg::StartCalc)));
        BuildCalcComponent{calc: Default::default(), res_builds: ctx.props().start_value.1.clone(), handle, peek_counter: 0, progress: (ctx.props().start_value.0,0), 
            disp_stats: vec![SearchParam::Calc(CalcStat::MeleeDps),
            SearchParam::Calc(CalcStat::Sp1Dmg),SearchParam::Calc(CalcStat::Sp2Dmg),SearchParam::Calc(CalcStat::Sp3Dmg),SearchParam::Calc(CalcStat::Sp4Dmg),
            SearchParam::Stat(Atrs::Mr),SearchParam::Calc(CalcStat::Ehp)].into()}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CalcMsg::StartCalc => {
                self.calc = BestBuildSearch::make(ctx.props().weapon, ctx.props().items.clone(), ctx.props().atree.clone(), 
                match ctx.props().optimizing_stat{
                    SearchParam::Stat(a) => atr_to_calc_fn(a),
                    SearchParam::Calc(c) => c.ord_fn_i32(),
                });
                self.calc.set_min_stat_requirements(ctx.props().min_reqs.clone());
                self.calc.set_max_stat_requirements(ctx.props().max_reqs.clone());

                self.calc.skip_combinations(self.progress.0);
                if self.progress.0!=0{
                    self.calc.set_best_builds(self.res_builds.clone());
                }
                
                let link = ctx.link().clone();
                self.handle = Some(Timeout::new(0, move || link.send_message(CalcMsg::ContinueCalc)));
                false
            },
            CalcMsg::ContinueCalc => {
                self.peek_counter+=1;
                self.progress=self.calc.progress_frac();
                if self.peek_counter>1000 { // save progress every few seconds
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
                self.progress = (self.calc.total_combinations(),self.calc.total_combinations());
                self.save_progress();
                ctx.props().on_finish.emit(self.res_builds.clone());
                true
            },
            CalcMsg::PeekCalc => {
                self.save_progress();
                true
            },
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <div class="build-calc">
                <div class="calc-loading">
                    <h3>{format!("{:.2}%",self.progress.0 as f64/self.progress.1 as f64*100.0)}</h3>
                </div>
                <div class="display-builds">
                {
                    self.res_builds.iter().enumerate().map(|(i,bld)|
                        html!{
                            <BuildDisp build={bld.clone()} title={format!("Build {}",i)} disp_stats={&self.disp_stats}/>
                        }
                    ).collect::<Html>()
                }
                </div>
            </div>
        }
    }
}
impl BuildCalcComponent{
    fn save_progress(&mut self){
        let doc = document().unchecked_into::<HtmlDocument>();
        self.res_builds = self.calc.peek_curr_bests().map(|(_ord,bld)| bld.clone()).collect::<Vec<WynnBuild>>();
        let _ = doc.set_cookie(&format!("CalcProgress={},{}; expires=Tue, 19 Jan 2038 03:14:07 UTC;",self.progress.0,self.res_builds.iter().map(|b| b.generate_hash()).collect::<Vec<String>>().join(",")));
    }
}

// rust requires closures to not consume any variables to cast so i have to do this stupid thing to force it to work

macro_rules! get_stat_fn_matcher(
    ($a: ident, $($atr: ident),*) => {
        match $a {
            $(
                Atrs::$atr => |bld| bld.get_stat(Atrs::$atr),
            )*
            _ => panic!("Can't calculate value for Atrs::{}",$a)
        }
    }
);
fn atr_to_calc_fn(a: Atrs) -> fn(&WynnBuild) -> i32{
    get_stat_fn_matcher!(a,Hp,EDef,TDef,WDef,FDef,ADef,DamMult,DefMult,AtkTier,CritDamPct,DamPct,DamRaw,ESteal,Expd,HealPct,HpBonus,HprPct,HprRaw,Jh,Kb,Lb,Ls,MainAtkRange,MaxMana,MdPct,MdRaw,Mr,Ms,Poison,Ref,SdPct,SdRaw,SlowEnemy,SpPct1,SpPct2,SpPct3,SpPct4,SpRaw1,SpRaw2,SpRaw3,SpRaw4,Spd,Sprint,SprintReg,Thorns,WeakenEnemy,Xpb,NDamPct,EDamPct,TDamPct,WDamPct,FDamPct,ADamPct,RDamPct,NSdPct,ESdPct,TSdPct,WSdPct,FSdPct,ASdPct,RSdPct,EDefPct,TDefPct,WDefPct,FDefPct,ADefPct,RDefPct,NDamRaw,EDamRaw,TDamRaw,WDamRaw,FDamRaw,ADamRaw,RDamRaw,NMdPct,EMdPct,TMdPct,WMdPct,FMdPct,AMdPct,RMdPct,NMdRaw,EMdRaw,TMdRaw,WMdRaw,FMdRaw,AMdRaw,RMdRaw,NSdRaw,ESdRaw,TSdRaw,WSdRaw,FSdRaw,ASdRaw,RSdRaw,NAddDam,EAddDam,TAddDam,WAddDam,FAddDam,AAddDam)
}