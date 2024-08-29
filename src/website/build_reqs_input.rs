use std::{cmp::Ordering, rc::Rc};

use web_sys::HtmlInputElement;
use yew::prelude::*;
use gloo::timers::callback::Timeout;

use crate::wynn_data::{items::Atrs, WynnEnum};
use super::AutocompleteInput;

const NUM_CALC_STATS: usize = 5;

#[derive(Clone, Copy, PartialEq)]
pub enum StatReq{
    Stat(Atrs, i32),
    EhpReq(f32),
    Sp1PerSec(f32),
    Sp2PerSec(f32),
    Sp3PerSec(f32),
    Sp4PerSec(f32),
}
impl StatReq{
    pub fn stat_eq(&self, other: &Self) -> bool{
        match self{
            Self::Stat(a, _) => match other{Self::Stat(b, _) => a==b, _ => false}
            _ => std::mem::discriminant(self) == std::mem::discriminant(other)
        }
    }
    pub fn name_and_val(&self) -> (String, f32){
        match self{
            Self::Stat(a, v) => (a.to_string(), *v as f32),
            Self::EhpReq(v) => (String::from("Ehp"), *v),
            Self::Sp1PerSec(v) => (String::from("Sp1 / sec"),*v),
            Self::Sp2PerSec(v) => (String::from("Sp2 / sec"),*v),
            Self::Sp3PerSec(v) => (String::from("Sp3 / sec"),*v),
            Self::Sp4PerSec(v) => (String::from("Sp4 / sec"),*v),
        }
    }
    pub fn usize_id(&self) -> usize{
        match self{
            Self::Stat(a, _) => {let temp: usize = (*a).into(); NUM_CALC_STATS + temp - Atrs::NUM_NON_STATS},
            Self::EhpReq(_) => 0,
            Self::Sp1PerSec(_) => 1,
            Self::Sp2PerSec(_) => 2,
            Self::Sp3PerSec(_) => 3,
            Self::Sp4PerSec(_) => 4,
        }
    }
    pub fn from_usize_and_val(id: usize, val: f32) -> Self{
        match id{
            0 => StatReq::EhpReq(val), 
            1 => StatReq::Sp1PerSec(val), 
            2 => StatReq::Sp2PerSec(val), 
            3 => StatReq::Sp3PerSec(val), 
            4 => StatReq::Sp4PerSec(val), 
            _ => StatReq::Stat(Atrs::VARIENTS[id-NUM_CALC_STATS+Atrs::NUM_NON_STATS], val as i32)
        }
    }
}
impl ToString for StatReq{
    fn to_string(&self) -> String {
        match self{
            Self::Stat(a, v) => format!("{} : {}", a, v),
            Self::EhpReq(v) => format!("Ehp : {}", v),
            Self::Sp1PerSec(v) => format!("Sp1 / Sec : {}", v),
            Self::Sp2PerSec(v) => format!("Sp2 / Sec : {}", v),
            Self::Sp3PerSec(v) => format!("Sp3 / Sec : {}", v),
            Self::Sp4PerSec(v) => format!("Sp4 / Sec : {}", v),
        }
    }
}
impl std::fmt::Debug for StatReq{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.to_string())
    }
}

#[derive(Properties, PartialEq)]
pub struct BuildReqsProps{
    #[prop_or_default]
    /// Callback to retrieve stat requirements when this component loses focus
    pub on_leave: Callback<Vec<(Ordering,StatReq)>>,
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
    selection: Vec<(Ordering,StatReq)>,
    stat_names: Rc<Vec<String>>,
    stat_colors: Rc<Vec<String>>,
    curr_stat_input: Option<usize>,
    curr_num_input: String,
    curr_ord_input: Ordering,
    reset_input: bool,
    unfocus_handle: Option<Timeout>
}
impl BuildReqsInput{
    // Hp,EDef,TDef,WDef,FDef,ADef,DamMult,DefMult,AtkTier,DamRaw,ESteal,Expd,HealPct,HpBonus,HprPct,HprRaw,Jh,Kb,Lb,Ls,MdPct,MdRaw,Mr,Ms,Poison,Ref,SdPct,SdRaw,SlowEnemy,SpPct1,SpPct2,SpPct3,SpPct4,SpRaw1,SpRaw2,SpRaw3,SpRaw4,SpRegen,Spd,Sprint,SprintReg,Thorns,WeakenEnemy,Xpb,NDamPct,EDamPct,TDamPct,WDamPct,FDamPct,ADamPct,RDamPct,EDefPct,TDefPct,WDefPct,FDefPct,ADefPct,RDefPct,NSdRaw,ESdRaw,TSdRaw,WSdRaw,FSdRaw,ASdRaw,RSdRaw,NSdPct,ESdPct,TSdPct,WSdPct,FSdPct,ASdPct,RSdPct,NMdRaw,EMdRaw,TMdRaw,WMdRaw,FMdRaw,AMdRaw,RMdRaw,NDamRaw,EDamRaw,TDamRaw,WDamRaw,FDamRaw,ADamRaw,RDamRaw,NMdPct,EMdPct,TMdPct,WMdPct,FMdPct,AMdPct,RMdPct,NAddDam,EAddDam,TAddDam,WAddDam,FAddDam,AAddDam
    const STAT_NAMES: [&'static str; 49] = ["Effective HP","Spell 1 / sec","Spell 2 / sec","Spell 3 / sec","Spell 4 / sec", "Health", "Earth Def", "Thunder Def", "Water Def", "Fire Def","Air Def","Damage Mult", "Defence Mult","Attack Spd", "Damage Raw", "Stealing", "Exploding", "Heal %", "Health Bonus", "Health Regen %", "Health Regen Raw", "Jump Height", "Knockback", "Loot Bonus", "Life Steal", "Melee Dam %", "Melee Dam Raw", "Mana Regen", "Mana Steal", "Poison", "Reflection", "Spell Damage %", "Spell Damage Raw", "Slow Enemy", "Spell 1 Cost %", "Spell 2 Cost %", "Spell 3 Cost %", "Spell 4 Cost %", "Spell 1 Cost Raw", "Spell 2 Cost Raw", "Spell 3 Cost Raw", "Spell 4 Cost Raw", "Soul Regen", "Speed", "Sprint", "Sprint Regen", "Thorns", "Weaken Enemy", "XP Bonus"];
    const STAT_COLOR_CLASSES: [&'static str; 49] = ["def", "int", "int", "int", "int", "def", "str", "dex", "int", "def", "agi", "neut", "def", "dex", "neut", "neut", "neut", "def", "def", "def", "def", "agi", "agi", "neut", "def", "str", "str", "int", "int", "neut", "neut", "int", "int", "neut", "int", "int", "int", "int", "int", "int", "int", "int", "neut", "agi", "agi", "agi", "neut", "neut", "neut"];
}
impl Component for BuildReqsInput{
    type Message = BuildReqsMsg;

    type Properties = BuildReqsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let stat_names: Vec<String> = Self::STAT_NAMES.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let stat_colors = Self::STAT_COLOR_CLASSES.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        BuildReqsInput{focused: false, unfocus_handle: None, selection: Vec::new(), stat_names: stat_names.into(), stat_colors: stat_colors.into(), curr_stat_input: None, curr_num_input: String::new(), reset_input: false, curr_ord_input: Ordering::Greater}
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
                            let temp = StatReq::from_usize_and_val(id, num_val);
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
                        let temp = stat_req.name_and_val();
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
                    <AutocompleteInput class={format!("stat-input")} reset={self.reset_input} options = {&self.stat_names} on_leave = {link.callback(move |(op, _)| BuildReqsMsg::StatInput(op))} options_classes ={&self.stat_colors}/>
                    <button class="comparator" onclick = {link.callback(|_| BuildReqsMsg::OrdToggle)}>{">"}</button>
                    <input class="num-input" oninput={link.callback(|event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); BuildReqsMsg::NumInput(input.value())})} value = {self.curr_num_input.clone()}/>
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