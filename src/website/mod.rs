use std::{cmp::Ordering, collections::HashSet, rc::Rc};

use web_sys::{console, Event, HtmlInputElement};
use yew::{prelude::*, virtual_dom::AttrValue};
use gloo::timers::callback::Timeout;

mod autocomplete; mod item_input_list; mod weapon_input; mod build_calc; mod build_disp; mod atree_input; mod build_reqs_input; mod build_ordering;

use autocomplete::AutocompleteInput;
use item_input_list::ItemInput;
use weapon_input::WeaponInput;
use build_calc::BuildCalcComponent;
use atree_input::AtreeInput;
use build_reqs_input::{BuildReqsInput, StatReq};
use build_ordering::OptimizingStatInput;

use crate::wynn_data::{atree::AtreeBuild, builder::WynnBuild, items::{self, Category, Tier, Type, WynnItem}, Class};
use crate::best_build_calc::{BestBuildCalc, CalcOrd};

pub enum RootMsg{
    ItemsUpdate(usize, Vec<WynnItem>),
    ReqsUpdate(Vec<(Ordering,StatReq)>),
    OptimizingStatUpdate(CalcOrd),
    GenerateBtnClicked,
    BeginSearch,
    None
}
pub struct RootComponent {
    // input_content: Vec<String>,
    // item_display: String,
    // items: [WynnItem; 9],
    // url_hash: String,
    // extra: String,
    items: [Vec<WynnItem>; 8],
    weapon: WynnItem,
    min_reqs: Vec<StatReq>,
    max_reqs: Vec<StatReq>,
    optimizing_stat: CalcOrd,
    atree: Rc<AtreeBuild>,
    calc: Option<BestBuildCalc>,
    res_builds: Vec<WynnBuild>,
    peek_counter: u32, // used to save data and optionally display the best found builds every few seconds
    handle: Option<Timeout>,
    on_input_page: bool
}
impl Component for RootComponent{
    type Message = RootMsg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        RootComponent{items: Default::default(), weapon: WynnItem::NULL, atree: Default::default(), 
            calc: None, res_builds: Default::default(), handle: None, peek_counter: 0,
        min_reqs: Vec::new(), max_reqs: Vec::new(), optimizing_stat: CalcOrd::MeleeHit, on_input_page: true}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            RootMsg::ItemsUpdate(type_id, itms) => {
                if type_id<8{
                    self.items[type_id]=itms;
                }else{
                    self.weapon = match itms.first(){
                        Some(itm) => *itm,
                        None => WynnItem::NULL,
                    }
                }
                false
            },
            RootMsg::ReqsUpdate(reqs) => {
                self.min_reqs=Vec::new();
                self.max_reqs=Vec::new();
                for (ordering, req) in reqs.into_iter(){
                    match ordering{
                        Ordering::Greater => self.min_reqs.push(req),
                        _ => self.max_reqs.push(req)
                    }
                }
                false
            },
            RootMsg::OptimizingStatUpdate(stat) => {
                self.optimizing_stat=stat;
                false
            },
            RootMsg::GenerateBtnClicked => {
                let link = ctx.link().clone();
                if self.items.iter().all(|v| !v.is_empty()) && !self.weapon.is_null(){
                    self.handle = Some(Timeout::new(1000, move || link.send_message(RootMsg::BeginSearch)));    
                }
                true
            }
            RootMsg::BeginSearch => {
                self.on_input_page=false;
                true
            },
            RootMsg::None => false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // if 
        let link = ctx.link();
        if self.on_input_page{
            html!{
                <div class={format!("page {}",if self.handle.is_none(){""}else{"transferring-page"})}>
                    <div class="header">
                        <h1>{"Wynncraft Build Generator"}</h1>
                    </div>
                    <div class="const-build-data">
                        <WeaponInput on_leave = {link.callback(|itm| RootMsg::ItemsUpdate(8, match itm{Some((_ty, itm)) => vec![itm], None => Vec::new()}))}/>
                        <AtreeInput/>
                    </div>
                    <div class="inputs-grid">
                        <ItemInput item_type = {Type::Helmet} on_leave = {link.callback(|(_, itms)| RootMsg::ItemsUpdate(0, itms))}/>
                        <ItemInput item_type = {Type::Chestplate} on_leave = {link.callback(|(_, itms)| RootMsg::ItemsUpdate(1, itms))}/>
                        <ItemInput item_type = {Type::Leggings} on_leave = {link.callback(|(_, itms)| RootMsg::ItemsUpdate(2, itms))}/>
                        <ItemInput item_type = {Type::Boots} on_leave = {link.callback(|(_, itms)| RootMsg::ItemsUpdate(3, itms))}/>
                        <ItemInput item_type = {Type::Ring} on_leave = {link.callback(|(_, itms)| RootMsg::ItemsUpdate(4, itms))}/>
                        <ItemInput item_type = {Type::Ring} on_leave = {link.callback(|(_, itms)| RootMsg::ItemsUpdate(5, itms))}/>
                        <ItemInput item_type = {Type::Bracelet} on_leave = {link.callback(|(_, itms)| RootMsg::ItemsUpdate(6, itms))}/>
                        <ItemInput item_type = {Type::Necklace} on_leave = {link.callback(|(_, itms)| RootMsg::ItemsUpdate(7, itms))}/>
                    </div>
                    <div class="search-params">
                        <BuildReqsInput on_leave = {link.callback(|v| RootMsg::ReqsUpdate(v))}/>
                        <OptimizingStatInput on_leave={link.callback(|stat| RootMsg::OptimizingStatUpdate(stat))}/>
                        <div class="gen-button-area">
                        <div class="gen-button-wrapper">
                            <button onclick={link.callback(|_| RootMsg::GenerateBtnClicked)}>{"Generate!"}</button>
                        </div>
                        </div>
                    </div>
                    <div class="bottom">
                    {format!("{:#?} {:#?} {:#?} {:#?} {:#?}",self.items,self.weapon, self.min_reqs,self.max_reqs,self.optimizing_stat)}
                    </div>
                </div>
            }
        }else{
            html!{
                <div class={format!("page {}",if self.handle.is_none(){""}else{"transferring-page"})}>
                <BuildCalcComponent items={self.items.clone()} weapon={self.weapon.clone()} atree={&self.atree} min_reqs={self.min_reqs.clone()} max_reqs={self.max_reqs.clone()} optimizing_stat={self.optimizing_stat.clone()} />
                </div>
            }
        }
    }
}