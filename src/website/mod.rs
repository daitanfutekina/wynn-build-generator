use std::{cmp::Ordering, collections::HashSet, rc::Rc};

use web_sys::{console, Event, HtmlDocument, HtmlInputElement, {wasm_bindgen::JsCast}};
use yew::{prelude::*, virtual_dom::AttrValue};
use gloo::{timers::callback::Timeout, utils::{document, window}};

mod autocomplete; mod item_input_list; mod weapon_input; mod build_calc; mod build_disp; mod atree_input; mod build_reqs_input; mod build_ordering;

use autocomplete::AutocompleteInput;
use item_input_list::ItemInput;
use weapon_input::WeaponInput;
use build_calc::BuildCalcComponent;
use atree_input::AtreeInput;
use build_reqs_input::BuildReqsInput;
use build_ordering::OptimizingStatInput;

use crate::{make_build, wynn_data::{atree::AtreeBuild, builder::WynnBuild, items::{self, Category, Tier, Type, WynnItem}, unhash_to_vec, unhash_val, url_hash_val, Class}, WynnEnum};
use crate::best_build_search::{BestBuildSearch, helper_enums::{CalcStat, SearchParam, SearchReq}};
#[derive(PartialEq, Clone, Default)]
pub enum Page{
    #[default]
    Input,
    Search,
    Settings
}
pub enum RootMsg{
    ItemsUpdate(usize, Vec<WynnItem>),
    AtreeUpdate(AtreeBuild),
    ReqsUpdate(Vec<(Ordering,SearchReq)>),
    OptimizingStatUpdate(SearchParam),
    RequestPage(Page),
    SwitchPage(Page),
    BestBuildsUpdate(Vec<WynnBuild>),
    Clear,
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
    min_reqs: Vec<SearchReq>,
    max_reqs: Vec<SearchReq>,
    optimizing_stat: SearchParam,
    atree: Rc<AtreeBuild>,
    res_builds: Vec<WynnBuild>,
    prev_search_data: Option<(u64, Vec<WynnBuild>)>,
    handle: Option<Timeout>,
    curr_page: Page
}
impl Component for RootComponent{
    type Message = RootMsg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let cookies = get_cookie_data();

        // loads inputted items from cookies
        let saved_items = items_from_hash(&get_cookie_val_from_tuples(&cookies, "InputtedItems").unwrap_or_default());
        let mut weapon = WynnItem::NULL;
        let mut items: [Vec<WynnItem>; 8] = Default::default();
        let mut ring2 = false;
        for itm in saved_items{
            if itm.is_null(){ring2 = true; continue}
            match itm.get_type(){
                Type::Helmet => items[0].push(itm),
                Type::Chestplate => items[1].push(itm),
                Type::Leggings => items[2].push(itm),
                Type::Boots => items[3].push(itm),
                Type::Ring => {items[if ring2{5}else{4}].push(itm)},
                Type::Bracelet => items[6].push(itm),
                Type::Necklace => items[7].push(itm),
                _ => weapon = itm
            }
        }

        // loads inputted search parameters (stat requirements and the mazimizing stat) from cookies
        let search_param_data = get_cookie_val_from_tuples(&cookies, "SearchParams").unwrap_or_default();
        let optimizing_stat = search_params_from_hash(search_param_data.get(0..3).unwrap_or_default());
        let max_min_indicies = (search_param_data.find(|c| c=='>').unwrap_or(0),search_param_data.find(|c| c=='<').unwrap_or(0));
        let min_reqs = search_reqs_from_hash(search_param_data.get((max_min_indicies.0+1)..max_min_indicies.1).unwrap_or_default());
        let max_reqs = search_reqs_from_hash(search_param_data.get((max_min_indicies.1+1)..).unwrap_or_default());

        // loads atree from cookies
        let atree_hash = get_cookie_val_from_tuples(&cookies, "Atree").unwrap_or_default();
        let atree = if !weapon.is_null(){
            match Class::try_from(weapon.get_type() as u8 - 7){
                Ok(c) => AtreeBuild::from_hash(&atree_hash, c),
                Err(_) => Default::default(),
            }
        }else{Default::default()};
        let atree_ref: Rc<AtreeBuild> = atree.into();

        // used to resume a prior search even if the window was closed
        let prev_search_data: Option<(u64, Vec<WynnBuild>)> = match get_cookie_val_from_tuples(&cookies, "CalcProgress"){Some(s) => if s.is_empty(){None} else {Some(build_calc_progress_from_hash(&s,atree_ref.clone()))}, None => None};
        
        RootComponent{items, weapon, atree: atree_ref, res_builds: Default::default(), handle: None, prev_search_data, 
        min_reqs, max_reqs, optimizing_stat: *optimizing_stat.first().unwrap_or(&SearchParam::Calc(CalcStat::MeleeHit)), curr_page: Page::Input}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            RootMsg::ItemsUpdate(type_id, itms) => {
                if type_id<8{
                    if self.items[type_id]==itms{return false}
                    self.items[type_id]=itms;
                }else{
                    self.weapon = match itms.first(){
                        Some(itm) => if self.weapon==*itm{return false}else{*itm},
                        None => WynnItem::NULL,
                    }
                }
                self.prev_search_data=None;
                true
            },
            RootMsg::ReqsUpdate(reqs) => {
                let mut min = Vec::new();
                let mut max = Vec::new();
                for (ordering, req) in reqs.into_iter(){
                    match ordering{
                        Ordering::Greater => min.push(req),
                        _ => max.push(req)
                    }
                }
                if self.min_reqs==min && self.max_reqs==max{
                    return false
                }
                self.min_reqs=min;
                self.max_reqs=max;
                self.prev_search_data=None;
                true
            },
            RootMsg::OptimizingStatUpdate(stat) => {
                if self.optimizing_stat==stat{return false}
                self.optimizing_stat=stat;
                self.prev_search_data=None;
                true
            },
            RootMsg::AtreeUpdate(atree) => {
                if atree==*self.atree{return false}
                self.atree=atree.into();

                self.prev_search_data=None;
                true
            },
            RootMsg::BestBuildsUpdate(blds) => {
                self.prev_search_data=Some((u64::MAX, blds));
                false
            },
            RootMsg::RequestPage(to) => {
                let link = ctx.link().clone();
                if self.curr_page==to{
                    return false;
                }
                match to{
                    Page::Input => {
                        window().scroll_to_with_x_and_y(0.0, 0.0);
                        self.handle = Some(Timeout::new(500, move || link.send_message(RootMsg::SwitchPage(to))));
                    }
                    Page::Search => {
                        if self.items.iter().all(|v| !v.is_empty()) && !self.weapon.is_null(){
                            window().scroll_to_with_x_and_y(0.0, 0.0);
                            self.save_cookies();
                            self.handle = Some(Timeout::new(500, move || link.send_message(RootMsg::SwitchPage(to))));    
                        }
                    }
                    Page::Settings => {
                        window().scroll_to_with_x_and_y(0.0, 0.0);
                        self.handle = Some(Timeout::new(500, move || link.send_message(RootMsg::SwitchPage(to))));
                    }
                }
                true
            },
            RootMsg::SwitchPage(to) => {
                self.curr_page=to;
                self.handle=None;
                true
            },
            RootMsg::Clear => {
                self.items=Default::default();
                self.weapon=WynnItem::NULL;
                self.atree=Default::default();
                self.res_builds=Default::default();
                self.prev_search_data=None;
                self.min_reqs=Vec::new();
                self.max_reqs=Vec::new();
                self.optimizing_stat=SearchParam::Calc(CalcStat::MeleeHit);
                false
            }
            RootMsg::None => false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let curr_page_temp = self.curr_page.clone();
        let page = match self.curr_page{
            Page::Input => html!{
                <div class={format!("page {}",if self.handle.is_none(){""}else{"transferring-page"})}>
                    <div class="const-build-data">
                        <WeaponInput on_leave = {link.callback(|itm| RootMsg::ItemsUpdate(8, match itm{Some((_ty, itm)) => vec![itm], None => Vec::new()}))} start_value={self.weapon.clone()}/>
                        <AtreeInput class={if self.weapon.is_null(){None}else{match Class::try_from(self.weapon.get_type() as u8 - 7){Ok(c) => Some(c), Err(_) => None}}} on_leave={link.callback(|a| RootMsg::AtreeUpdate(a))} start_value={&self.atree}/>
                    </div>
                    <div class="inputs-grid">
                        <ItemInput item_type = {Type::Helmet} on_leave = {link.callback(|(_, itms)| RootMsg::ItemsUpdate(0, itms))} start_value={self.items[0].clone()}/>
                        <ItemInput item_type = {Type::Chestplate} on_leave = {link.callback(|(_, itms)| RootMsg::ItemsUpdate(1, itms))} start_value={self.items[1].clone()}/>
                        <ItemInput item_type = {Type::Leggings} on_leave = {link.callback(|(_, itms)| RootMsg::ItemsUpdate(2, itms))} start_value={self.items[2].clone()}/>
                        <ItemInput item_type = {Type::Boots} on_leave = {link.callback(|(_, itms)| RootMsg::ItemsUpdate(3, itms))} start_value={self.items[3].clone()}/>
                        <ItemInput item_type = {Type::Ring} on_leave = {link.callback(|(_, itms)| RootMsg::ItemsUpdate(4, itms))} start_value={self.items[4].clone()}/>
                        <ItemInput item_type = {Type::Ring} on_leave = {link.callback(|(_, itms)| RootMsg::ItemsUpdate(5, itms))} start_value={self.items[5].clone()}/>
                        <ItemInput item_type = {Type::Bracelet} on_leave = {link.callback(|(_, itms)| RootMsg::ItemsUpdate(6, itms))} start_value={self.items[6].clone()}/>
                        <ItemInput item_type = {Type::Necklace} on_leave = {link.callback(|(_, itms)| RootMsg::ItemsUpdate(7, itms))} start_value={self.items[7].clone()}/>
                    </div>
                    <div class="search-params">
                        <BuildReqsInput on_leave = {link.callback(|v| RootMsg::ReqsUpdate(v))} start_value={(self.min_reqs.clone(),self.max_reqs.clone())}/>
                        <OptimizingStatInput on_leave={link.callback(|stat| RootMsg::OptimizingStatUpdate(stat))} start_value={self.optimizing_stat}/>
                        <div class="gen-button-area">
                        <div class="gen-button-wrapper">
                            <button onclick={link.callback(|_| RootMsg::RequestPage(Page::Search))}>{if self.prev_search_data.is_some(){"Continue"}else{"Generate!"}}</button>
                        </div>
                        </div>
                    </div>
                    // <div class="bottom">
                    // {format!("{:#?} {:#?} {:#?} {:#?} {:#?}",self.items,self.weapon, self.min_reqs,self.max_reqs,self.optimizing_stat)}
                    // </div>
                    <div class="bottom">
                    {format!("{:#?} {:#?} {}",self.atree.get_hash(),self.atree.get_melee_mults(),self.atree.get_cost(0))}
                    </div>
                </div>
            },
            Page::Search => html!{
                <div class={format!("page {}",if self.handle.is_none(){""}else{"transferring-page"})}>
                    if self.prev_search_data.is_some(){
                        <BuildCalcComponent items={self.items.clone()} weapon={self.weapon.clone()} atree={&self.atree} min_reqs={self.min_reqs.clone()} max_reqs={self.max_reqs.clone()} optimizing_stat={self.optimizing_stat.clone()} start_value={self.prev_search_data.clone().unwrap()} on_finish={link.callback(|blds| RootMsg::BestBuildsUpdate(blds))}/>
                    }else{
                        <BuildCalcComponent items={self.items.clone()} weapon={self.weapon.clone()} atree={&self.atree} min_reqs={self.min_reqs.clone()} max_reqs={self.max_reqs.clone()} optimizing_stat={self.optimizing_stat.clone()} on_finish={link.callback(|blds| RootMsg::BestBuildsUpdate(blds))}/>
                    }
                </div>
            },
            Page::Settings => html!{
                <div class={format!("page {}",if self.handle.is_none(){""}else{"transferring-page"})}>
                    <p>{"todo!(\"make the settings page\")"}</p>
                    <button onclick={link.callback(|_| RootMsg::Clear)}>{"Clear Inputs"}</button>
                </div>
            }
        };
        html!{
            <>
                <div class="header">
                    <h1 onclick={link.callback(|_| RootMsg::RequestPage(Page::Input))}>{"Wynncraft Build Generator"}</h1>
                    if self.curr_page!=Page::Search{
                        <button class="settings-button" onclick={link.callback(move |_| RootMsg::RequestPage(if curr_page_temp==Page::Settings{Page::Input}else{Page::Settings}))}><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#000000" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/><circle cx="12" cy="12" r="3"/></svg></button>
                    }
                </div>
                {page}
            </>
        }
    }
}
impl RootComponent{
    fn save_cookies(&self){
        let doc = document().unchecked_into::<HtmlDocument>();
        let _ = doc.set_cookie(&format!("InputtedItems={}; expires=Tue, 19 Jan 2038 03:14:07 UTC;",self.hash_inputted_items()));
        let _ = doc.set_cookie(&format!("SearchParams={}; expires=Tue, 19 Jan 2038 03:14:07 UTC;",self.hash_stats()));
        let _ = doc.set_cookie(&format!("Atree={}; expires=Tue, 19 Jan 2038 03:14:07 UTC;",self.atree.get_hash()));
    }
    fn hash_inputted_items(&self) -> String{
        let mut res = String::new();
        if !self.weapon.is_null(){ res+=&self.weapon.get_hash() }
        let mut ring2_null_gap = false; // adds a NULL item to signify if an item belongs to ring1 or ring2
        for i in 0..8{
            if i==5 && !ring2_null_gap{ring2_null_gap=true; res+="zzz"}
            for itm in self.items[i].iter(){
                res+=&itm.get_hash();
            }
        }
        res
    }
    fn hash_stats(&self) -> String{
        let mut res = url_hash_val(self.optimizing_stat.usize_id() as u32, 2);
        res+=">";
        for req in self.min_reqs.iter(){
            res+=&format!("{}{}",url_hash_val(req.usize_id() as u32, 2),match req{
                SearchReq::Stat(_, v) => url_hash_val(v.clone(),6),
                SearchReq::Calc(_, v) => url_hash_val(v.to_bits(),6),
            });
        }
        res+="<";
        for req in self.max_reqs.iter(){
            res+=&format!("{}{}",url_hash_val(req.usize_id() as u32, 2),match req{
                SearchReq::Stat(_, v) => url_hash_val(v.clone(),6),
                SearchReq::Calc(_, v) => url_hash_val(v.to_bits(),6),
            });
        }
        res
    }
}

fn items_from_hash(hashes: &str) -> Vec<WynnItem>{
    unhash_to_vec(hashes, 3, |hash| WynnItem::from_hash(hash).unwrap_or(WynnItem::NULL))
}

fn search_params_from_hash(hashes: &str) -> Vec<SearchParam>{
    unhash_to_vec(hashes, 2, |hash| SearchParam::from_usize(unhash_val(hash)))
}

fn search_reqs_from_hash(hashes: &str) -> Vec<SearchReq>{
    unhash_to_vec(hashes, 8, |hash| {
        let id = unhash_val(hash.get(0..2).unwrap_or("00"));
        if id < CalcStat::NUM_VARIENTS{
            SearchReq::from_usize_and_f32(id, f32::from_bits(unhash_val(hash.get(2..).unwrap_or("000000"))))
        }else{
            SearchReq::from_usize_and_i32(unhash_val(hash.get(0..2).unwrap_or("00")), unhash_val(hash.get(2..).unwrap_or("000000")))
        }
    })
}

fn build_calc_progress_from_hash(hashes: &str, atree: Rc<AtreeBuild>) -> (u64, Vec<WynnBuild>){
    let mut builds = hashes.split(',');
    let curr_comb: u64 = builds.next().unwrap_or("0").parse().unwrap_or(0);
    (curr_comb, builds.map(|build_hash| WynnBuild::from_hash(build_hash,atree.clone()).unwrap_or(make_build!((OAK_WOOD_SPEAR)).unwrap())).collect::<Vec<WynnBuild>>())
}

fn get_cookie_data() -> Vec<(String, String)>{
    let get_cookies = document().unchecked_into::<HtmlDocument>().cookie().unwrap_or(String::from(""));
    get_cookies.split("; ").map(|s| match s.split_once('='){Some((a,b)) => (a.to_string(),b.to_string()), None => (s.to_string(),String::new())}).collect::<Vec<(String, String)>>()
}

fn get_cookie_val_from_tuples(data: &[(String, String)], key: &str) -> Option<String>{
    match data.iter().find(|ck| ck.0==key){
        Some((_, value)) => Some(value.clone()),
        None => None
    }
}

// Rename and provide styling for all 'stats' displayed on the website

// Hp,EDef,TDef,WDef,FDef,ADef,DamMult,DefMult,AtkTier,DamRaw,ESteal,Expd,HealPct,HpBonus,HprPct,HprRaw,Jh,Kb,Lb,Ls,MdPct,MdRaw,Mr,Ms,Poison,Ref,SdPct,SdRaw,SlowEnemy,SpPct1,SpPct2,SpPct3,SpPct4,SpRaw1,SpRaw2,SpRaw3,SpRaw4,SpRegen,Spd,Sprint,SprintReg,Thorns,WeakenEnemy,Xpb,NDamPct,EDamPct,TDamPct,WDamPct,FDamPct,ADamPct,RDamPct,EDefPct,TDefPct,WDefPct,FDefPct,ADefPct,RDefPct,NSdRaw,ESdRaw,TSdRaw,WSdRaw,FSdRaw,ASdRaw,RSdRaw,NSdPct,ESdPct,TSdPct,WSdPct,FSdPct,ASdPct,RSdPct,NMdRaw,EMdRaw,TMdRaw,WMdRaw,FMdRaw,AMdRaw,RMdRaw,NDamRaw,EDamRaw,TDamRaw,WDamRaw,FDamRaw,ADamRaw,RDamRaw,NMdPct,EMdPct,TMdPct,WMdPct,FMdPct,AMdPct,RMdPct,NAddDam,EAddDam,TAddDam,WAddDam,FAddDam,AAddDam

const STAT_RENAMES: &'static [&'static str; 58] = &["Melee Single Hit","Melee Dps","Base Spell Damage","Spell 1 Damage","Spell 2 Damage","Spell 3 Damage","Spell 4 Damage","Spell 1 / sec","Spell 2 / sec","Spell 3 / sec","Spell 4 / sec","Effective Mana Regen","Effective HP", "Effective HP Regen", "Health", "Earth Def", "Thunder Def", "Water Def", "Fire Def","Air Def","Damage Mult", "Defence Mult","Attack Spd", "Damage Raw", "Stealing", "Exploding", "Heal %", "Health Bonus", "Health Regen %", "Health Regen Raw", "Jump Height", "Knockback", "Loot Bonus", "Life Steal", "Melee Dam %", "Melee Dam Raw", "Mana Regen", "Mana Steal", "Poison", "Reflection", "Spell Damage %", "Spell Damage Raw", "Slow Enemy", "Spell 1 Cost %", "Spell 2 Cost %", "Spell 3 Cost %", "Spell 4 Cost %", "Spell 1 Cost Raw", "Spell 2 Cost Raw", "Spell 3 Cost Raw", "Spell 4 Cost Raw", "Soul Regen", "Speed", "Sprint", "Sprint Regen", "Thorns", "Weaken Enemy", "XP Bonus"];
const STAT_COLOR_CLASSES: &'static [&'static str; 58] = &["str","str","int","int","int","int","int","int","int","int","int","int","def","def","def", "str", "dex", "int", "def", "agi", "neut", "def", "dex", "neut", "neut", "neut", "def", "def", "def", "def", "agi", "agi", "neut", "def", "str", "str", "int", "int", "neut", "neut", "int", "int", "neut", "int", "int", "int", "int", "int", "int", "int", "int", "neut", "agi", "agi", "agi", "neut", "neut", "neut"];

