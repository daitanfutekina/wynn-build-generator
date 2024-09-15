pub mod archer;pub mod assassin;pub mod mage;pub mod shaman;pub mod warrior;mod enums;mod spells;
use super::{WynnEnum,Class,WynnDataIter,parse_data_key,parse_data_k,parse_data_val,parse_data_uval,parse_data_u32,parse_data_i32,items::Atrs,url_hash_val,unhash_to_vec,unhash_val};
use enums::*;
use spells::Spell;
use std::marker::PhantomData;

struct AtreeItemData<T: ClassAtreeEnums>{
    name: &'static str,
    parents: &'static [u8],
    deps: &'static [u8],
    blockers: &'static [u8],
    props: &'static [u32],
    effects: &'static [&'static [u32]],
    data: &'static [u32],
    enum_id: T::Items
}
impl <T: ClassAtreeEnums> AtreeItemData<T>{
    const fn iter_parents(&self) -> IterAtreeItems<T::Items>{
        IterAtreeItems{data: self.parents, curr: 0, phantom: PhantomData}
    }
    const fn iter_deps(&self) -> IterAtreeItems<T::Items>{
        IterAtreeItems{data: self.deps, curr: 0, phantom: PhantomData}
    }
    const fn iter_blockers(&self) -> IterAtreeItems<T::Items>{
        IterAtreeItems{data: self.blockers, curr: 0, phantom: PhantomData}
    }
    const fn iter_effects(&self) -> IterAtreeEffects<T>{
        IterAtreeEffects{data: self.effects, curr: 0, phantom: PhantomData}
    }
}


pub struct AtreeItem<T: ClassAtreeEnums + 'static>{
    data: &'static AtreeItemData<T>,
    // some atree items modify properties of other atree items, and some majorids modify properties of atree items. 
    // thus a modifyable 'props' may be needed for atree items to fully function
    // i need to dig more into how the atree works to come up with a good final solution though
    // props: Vec<u32> 
}
impl <T: ClassAtreeEnums + 'static> Clone for AtreeItem<T>{
    fn clone(&self) -> Self {
        Self { data: self.data, } // props: self.props.clone()
    }
}
macro_rules! impl_atreeitemtrait(
    ($cls: ident, $mod_name: ident, $enum_name: ident) => {
        struct $enum_name{}
        impl ClassAtreeEnums for $enum_name{
            const CLASS: Class = Class::$cls;
            type Items = $mod_name::AtreeItems;
            type Prop = $mod_name::Prop;
            type Spell = $mod_name::Spell;
            type SpellPart = $mod_name::SpellPart;
        }
        impl From<$mod_name::AtreeItems> for AtreeItem<$enum_name>{
            fn from(value: $mod_name::AtreeItems) -> Self {
                Self{ data: $mod_name::atree_data::ATREE_DATA[value as usize], } // props: Vec::new()
            }
        }
        impl std::convert::From<&[$mod_name::AtreeItems]> for AtreeBuild{
            /// Creates an AtreeBuild from an array of AtreeItems enums
            /// 
            /// This is the standard way of creating AtreeBuilds
            fn from(value: &[$mod_name::AtreeItems]) -> Self {
                let mut temp_items_ids = 0;
                for itm in value{
                    temp_items_ids |= 1<<*itm as u128;
                }
                AtreeBuild::from((Class::$cls, temp_items_ids))
            }
        }
        impl std::iter::FromIterator<$mod_name::AtreeItems> for AtreeBuild{
            /// Used to collect an iterable of AtreeItems enums into an atreebuild
            fn from_iter<T: IntoIterator<Item = $mod_name::AtreeItems>>(iter: T) -> Self {
                let mut temp_items_ids = 0;
                for itm in iter{
                    temp_items_ids |= 1<<itm as u128;
                }
                AtreeBuild::from((Class::$cls, temp_items_ids))
            }
        }
    }
);
impl_atreeitemtrait!(Archer, archer, ArcherAtreeEnums);
impl_atreeitemtrait!(Warrior, warrior, WarriorAtreeEnums);
impl_atreeitemtrait!(Mage, mage, MageAtreeEnums);
impl_atreeitemtrait!(Assassin, assassin, AssassinAtreeEnums);
impl_atreeitemtrait!(Shaman, shaman, ShamanAtreeEnums);

pub struct AtreeItemEffect<ClassEnum: ClassAtreeEnums>{
    data: &'static [u32],
    phantom: PhantomData<ClassEnum>
}
impl <T: ClassAtreeEnums> AtreeItemEffect<T>{
    /// Gets the index of `self.data` which has key `key`<br>
    /// This assumes `self.data` is sorted by key, and will cut off and return self.data.len() if a found datakey > `key`
    const fn get_idx_of_key(&self, key: EffectKey) -> usize{
        let key_u32 = key as u32;
        let mut i = 0;
        while i < self.data.len(){
            let curr_key = parse_data_k(self.data[i]);
            if curr_key==key_u32{return i}
            else if curr_key>key_u32{return self.data.len()}
            // the python generator file should make all 'iterable' types come after EffectKey::Parts
            if curr_key>=EffectKey::Parts as u32{
                i+=parse_data_u32(self.data[i]) as usize
            }
            i+=1;
        }
        i
    }
    pub const fn get_type(&self) -> EffectType{
        EffectType::VARIENTS[parse_data_u32(self.data[0]) as usize]
    }
    pub const fn get_name(&self) -> Option<T::Spell>{
        if parse_data_k(self.data[1])==EffectKey::Name as u32{
            Some(T::Spell::VARIENTS[parse_data_u32(self.data[1]) as usize])
        }else{
            None
        }
    }
    pub const fn get_cost(&self) -> Option<i32>{
        if parse_data_k(self.data[2])==EffectKey::Cost as u32{
            Some(parse_data_i32(self.data[2]))
        }else if parse_data_k(self.data[3])==EffectKey::Cost as u32{
            Some(parse_data_i32(self.data[3]))
        }else{
            None
        }
    }
    pub const fn get_base_spell(&self) -> Option<usize>{
        if parse_data_k(self.data[1])==EffectKey::BaseSpell as u32{
            Some(parse_data_u32(self.data[1]) as usize)
        }else if parse_data_k(self.data[2])==EffectKey::BaseSpell as u32{
            Some(parse_data_u32(self.data[2]) as usize)
        }else if parse_data_k(self.data[3])==EffectKey::BaseSpell as u32{
            Some(parse_data_u32(self.data[3]) as usize)
        }else{
            None
        }
    }
    pub (self) fn iter_hits_from_idx(&self,hits_idx: usize,num_items: usize) -> AtreeEffectIter<T::SpellPart,u32>{
        AtreeEffectIter::make(self.data,hits_idx+1,hits_idx+1+num_items,u32::MAX)
    }
    pub (self) fn iter_mults_from_idx(&self,mults_idx: usize,num_items: usize) -> std::iter::Map<std::slice::Iter<'_, u32>, fn(&u32) -> i32>{
        // println!("iterating mults from index {} {} for effect {:?}",mults_idx, num_items, self.get_name());
        self.data[mults_idx+1..mults_idx+1+num_items].into_iter().map(|v| *v as i32)
        // AtreeEffectIter::make(self.data,mults_idx+1,num_items,u32::MAX)
    }
    pub (self) fn iter_parts_from_idx(&self,parts_idx: usize,num_items: usize) -> AtreeEffectIter<EffectPartKey,i32>{
        // println!("iterating parts from index {} {} for effect {:?}",parts_idx, num_items, self.get_name());
        AtreeEffectIter::make(self.data,parts_idx+1,parts_idx+1+num_items,EffectPartKey::Multipliers as u32)
    }
    pub (self) fn iter_bonuses_from_idx(&self,bonuses_idx: usize,num_items: usize) -> AtreeEffectIter<EffectPartKey,i32>{
        AtreeEffectIter::make(self.data,bonuses_idx+1,bonuses_idx+1+num_items,u32::MAX)
    }
    pub (self) fn iter_data<N>(&self) -> AtreeEffectIter<EffectKey,N>{
        AtreeEffectIter::make(self.data,0,self.data.len(),EffectKey::Parts as u32)
    }
}

struct AtreeBuildConstructor{

}

/// Parses atree items into usable data
/// 
/// This struct breaks down the abilities provided by an atree and transforms them into their bare-bones data,
/// for example this will transform spells into a simple array of floats representing the damage multipliers provided by that spell. 
/// 
/// Note that AtreeItems are unchecked when constructing the AtreeBuild, so it is possible to have multiple of the same AtreeItem or create "invalid" atrees
/// 
/// TODO: add an optional safety check
pub struct AtreeBuild{
    spells: [Spell; 5],
    stat_bonuses: Vec<(Atrs,i32)>,
    stat_scalers: Vec<(Vec<(Atrs,f32)>,Atrs,f32)>, // <((stat, scale), output stat, max)>. todo - do something else this is ugly
    pub total_spell_mults: [[f32; 6]; 5],
    spell_costs: [i32; 4],
    atree_class: Class,
    atree_items_ids: u128
}
impl AtreeBuild{
    pub fn iter_stat_bonuses(&self) -> std::slice::Iter<'_, (Atrs, i32)>{
        self.stat_bonuses.iter()
    }
    fn add_raw_stat_effect<T: ClassAtreeEnums>(&mut self, effect: AtreeItemEffect<T>){
        for (k,v,effect_idx) in effect.iter_data::<u32>(){
            match k{
                EffectKey::Type => assert_eq!(EffectType::RawStat, v.try_into().unwrap()),
                EffectKey::Toggle => return, // TODO: add ability to control this toggle somehow
                EffectKey::Bonuses => {
                    for i in 0..v as usize/2{
                        if effect.data[effect_idx+1+i*2]==0{ // standard stat addition
                            let data = effect.data[effect_idx+2+i*2];
                            match parse_data_key(data){
                                Ok(k) => self.stat_bonuses.push((k,parse_data_i32(data))),
                                Err(_) => (),
                            }
                        }else{
                            // prop modification? 
                        }
                    }
                },
                _ => panic!("Found unknown data inside of raw_stat effect")
            }
        }
        // for bonus in effect.iter_bonuses_from_idx(bonuses_idx, num_items){

        // }
    }
    fn add_spell<T: ClassAtreeEnums>(&mut self, spell_effect: AtreeItemEffect<T>){
        match spell_effect.get_base_spell(){
            Some(spell_id) => match spell_effect.get_type(){
                EffectType::ReplaceSpell => self.spells[spell_id] = Spell::make_spell(spell_effect), 
                EffectType::AddSpellProp => self.spells[spell_id].add_spell_prop(spell_effect),
                _ => panic!("Attempted to parse a non-spell atree item as a spell")
            },
            None => todo!(),
        }
    }
    fn setup_base_melee(&mut self){
        self.spells[0]=Spell::melee_default();
    }
    fn finalize_spells(&mut self){
        for i in 0..5{
            self.total_spell_mults[i] = self.spells[i].total_mult();
            if i > 0 { // for all real 'spells' (not melee)
                self.spell_costs[i-1] = self.spells[i].cost();
            }
        }
    }
    /// Get the base cost of spell 0-3
    pub fn get_cost(&self, spell_idx: usize) -> i32{
        self.spell_costs[spell_idx]
    }
    /// Get the damage mults of spell 0-3
    pub fn get_spell_mults(&self, spell_idx: usize) -> [f32; 6]{
        self.total_spell_mults[spell_idx+1]
    }
    /// Get the damage mults of a melee hit
    pub fn get_melee_mults(&self) -> [f32; 6]{
        self.total_spell_mults[0]
    }

    /// Gets the atree item ids used to construct this atree build, in addition to their associated class
    pub fn get_atree_items_ids(&self) -> (Class, Vec<u32>){
        let mut temp = self.atree_items_ids;
        let mut curr = 0;
        (self.atree_class, (0..self.atree_items_ids.count_ones()).map(|_| {let res = temp.trailing_zeros()+1; temp>>=res; curr+=res; curr-1}).collect::<Vec<u32>>())
    }

    /// Transforms this atree into a compressed string hash which can be used to save this build as a string
    pub fn get_hash(&self) -> String{
        url_hash_val(self.atree_items_ids,0)
    }

    /// Transforms this atree into a string representing the atree, which is used by wynnbuilder's URL sharing system
    /// 
    /// (broken idk why)
    pub fn get_wynnbuilder_hash(&self) -> String{
        url_hash_val(self.atree_items_ids>>1,0)
    }

    pub fn from_hash(hash: &str, class: Class) -> Self{
        Self::from((class, unhash_val(hash)))
    }

    fn with_atree_items_ids(mut self, atree_items_ids: u128) -> Self{
        self.atree_items_ids = atree_items_ids;
        self
    }
}
impl PartialEq for AtreeBuild{
    fn eq(&self, other: &Self) -> bool {
        self.atree_class==other.atree_class && self.atree_items_ids==other.atree_items_ids
    }
}
impl Default for AtreeBuild{
    fn default() -> Self {
        Self { spells: [Spell::melee_default(), Default::default(), Default::default(), Default::default(), Default::default()], stat_bonuses: Default::default(), stat_scalers: Default::default(), total_spell_mults: Default::default(), spell_costs: Default::default(), atree_class: Default::default(), atree_items_ids: Default::default() }
    }
}

// might need to allow for modified atree items?
// ie, how nighthawk's major id modifies the 'num_streams' property of arrow storm
// not sure how to go about major ids yet though... 

// impl <T: ClassAtreeEnums> std::convert::From<&[AtreeItem<T>]> for AtreeBuild{
//     fn from(value: &[AtreeItem<T>]) -> Self {
//         let mut temp_items_ids = 0;
//         for itm in value{
//             temp_items_ids |= 1<<itm.data.enum_id.into() as u128;
//         }
//         AtreeBuild::from((T::CLASS, temp_items_ids))
//     }
// }

impl <A: ClassAtreeEnums> std::iter::FromIterator<&'static AtreeItemData<A>> for AtreeBuild{
    /// You shouldn't be calling this unless you really know what you're doing
    fn from_iter<T: IntoIterator<Item = &'static AtreeItemData<A>>>(iter: T) -> Self {
        let mut res = AtreeBuild::default();
        res.atree_class=A::CLASS;
        for itm in iter{
            for effect in itm.iter_effects(){
                match effect.get_type(){
                    EffectType::ReplaceSpell | EffectType::AddSpellProp => res.add_spell(effect),
                    EffectType::RawStat => res.add_raw_stat_effect(effect),
                    EffectType::StatScaling => (),
                }
            }
        }
        res.finalize_spells();
        res
    }
}

impl std::convert::From<(Class, u128)> for AtreeBuild{
    /// Creates an AtreeBuild from a specified wynncraft class, and a bit table representing selected atree items
    fn from(value: (Class, u128)) -> Self {
        let mut temp = value.1;
        let mut curr = 0;
        match value.0{
            Class::Archer => AtreeBuild::from_iter((0..value.1.count_ones()).map(|_| {let res = temp.trailing_zeros()+1; temp>>=res; curr+=res; archer::atree_data::ATREE_DATA[(curr-1) as usize]})),
            Class::Warrior => AtreeBuild::from_iter((0..value.1.count_ones()).map(|_| {let res = temp.trailing_zeros()+1; temp>>=res; curr+=res; warrior::atree_data::ATREE_DATA[(curr-1) as usize]})),
            Class::Mage => AtreeBuild::from_iter((0..value.1.count_ones()).map(|_| {let res = temp.trailing_zeros()+1; temp>>=res; curr+=res; mage::atree_data::ATREE_DATA[(curr-1) as usize]})),
            Class::Assassin => AtreeBuild::from_iter((0..value.1.count_ones()).map(|_| {let res = temp.trailing_zeros()+1; temp>>=res; curr+=res; assassin::atree_data::ATREE_DATA[(curr-1) as usize]})),
            Class::Shaman => AtreeBuild::from_iter((0..value.1.count_ones()).map(|_| {let res = temp.trailing_zeros()+1; temp>>=res; curr+=res; shaman::atree_data::ATREE_DATA[(curr-1) as usize]})),
        }.with_atree_items_ids(value.1)
    }
}

struct IterAtreeItems<T: WynnEnum + std::convert::TryFrom<u8>>{
    data: &'static[u8],
    curr: usize,
    phantom: PhantomData<T>
}
impl <T> std::convert::From<&'static[u8]> for IterAtreeItems<T> where T: WynnEnum + std::convert::TryFrom<u8>{
    fn from(value: &'static[u8]) -> Self {
        Self{data: value, curr: 0, phantom: Default::default()}
    }
}
impl <T> Iterator for IterAtreeItems<T> where T: WynnEnum + std::convert::TryFrom<u8>{    
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr>=self.data.len(){
            None
        }else{
            self.curr+=1;
            match T::try_from(self.data[self.curr-1]){
                Ok(t) => Some(t),
                Err(_) => None
            }
        }
    }
}
struct IterAtreeEffects<T: ClassAtreeEnums>{
    data: &'static[&'static[u32]],
    curr: usize,
    phantom: PhantomData<T>
}
impl <T> Iterator for IterAtreeEffects<T> where T: ClassAtreeEnums{    
    type Item = AtreeItemEffect<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr>=self.data.len(){
            None
        }else{
            self.curr+=1;
            Some(Self::Item{data: self.data[self.curr-1],phantom:PhantomData})
        }
    }
}

struct AtreeEffectIter<K: WynnEnum,T>{
    data: &'static[u32],
    curr: usize,
    end: usize,
    check_skip_past_idx: u32,
    phantom: PhantomData<(K,T)>
}
impl <K:WynnEnum,T> AtreeEffectIter<K,T>{
    const fn make(data: &'static[u32], start: usize, end: usize, use_val_to_skip_past_idx: u32) -> Self{
        Self{data, curr: start, end, check_skip_past_idx: use_val_to_skip_past_idx, phantom: PhantomData}
    }
}
impl <K: WynnEnum> Iterator for AtreeEffectIter<K, i32>{    
    type Item = (K,i32,usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr>=self.end{
            None
        }else{
            let curr_value = self.data[self.curr];
            // println!("curr: {} {} {:?} {}",self.curr, curr_value, parse_data_key::<K>(curr_value), parse_data_i32(curr_value));
            let res = Some((parse_data_key(curr_value).unwrap(),parse_data_i32(curr_value),self.curr));
            if parse_data_k(curr_value)>=self.check_skip_past_idx{
                self.curr+=parse_data_u32(curr_value) as usize;
            }
            self.curr+=1;
            res
        }
    }
}
impl <K: WynnEnum> Iterator for AtreeEffectIter<K, u32>{    
    type Item = (K,u32,usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr>=self.end{
            None
        }else{
            let curr_value = self.data[self.curr];
            let res = Some((parse_data_key(curr_value).unwrap(),parse_data_u32(curr_value),self.curr));
            if parse_data_k(curr_value)>=self.check_skip_past_idx{
                self.curr+=parse_data_u32(curr_value) as usize;
            }
            self.curr+=1;
            res
        }
    }
}
impl <K: WynnEnum> Iterator for AtreeEffectIter<K, f32>{    
    type Item = (K,f32,usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr>=self.end{
            None
        }else{
            let curr_value = self.data[self.curr];
            let res = Some((parse_data_key(curr_value).unwrap(),parse_data_i32(curr_value) as f32 / 100f32,self.curr));
            if parse_data_k(curr_value)>=self.check_skip_past_idx{
                self.curr+=parse_data_u32(curr_value) as usize;
            }
            self.curr+=1;
            res
        }
    }
}

struct AtreeBonus{
    bonus_type: BonusType,
    bonus_abil: u32, //todo
    key_val: u32
}

struct AtreeBonusIter<C: ClassAtreeEnums>{
    data: &'static[u32],
    curr: usize,
    end: usize,
    phantom: PhantomData<C>
}
impl <C: ClassAtreeEnums> Iterator for AtreeBonusIter<C>{
    type Item = AtreeBonus;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr>=self.end{
            None
        }else{
            let res = Some(AtreeBonus{
                bonus_type: parse_data_key(self.data[self.curr]).unwrap(), 
                bonus_abil: parse_data_u32(self.data[self.curr]),
                key_val: self.data[self.curr+1]
            });
            self.curr+=2;
            res
        }
    }
}

pub trait ClassAtreeEnums{
    const CLASS: Class;
    type Items: WynnEnum + std::convert::TryFrom<u8>;
    type Prop: WynnEnum;
    type Spell: WynnEnum;
    type SpellPart: WynnEnum;
}



// struct Atree