use std::{collections::BinaryHeap, rc::Rc};
use super::{*, items, items::{enums::*,WynnItem}, sets::*, atree::AtreeBuild, url_hash_val, I12x5, unhash_to_vec};
use crate::ref_irrelevent_struct_func;

/// Represents a wynncraft build and provides methods to access and calculate stats
/// 
/// Returns None if the provided items cannot be made into a build (due to a lack of skill points or invalid combination)
/// 
/// # Example
/// ```
/// let bld: Option<WynnBuild> = WynnBuild::make_from_names(&["Cumulonimbus", "Libra", "The Ephemeral", "Stardew", "Yang", "Intensity", "Succession", "Diamond Fusion Necklace", "Nirvana"], 200);
/// assert!(bld.is_none()); // this build requires 201 skill points to use (tomes)
/// 
/// let bld2: Option<WynnBuild> = WynnBuild::make_from_names(&["Cumulonimbus", "Libra", "The Ephemeral", "Stardew", "Yang", "Intensity", "Succession", "Diamond Fusion Necklace", "Nirvana"], 201);
/// assert!(bld2.is_some());
/// let bld2_unwrapped = bld2.unwrap();
/// assert!(bld2_unwrapped.get_stat(Atrs::Hp)+bld2_unwrapped.get_stat(Atrs::HpBonus), 10710);
/// ```
#[derive(Clone, PartialEq)]
pub struct WynnBuild {
    stats: [i32; Atrs::NUM_STATS],
    dams: [(u32, u32); 6],
    pub skills: I12x5,
    assigned_skills: I12x5,
    free_sps: i32,
    items: [WynnItem; 9],
    lvl: i32,
    atree: Rc<AtreeBuild>
}
impl WynnBuild {
    /// Gets a specific identification stat from this build. 
    pub fn get_stat(&self, atr: Atrs) -> i32 {
        return self.stats[atr as usize-Atrs::NUM_NON_STATS];
    }

    pub fn total_health(&self) -> f32{
        (self.get_stat(Atrs::Hp)+self.get_stat(Atrs::HpBonus)+get_health_at_level(self.lvl)) as f32
    }

    /// Calculates the ehp of this build
    pub fn calc_ehp(&self) -> f32{
        let hp = self.total_health();
        let class_mult: f32 = match self.items.last().unwrap_or(&WynnItem::OAK_WOOD_DAGGER).get_type(){
            Type::Wand => 5.0/6.0,
            Type::Relik => 5.0/7.0,
            Type::Bow => 10.0/13.0,
            _ => 1.0
        };
        (hp/(0.1*skill_to_pct(Skill::Agi, self.skills.get(Skill::Agi))+(1.0-skill_to_pct(Skill::Agi, self.skills.get(Skill::Agi)))*(1.0-skill_to_pct(Skill::Def, self.skills.get(Skill::Def)))))*class_mult
    }

    /// Calculates maximum ehp of the build by dumping all free skill points into defence, or into agility if def < 0
    /// 
    /// TODO: Instead, this function should determine the exact def and agi to maximize ehp, but that's more work and would run slower...
    pub fn calc_max_ehp(&self) -> f32{
        let hp = self.total_health();
        let class_mult: f32 = match self.items.last().unwrap().get_type(){
            Type::Wand => 5.0/6.0,
            Type::Relik => 5.0/7.0,
            Type::Bow => 10.0/13.0,
            _ => 1.0
        };
        if self.skills.get::<_,i32>(Skill::Def)<0{
            let mut agi = self.calc_max_skill(Skill::Agi, self.free_sps);
            let def = self.skills.get::<_,i32>(Skill::Def)+self.calc_max_skill(Skill::Def, self.free_sps-agi);
            agi+=self.skills.get::<_,i32>(Skill::Agi);
            (hp/(0.1*skill_to_pct(Skill::Agi, agi)+(1.0-skill_to_pct(Skill::Agi, agi))*(1.0-skill_to_pct(Skill::Def, def))))*class_mult
        }else{
            let mut def = self.calc_max_skill(Skill::Def, self.free_sps);
            let agi = self.skills.get::<_,i32>(Skill::Agi)+self.calc_max_skill(Skill::Agi, self.free_sps-def);
            def+=self.skills.get::<_,i32>(Skill::Def);
            (hp/(0.1*skill_to_pct(Skill::Agi, agi)+(1.0-skill_to_pct(Skill::Agi, agi))*(1.0-skill_to_pct(Skill::Def, def))))*class_mult
        }
    }

    pub fn calc_ehpr(&self) -> f32{
        let hpr = self.get_stat(Atrs::HprRaw) as f32 * ((100+self.get_stat(Atrs::HprPct)) as f32);
        let class_mult: f32 = match self.items.last().unwrap_or(&WynnItem::OAK_WOOD_DAGGER).get_type(){
            Type::Wand => 5.0/6.0,
            Type::Relik => 5.0/7.0,
            Type::Bow => 10.0/13.0,
            _ => 1.0
        };
        (hpr/(0.1*skill_to_pct(Skill::Agi, self.skills.get(Skill::Agi))+(1.0-skill_to_pct(Skill::Agi, self.skills.get(Skill::Agi)))*(1.0-skill_to_pct(Skill::Def, self.skills.get(Skill::Def)))))*class_mult
    }
    
    /// Calculates the maximum number of skill points that can be ***assigned*** to a single skill. 
    /// <br>Remember to add the result of this to the current amount of skillpoints of that skill.
    fn calc_max_skill(&self, skill: Skill, free_sps: i32) -> i32{
        ((100-self.assigned_skills.get::<_,i32>(skill)).clamp(0,free_sps)).min(150)
    }

    /// Calculates the average melee damage
    /// 
    /// Note this automatically puts extra skill points into strength, then dexterity (TODO: add separate function for this option)
    pub fn calc_melee_dam(&self, use_atk_spd: bool) -> f32{
        // let mut avg = (0.0,0.0);
        let m: DamageData = DamageData::melee(self,false);
        self.calc_dam(m, false,self.atree.get_melee_mults(),1) * if use_atk_spd{atk_spd_mult(self.overall_atk_spd())} else {1.0}
    }

    /// Calculates the average spell damage
    /// 
    /// `spell`: calc spell damage for spell 0-3
    /// 
    /// Note this automatically puts extra skill points into strength, then dexterity (TODO: add separate function for this option)
    pub fn calc_spell_dam(&self, spell: usize) -> f32{
        // let mut avg = (0.0,0.0);
        let s: DamageData = DamageData::spell(self);
        self.calc_dam(s, true, if spell<=3{self.atree.get_spell_mults(spell)}else{[1.0,0.0,0.0,0.0,0.0,0.0]}, 1)
    }

    /// calculates the average damage of the build given some damage data 
    /// copied from https://github.com/wynnbuilder/wynnbuilder.github.io/blob/master/js/damage_calc.js and thrown into symbolab to compress,
    fn calc_dam(&self, d: DamageData, sp_dam: bool, mults: [f32; 6], num_hits: i32) -> f32{
        let mut avg = (0.0,0.0);
        let atk_spd_mul = if sp_dam {atk_spd_mult(self.weapon_atk_spd())} else {1.0};
        let mults_total: f32 = mults.iter().sum();
        let add_dam = [(0.0,0.0),(0.0,0.0),(0.0,0.0),(0.0,0.0),(0.0,0.0),(0.0,0.0)]; // todo: atree ele mastery
        for i in 0_usize..6{
            let mut pct_bonus = 1.0+d.pct+d.ele_dam_pcts[i];
            let raw = (d.raw*(d.dams[i].0/d.total_dam.0)+d.ele_dam_raws[i], d.raw*(d.dams[i].1/d.total_dam.1)+d.ele_dam_raws[i]);
            let mut weap_dam = (d.dams[i].0*mults[0],d.dams[i].1*mults[0]);
            if i>0{ // adds elem damage
                weap_dam.0+=mults[i]*d.total_dam.0;
                weap_dam.1+=mults[i]*d.total_dam.1;
                pct_bonus+=d.ele_dam_pcts[6]+d.skill_dam_bonus[i-1];
            }
            let mult = 1.0+d.skill_dam_bonus[0]+d.skill_dam_bonus[1];
            avg.0+=(((weap_dam.0*atk_spd_mul+add_dam[i].0)*pct_bonus+raw.0*mults_total)*mult).max(0.0);
            avg.1+=(((weap_dam.1*atk_spd_mul+add_dam[i].1)*pct_bonus+raw.1*mults_total)*mult).max(0.0);
        }
        num_hits as f32*(avg.0+avg.1)/2.0
    }

    /// Returns a splice of length len containing the ids starting from the given id (ie, `ids_splice(Atrs::EDamPct, 6)` returns all the **elemental damage percents** (including rainbow at index 6))
    /// You should make sure you know the order of the Atrs enum to use this function
    fn ids_splice(&self, start: Atrs, len: usize) -> &[i32]{
        let from = start as usize - Atrs::NUM_NON_STATS;
        &self.stats[from..from+len]
    }

    pub fn overall_atk_spd(&self) -> AtkSpd{
        AtkSpd::try_from((self.items.last().unwrap_or(&WynnItem::NULL).atk_spd() as i32 + self.get_stat(Atrs::AtkTier)).clamp(0,6) as u32).unwrap_or(AtkSpd::Normal)
    }

    pub fn weapon_atk_spd(&self) -> AtkSpd{
        self.items.last().unwrap_or(&WynnItem::NULL).atk_spd()
    }

    /// Calculates the mana cost of a spell
    /// 
    /// `spell`: number 0 to 3
    pub fn get_spell_cost(&self, spell: usize) -> f32{
        ((self.atree.get_cost(spell) as f32 * (1.0-skill_to_pct(Skill::Int, self.skills.get(Skill::Int))) + self.stats[Atrs::SpRaw1 as usize - Atrs::NUM_NON_STATS + spell] as f32) * (100+self.stats[Atrs::SpPct1 as usize - Atrs::NUM_NON_STATS + spell]) as f32).round() / 100.0
    }

    /// Calculates the number of a specific spell that can be cast per second (ignoring spam cost increases)
    /// 
    /// TODO: major ids not included (ie, this ignores transcendence)
    /// 
    /// `spell`: number 0 to 3<br>
    /// `mana_steal`: true to allow mana steal, false to only use mana regen
    pub fn spell_per_second(&self, spell: usize, mana_steal: bool) -> f32{
        (self.get_stat(Atrs::Mr) as f32 / 5.0 + if mana_steal{self.get_stat(Atrs::Ms) as f32 / 3.0}else{0.0}) / self.get_spell_cost(spell)
    }

    /// Calculates the effective mana regen of this build (mana regen * int cost reduction)
    pub fn calc_emr(&self) -> f32{
        self.get_stat(Atrs::Mr) as f32 / 5.0 / (1.0-skill_to_pct(Skill::Int, self.skills.get(Skill::Int)))
    }

    /// Generates the hash of this build, which can be used to save this build in a text format
    /// 
    /// Use wynnbuilder_hash to get the hash used by wynnbuilder's URL sharing system
    pub fn generate_hash(&self) -> String {
        self.items.iter().enumerate().map(|(t, item)| {if item.is_null() {url_hash_val(10000 + t as i32, 3)} else {item.get_hash()}
            }).collect::<String>() + &self.skills.iter().map(|s: i32| url_hash_val(s, 2)).collect::<String>()+&url_hash_val(self.lvl, 2)+"000000"+"z0z0+0+0+0+0-1T"+&self.atree.get_hash()
    }

    /// Used to transform this build into the hash used by wynnbuilder's URL sharing system
    /// 
    /// (this is broken, idk how wynnbuilder's atree hash works)
    pub fn wynnbuilder_hash(&self) -> String{
        self.items.iter().enumerate().map(|(t, item)| {if item.is_null() {url_hash_val(10000 + t as i32, 3)} else {item.get_hash()}
            }).collect::<String>() + &self.skills.iter().map(|s: i32| url_hash_val(s, 2)).collect::<String>()+&url_hash_val(self.lvl, 2)+"000000"+"z0z0+0+0+0+0-1T"+&self.atree.get_wynnbuilder_hash()
    }

    /// Makes a build from a hash, such as from a wynnbuilder link. 
    /// 
    /// The hash should be of the form `hashed items` `hashed skills` `hashed level` `powders` `tomes (ignored)`
    /// 
    /// The atree should be manually unhashed and provided (this prevents duplicate atrees from being generated for every single wynnbuild, which would eat excessive memory)
    /// 
    /// note that this may falsely return None if a build cannot be made without extra skill points (ie, from tomes)
    pub fn from_hash(hash: &str, atree: Rc<AtreeBuild>) -> Option<Self> {
        match <WynnBuild as MakeBuild<WynnItem>>::make(&unhash_to_vec(hash.get(..27).unwrap_or("2SG2SH2SI2SJ2SK2SL2SM2SN0Vv"), 3, |hash| WynnItem::from_hash(hash).unwrap_or(WynnItem::NULL)),
        unhash_val(hash.get(37..39).unwrap_or("1g")),
        I12x5::ZERO,
        atree
        ){
            Some(mut v) => {let temp = I12x5::from(unhash_to_vec(hash.get(27..37).unwrap_or("0000000000"), 2, |hash| unhash_val::<i32>(hash)).as_slice()); v.assigned_skills += temp-v.skills; v.free_sps=get_spts_at_level(v.lvl)-v.assigned_skills.sum(); v.skills=temp; Some(v)},
            None => None
        }
    }
    
    /// Generates a string containing all the names of items in this build, separated by commas. 
    pub fn item_names(&self) -> String{
        self.items.iter().enumerate().map(|(t, item)| {if item.is_null() {"null ".to_string()} else {item.name().to_string()+", "}}).collect::<String>()
    }

    pub fn iter_items(&self) -> std::slice::Iter<'_, WynnItem>{
        self.items.iter()
    }

    /// Gets an item from the build using an index which returns items in the following order:<br>
    /// [Helmet, Chestplate, Leggings, Boots, Ring1, Ring2, Bracelet, Necklace, Weapon]
    pub fn get_item(&self, type_idx: usize) -> WynnItem{
        self.items[type_idx]
    }
}

/// generates and stores all data related to damage. 
/// <br><br>note that this will automatically put extra skillpoints to strength, or dex if no more can be put into str
/// <br>TODO: make this put extra spts into dex instead of str if #str < 0
#[derive(Clone)]
struct DamageData{
    /// elemental damage percents, including neutral at [0] and rainbow at [6]
    ele_dam_pcts: [f32; 7],
    /// raw elemental damages, including neutral at [0] and rainbow at [6]
    ele_dam_raws: [f32; 7],
    dams: [(f32,f32); 6],
    total_dam: (f32, f32),
    skill_dam_bonus: [f32; 5],
    raw: f32,
    pct: f32
}
impl DamageData{
    // TODO: lots of duplicate code here
    fn melee(bld: &WynnBuild, use_atk_spd: bool) -> Self{
        let dam_pcts_splice = bld.ids_splice(Atrs::NDamPct, 7);
        let dam_raws_splice = bld.ids_splice(Atrs::NDamRaw, 7);
        let mdam_pcts_splice = bld.ids_splice(Atrs::NMdPct,7);
        let mdam_raws_splice = bld.ids_splice(Atrs::NMdRaw,7);

        let mut dam_pcts: [f32; 7] = [0.0;7];
        let mut dam_raws: [f32; 7] = [0.0;7];
        let mut dams: [(f32,f32); 6] = [(0.0,0.0);6];
        let mut total_dam: (f32, f32) = (0.0,0.0);
        let mut skill_dam_bonus: [f32; 5] = [0.0; 5];
        // todo, if str<0, just put all extra skill points into dex
        let mut sp_bonus = [bld.calc_max_skill(Skill::Str, bld.free_sps), 0];
        sp_bonus[1] = bld.calc_max_skill(Skill::Dex, bld.free_sps-sp_bonus[0]);
        // sp_bonus = [0,0]; // TODO: option to turn off auto application of extra skill points
        for i in 0_usize..7{
            if i<6{
                dams[i]=(bld.dams[i].0 as f32, bld.dams[i].1 as f32);
                total_dam.0+=dams[i].0;
                total_dam.1+=dams[i].1;
                if i<5{
                    skill_dam_bonus[i]=skill_damage_mult(Skill::VARIENTS[i], if i<2{bld.skills.get::<_,i32>(i)+sp_bonus[i]} else {bld.skills.get::<_,i32>(i)});
                }
            }
            dam_pcts[i]=(dam_pcts_splice[i] + mdam_pcts_splice[i]) as f32 / 100.0;
            dam_raws[i]=(dam_raws_splice[i]+mdam_raws_splice[i]) as f32;
        }
        let melee_raw = bld.get_stat(Atrs::MdRaw) as f32 + bld.get_stat(Atrs::DamRaw) as f32;
        let melee_pct = bld.get_stat(Atrs::MdPct) as f32 / 100.0;
        Self{ele_dam_pcts: dam_pcts, ele_dam_raws: dam_raws, dams, total_dam, skill_dam_bonus, raw: melee_raw, pct: melee_pct}
    }
    fn spell(bld: &WynnBuild) -> Self{
        let dam_pcts_splice = bld.ids_splice(Atrs::NDamPct, 7);
        let dam_raws_splice = bld.ids_splice(Atrs::NDamRaw, 7);
        let sdam_pcts_splice = bld.ids_splice(Atrs::NSdPct,7);
        let sdam_raws_splice = bld.ids_splice(Atrs::NSdRaw,7);

        let mut dam_pcts: [f32; 7] = [0.0;7];
        let mut dam_raws: [f32; 7] = [0.0;7];
        let mut dams: [(f32,f32); 6] = [(0.0,0.0);6];
        let mut total_dam: (f32, f32) = (0.0,0.0);
        let mut skill_dam_bonus: [f32; 5] = [0.0; 5];
        let mut sp_bonus = [bld.calc_max_skill(Skill::Str, bld.free_sps), 0];
        sp_bonus[1] = bld.calc_max_skill(Skill::Dex, bld.free_sps-sp_bonus[0]);
        // let atk_spd_mul = atk_spd_mult(bld.atk_spd());
        sp_bonus = [0,0]; // add option
        for i in 0_usize..7{
            if i<6{
                dams[i]=(bld.dams[i].0 as f32, bld.dams[i].1 as f32);
                total_dam.0+=dams[i].0;
                total_dam.1+=dams[i].1;
                if i<5{
                    skill_dam_bonus[i]=skill_damage_mult(Skill::VARIENTS[i], if i<2{bld.skills.get::<_,i32>(i)+sp_bonus[i]}else{bld.skills.get::<_,i32>(i)});
                }
            }
            dam_pcts[i]=(dam_pcts_splice[i] + sdam_pcts_splice[i]) as f32 / 100.0;
            dam_raws[i]=(dam_raws_splice[i]+sdam_raws_splice[i]) as f32;
        }
        let spell_raw = bld.get_stat(Atrs::SdRaw) as f32 + bld.get_stat(Atrs::DamRaw) as f32;
        let spell_pct = bld.get_stat(Atrs::SdPct) as f32 / 100.0;
        Self{ele_dam_pcts: dam_pcts, ele_dam_raws: dam_raws, dams, total_dam, skill_dam_bonus, raw: spell_raw, pct: spell_pct}
    }
}

macro_rules! add_items(
    ($items: ident, $res: ident, $skill_data: ident $(,$deref: tt)?) => {
        for (s,n) in $skill_data.3{
            for (id,val) in super::sets::get_set_bonuses(s, n){
                if id as usize>=Atrs::NUM_NON_STATS{
                    $res.stats[id as usize - Atrs::NUM_NON_STATS] += val;
                }else if id==Atrs::FixID{ // for some reason i decided to define 'invalid set' as FixID
                    return None
                }
            }
        }
        for i in 0..$items.len() {
            if !$items[i].is_null() {
                $res.add_item($($deref)?$items[i])
            }
        }
    }
);

/// ```
/// fn make(items: &[WynnItem], lvl: i32, base_skills: I12x5, atree: Rc<AtreeBuild>, stats: &[i32; Atrs::NUM_STATS]) -> Option<WynnBuild>
/// ```
/// <hr>
/// Used to make a WynnBuild
/// 
/// This macro calls WynnBuild::make(), but uses defaults for succeeding values, allowing you to not include them if you want.<br>
/// By default: `lvl: 106, base_skills: I12x5::ZERO, atree: Default::default()`
/// 
/// # Important
/// This macro returns an `Option<WynnBuild>`, so make sure you unwrap() the result!
/// <br>Syntax errors for macros don't yet bubble to the macro's invocation site, so you may get compile errors without highlighting. 
/// 
/// # Examples
/// ```
/// // insert some items
/// let items: [WynnItem] = [items::with_name("Oak Wood Spear")];
/// 
/// // make build with [items], lvl: 100, and 3 skill points into dex
/// let my_build: Option<WynnBuild> = make_build!(&items, 100, I12x5::from((0,3,0,0,0)));
/// 
/// // Alternatively, you can make a build using the const item names defined in WynnItem
/// let build = make_build!((HARD_LEATHER_CAP, OAK_WOOD_SPEAR), 100);
///
/// // Or you can do this? idk why i added this feature tbh, kinda weird, might remove it...
/// let build2 = make_build!({Hard Leather Cap, Oak Wood Spear}, 100);
/// ```
#[macro_export]
macro_rules! make_build(
    (($($item_name: ident),+) $(,$arg: expr)*) => {
        make_build!(&[$($crate::items::WynnItem::$item_name,)+] $(,$arg)*)
    };
    ({$($($item_name: ident)+),+} $(,$arg: expr)*) => {
        make_build!(&[$(crate::item_from_tt!($($item_name)+),)+] $(,$arg)*)
    };
    ($items: expr) => {
        <$crate::builder::WynnBuild as $crate::builder::MakeBuild<_>>::make($items,106,$crate::I12x5::ZERO,$crate::atree::AtreeBuild::default().into())
    };
    ($items: expr, $lvl: expr) => {
        <$crate::builder::WynnBuild as $crate::builder::MakeBuild<_>>::make($items,$lvl,$crate::I12x5::ZERO,$crate::atree::AtreeBuild::default().into())
    };
    ($items: expr, $lvl: expr, $base_skills: expr) => {
        <$crate::builder::WynnBuild as $crate::builder::MakeBuild<_>>::make($items,$lvl,$base_skills,$crate::atree::AtreeBuild::default().into())
    };
    ($items: expr, $lvl: expr, $base_skills: expr, $atree: expr) => {
        <$crate::builder::WynnBuild as $crate::builder::MakeBuild<_>>::make($items,$lvl,$base_skills,$atree)
    };
    ($items: expr, $lvl: expr, $base_skills: expr, $atree: expr, $stats: expr) => {
        <$crate::builder::WynnBuild as $crate::builder::MakeBuildWStats<_>>::make_w_stats($items,$lvl,$base_skills,$atree, $stats)
    };
);

ref_irrelevent_struct_func!(WynnBuild, pub MakeBuild, 
    fn make(*items: &[WynnItem], lvl: i32, base_skills: I12x5, atree: Rc<AtreeBuild>) -> Option<WynnBuild>{
        match WynnBuild::skillpoint_setup(items, get_spts_at_level(lvl), base_skills) {
            Some(s) => {
                let mut res: WynnBuild = WynnBuild {
                    stats: [0; Atrs::NUM_STATS],
                    dams: [(0, 0); 6],
                    skills: s.1,
                    free_sps: s.0,
                    assigned_skills: s.2,
                    lvl,
                    items: [WynnItem::NULL; 9],
                    atree
                };
                res.stats[Atrs::Hp as usize - Atrs::NUM_NON_STATS] = get_health_at_level(lvl);
                add_items!(items, res, s);
                Some(res)
            }
            None => None,
        }
    }
);
impl MakeBuild<&str> for WynnBuild{
    fn make(item_names: &[&str], lvl: i32, base_skills: I12x5, atree: Rc<AtreeBuild>) -> Option<WynnBuild>{
        let items: Vec<WynnItem> = item_names.iter().map(|name| items::with_name(name).unwrap()).collect();
        <Self as MakeBuild<WynnItem>>::make(&items, lvl, base_skills, atree)
    }
}

ref_irrelevent_struct_func!(WynnBuild, pub MakeBuildWStats, 
    fn make_w_stats(*items: &[WynnItem], lvl: i32, base_skills: I12x5, atree: Rc<AtreeBuild>, stats: &[i32; Atrs::NUM_STATS]) -> Option<WynnBuild>{
        match WynnBuild::skillpoint_setup(items, get_spts_at_level(lvl), base_skills) {
            Some(s) => {
                let mut res: WynnBuild = WynnBuild {
                    stats: stats.clone(),
                    dams: [(0, 0); 6],
                    skills: s.1,
                    free_sps: s.0,
                    assigned_skills: s.2,
                    lvl,
                    items: [WynnItem::NULL; 9],
                    atree: atree
                };
                res.stats[Atrs::Hp as usize - Atrs::NUM_NON_STATS] += get_health_at_level(lvl);
                add_items!(items, res, s);
                Some(res)
            }
            None => None,
        }
    }
);

ref_irrelevent_struct_func!(WynnBuild, AddItem, 
    fn add_item(self: &mut Self, *item: WynnItem) {
        for (i, d) in item.iter_damages() {
            self.dams[i as usize].0 += d.0;
            self.dams[i as usize].1 += d.1;
        }
        for d in item.iter_ids() {
            self.stats[d.0 as usize - Atrs::NUM_NON_STATS] += d.1;
        }
        let type_usize = item.get_type() as usize;
        self.items[if type_usize>4 || type_usize==4 && !self.items[4].is_null() {(type_usize+1).min(8)} else {type_usize}] = item.clone();
    }
);

ref_irrelevent_struct_func!(WynnBuild, SkillpointSetup,
    // jank - i expect to find some example where this breaks but until then i'm using this solution because it's fast
    /// Sets up the skillpoints for this item (and counts number of set items because it's convenient), 
    /// returning `(free skillpoints: i32, skillpoints: I12x5, assigned_skillpoints: I12x5, num_sets: Vec<(Sets,usize)>)`
    fn skillpoint_setup(*items: &[WynnItem], extra_spts: i32, base_skills: I12x5) -> Option<(i32,I12x5,I12x5,Vec<(Sets,usize)>)>{
        let mut temp = items.to_vec();
        // sort items in the equip order which minimizes required skill points
        // todo: use a sorting network instead
        temp.sort_by(|a, b| 
            // weapons should always be considered last
            if a.get_category()==Category::Weapon{std::cmp::Ordering::Greater}
            else if b.get_category()==Category::Weapon{std::cmp::Ordering::Less}
            // if items don't provide any bonuses, just go in order of their highest skillpoint requirement 
            else if a.get_skill_bonuses()==I12x5::ZERO || b.get_skill_bonuses()==I12x5::ZERO
                {a.get_skill_reqs().get_max().cmp(&b.get_skill_reqs().get_max())}
            // compares items by the 'benefit' they provide, ie the item who's bonuses help achieve the other's requirements better. 
            else{
                // benefit item a's bonuses provide item b (if all b's reqs > a's reqs, this is simply sum of a's bonuses)
                let a_bonus = (a.get_skill_bonuses() & (a.get_skill_reqs()-b.get_skill_reqs()).mask_negs()).sum();
                // benefit item b's bonuses provide item a
                let b_bonus = (b.get_skill_bonuses() & (b.get_skill_reqs()-a.get_skill_reqs()).mask_negs()).sum();
                match b_bonus.cmp(&a_bonus){
                    std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                    std::cmp::Ordering::Equal => a.get_skill_reqs().get_max().cmp(&b.get_skill_reqs().get_max()),
                    std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                }
            }
        );
        let mut req = I12x5::fill_data(-1024);
        let mut skills = base_skills;
        let mut assigned_skills = I12x5::ZERO;
        let mut num_set: Vec<(Sets,usize)> = Vec::new();
        // goes in the equip order of items, uses necessary skillpoints to get each item, then adds it's bonuses
        for itm in temp{
            // println!("adding item {:#?}, skills: {}, assigned: {}",itm,skills,assigned_skills);
            req = req.max(itm.get_skill_reqs());
            // skillpoints required to add an item to the build
            let diff = (req-skills).get_pos();
            assigned_skills+=diff;
            if assigned_skills.sum()>extra_spts || assigned_skills!=assigned_skills.with_max(100){return None}
            // adds skills from set bonuses
            if itm.get_set()!=Sets::None{
                let itm_set = itm.get_set();
                match num_set.iter_mut().find(|(s,_)| *s==itm_set){
                    Some((_,n)) => {
                        skills+=super::sets::get_set_skill_bonuses(itm_set,*n+1)-super::sets::get_set_skill_bonuses(itm_set,*n);
                        *n+=1;
                    },
                    None => {
                        skills+=super::sets::get_set_skill_bonuses(itm_set,1);
                        num_set.push((itm_set,1));
                    }
                }
            }
            // adds item skill bonuses
            skills=(skills+diff+itm.get_skill_bonuses()).with_max(150);
        }
        Some((extra_spts-assigned_skills.sum(), skills.with_max(150), assigned_skills, num_set))
    }
);

// Deprecated methods
impl WynnBuild{
    #[deprecated(note="Use `Use `WynnBuild::make()` or make_build!() instead")]
    pub fn make_from_names(item_names: &[&str], free_sps: i32) -> Option<WynnBuild>{
        let items: Vec<WynnItem> = item_names.iter().map(|name| items::with_name(name).unwrap()).collect();
        Self::make_with_free_spts(&items, free_sps)
    }

    #[deprecated(note="Use `Use `WynnBuild::make()` or make_build!() instead")]
    pub fn make_with_free_spts(items: &[WynnItem], free_sps: i32) -> Option<WynnBuild> {
        match WynnBuild::skillpoint_setup(items, free_sps, I12x5::ZERO) {
            Some(s) => {
                let mut res: WynnBuild = WynnBuild {
                    stats: [0; Atrs::NUM_STATS],
                    dams: [(0, 0); 6],
                    skills: s.1,
                    free_sps: s.0,
                    assigned_skills: s.2,
                    lvl: 106,
                    items: [WynnItem::NULL; 9],
                    atree: AtreeBuild::default().into()
                };
                for (s,n) in s.3{
                    for (id,val) in super::sets::get_set_bonuses(s, n){
                        if id as usize>=Atrs::NUM_NON_STATS{
                            res.stats[id as usize - Atrs::NUM_NON_STATS] += val;
                        }else if id==Atrs::FixID{ // for some reason i decided to define 'invalid set' as FixID
                            return None
                        }
                    }
                }
                for i in 0..items.len() {
                    if !items[i].is_null() {
                        res.add_item(items[i])
                    }
                }
                Some(res)
            }
            None => None,
        }
    }
    
    // my first attempt at a 'fast' algorithmn for assigning skillpoints
    // this splits skill pt requirements into each of the 5 skills, then goes in order of least to greatest for each of the 5 skills
    // note this does not consider set bonuses because i hadn't coded that at the time
    #[deprecated]
    fn setup_skillpoints_with_base_skills(items: &[&WynnItem], extra_sps: i32, base_skills: &[i32; 5]) -> Option<(i32,[i32; 5],[i32;5])> {
        let mut v: [BinaryHeap<(i32, i32)>; 5] = [
            BinaryHeap::with_capacity(8),
            BinaryHeap::with_capacity(8),
            BinaryHeap::with_capacity(8),
            BinaryHeap::with_capacity(8),
            BinaryHeap::with_capacity(8),
        ];
        let mut extra_skill_pts = extra_sps;
        let mut skills = base_skills.clone();
        let mut assigned_skills = [0,0,0,0,0];
        let mut weapon_skill_data = [(0,0);5];
        for item in items {
            if item.is_null(){continue}
            if item.get_category()==Category::Weapon{
                for (skill, req, bonus) in item.iter_skills(){
                    weapon_skill_data[skill as usize] = (req,bonus);
                }
                continue
            }
            for (skill, req, bonus) in item.iter_skills(){
                if req>0{
                    v[skill as usize].push((-req,bonus))
                } else{
                    skills[skill as usize]+=bonus;
                }
            }
        }
        for skill_type in 0..5_usize {
            while !v[skill_type].is_empty() {
                let temp = v[skill_type].pop().unwrap();
                let diff = skills[skill_type] + temp.0;
                if diff < 0 {
                    extra_skill_pts += diff;
                    skills[skill_type] -= diff;
                    assigned_skills[skill_type]-=diff;
                }
                if extra_skill_pts < 0 || assigned_skills[skill_type]>100 {
                    return None;
                }
                skills[skill_type] = (skills[skill_type]+temp.1).min(150);
            }
            let weap_diff = skills[skill_type]-weapon_skill_data[skill_type].0;
            if weapon_skill_data[skill_type].0>0 && weap_diff<0{
                extra_skill_pts += weap_diff;
                skills[skill_type] -= weap_diff;
                assigned_skills[skill_type]-=weap_diff;
            }
            if extra_skill_pts < 0 || assigned_skills[skill_type]>100{
                return None;
            }
            skills[skill_type]=(skills[skill_type]+weapon_skill_data[skill_type].1).min(150);
        }
        Some((extra_skill_pts,skills,assigned_skills))
    }

    #[deprecated(note="Use `Use `WynnBuild::make()` or make_build!() instead")]
    pub fn make_with_base_skills(items: &[WynnItem], free_sps: i32, base_skills: I12x5) -> Option<WynnBuild> {
        match WynnBuild::skillpoint_setup(items, free_sps, base_skills) {
            Some(s) => {
                let mut res: WynnBuild = WynnBuild {
                    stats: [0; Atrs::NUM_STATS],
                    dams: [(0, 0); 6],
                    skills: s.1,
                    free_sps: s.0,
                    assigned_skills: s.2,
                    lvl: 106,
                    items: [WynnItem::NULL; 9],
                    atree: AtreeBuild::default().into()
                };
                for (s,n) in s.3{
                    for (id,val) in super::sets::get_set_bonuses(s, n){
                        if id as usize>=Atrs::NUM_NON_STATS{
                            res.stats[id as usize - Atrs::NUM_NON_STATS] += val;
                        }else if id==Atrs::FixID{ // for some reason i decided to define 'invalid set' as FixID
                            return None
                        }
                    }
                }
                // let mut num_of_sets: Vec<(Sets, u8)> = Vec::new();
                for i in 0..items.len() {
                    if !items[i].is_null() {
                        res.add_item(items[i])
                    }
                }
                Some(res)
            }
            None => None,
        }
    }
    #[deprecated(note="Use `Use `WynnBuild::make()` or make_build!() instead")]
    pub fn make_with_skills_and_stats(items: &[WynnItem], free_sps: i32, base_skills: I12x5, ids: &[i32; Atrs::NUM_STATS]) -> Option<WynnBuild> {
        match WynnBuild::skillpoint_setup(items, free_sps, base_skills) {
            Some(s) => {
                let mut res: WynnBuild = WynnBuild {
                    stats: ids.clone(),
                    dams: [(0, 0); 6],
                    skills: s.1,
                    free_sps: s.0,
                    assigned_skills: s.2,
                    lvl: 106,
                    items: [WynnItem::NULL; 9],
                    atree: AtreeBuild::default().into()
                };
                for (s,n) in s.3{
                    for (id,val) in super::sets::get_set_bonuses(s, n){
                        if id as usize>=Atrs::NUM_NON_STATS{
                            res.stats[id as usize - Atrs::NUM_NON_STATS] += val;
                        }else if id==Atrs::FixID{ // for some reason i decided to define 'invalid set' as FixID
                            return None
                        }
                    }
                }
                for i in 0..items.len() {
                    if !items[i].is_null() {
                        res.add_item(items[i])
                    }
                }
                Some(res)
            }
            None => None,
        }
    }

    // similar to 'setup_skillpoints_with_base_skills', this splits skillpoint requirement into each individual skill type
    // then, it simply goes in order of items with least requirements to greatest requirement and assigns necessary skillpoints for each skill type
    // this fails for certain builds, one example i found was elf set. 
    // i believe this has no false negatives but may have false positives. idk tho
    #[deprecated]
    fn skillpoint_setup_with_base_skills(items: &[WynnItem], extra_spts: i32, base_skills: I12x5) -> Option<(i32,I12x5,I12x5)> {
        let mut skills = base_skills;
        let mut assigned_skills = I12x5::default();
        let mut reqs = [I12x5::ZERO; 9];
        let mut bonuses = [I12x5::ZERO; 9];
        let mut set = [I12x5::ZERO; 9];
        let mut passed_weapon = 0;
        let mut set_counter: Vec<(Sets, I12x5)> = Vec::new();

        for (i,item) in items.iter().enumerate(){
            if item.get_category()==Category::Weapon{reqs[8]=item.get_skill_reqs();bonuses[8]=item.get_skill_bonuses();passed_weapon=1;continue}
            let itm_set = item.get_set();
            reqs[i-passed_weapon]=item.get_skill_reqs();
            bonuses[i-passed_weapon]=item.get_skill_bonuses();
            set[i-passed_weapon]=I12x5::filled(itm_set);
            if set_counter.iter().all(|(s,_)| *s!=itm_set){set_counter.push((itm_set,I12x5::ZERO))}
            for j in (0..(i-passed_weapon)).rev(){
                let diff = reqs[j]-reqs[j+1];
                let mask = diff.mask_pos();
                
                reqs[j+1]=reqs[j]-diff.get_negs();
                reqs[j]-=diff.get_pos();

                let temp = (bonuses[j+1] & mask) + (bonuses[j] & !mask);
                bonuses[j+1] = (bonuses[j+1] & !mask) + (bonuses[j] & mask);
                bonuses[j] = temp;

                let temp = (set[j+1] & mask) + (set[j] & !mask);
                set[j+1] = (set[j+1] & !mask) + (set[j] & mask);
                set[j]=temp;
            }
        }
        let mut set_sp_bonus = I12x5::ZERO;
        for i in 0..reqs.len(){
            let diff = (reqs[i]-skills-set_sp_bonus).get_pos();
            assigned_skills+=diff;
            if assigned_skills.sum()>extra_spts || assigned_skills!=assigned_skills.with_max(100){return None}
            if set[i]!=I12x5::ZERO{
                set_sp_bonus = I12x5::ZERO;
                for j in 0..set_counter.len(){
                    let checking_set = set_counter[j].0;
                    set_counter[j].1 += I12x5::fill_data(1) & set[i].mask_eq(I12x5::fill_data(checking_set as i32));
                    for skill in Skill::iter(){
                        let sp_bonus: i32 = super::sets::get_set_skill_bonuses(set_counter[j].0, set_counter[j].1.get(*skill)).get(*skill);
                        set_sp_bonus+=(*skill,sp_bonus);
                    }
                }
            }
            skills=(skills+bonuses[i]+diff).with_max(150);
        }
        Some((extra_spts-assigned_skills.sum(), (skills+set_sp_bonus).with_max(150), assigned_skills))
    }

    // these use 'skillpoint_setup_with_base_skills' for assigning skill points. 
    #[deprecated]
    pub fn from_names_test(item_names: &[&str], free_sps: i32) -> Option<WynnBuild>{
        let items: Vec<WynnItem> = item_names.iter().map(|name| items::with_name(name).unwrap()).collect();
        Self::from_test(&items, free_sps, I12x5::ZERO)
    }
    #[deprecated]
    pub fn from_test(items: &[WynnItem], free_sps: i32, base_skills: I12x5) -> Option<WynnBuild> {
        match WynnBuild::skillpoint_setup_with_base_skills(items, free_sps, base_skills) {
            Some(s) => {
                let mut res: WynnBuild = WynnBuild {
                    stats: [0; Atrs::NUM_STATS],
                    dams: [(0, 0); 6],
                    skills: s.1,
                    free_sps: s.0,
                    assigned_skills: s.2,
                    lvl: 106,
                    items: [WynnItem::NULL; 9],
                    atree: AtreeBuild::default().into()
                };
                // let mut num_of_sets: Vec<(Sets, u8)> = Vec::new();
                for i in 0..items.len() {
                    if !items[i].is_null() {
                        res.add_item(items[i])
                    }
                }
                Some(res)
            }
            None => None,
        }
    }

}
