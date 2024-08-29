mod archer;mod assassin;mod mage;mod shaman;mod warrior;mod enums;mod spells;
use super::{WynnEnum,Class,WynnDataIter,parse_data_key,parse_data_k,parse_data_val,parse_data_uval,parse_data_u32,parse_data_i32,items::Atrs};
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
    props: Vec<u32>
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
        if parse_data_k(self.data[1])==EffectKey::Cost as u32{
            Some(parse_data_i32(self.data[1]))
        }else if parse_data_k(self.data[2])==EffectKey::Cost as u32{
            Some(parse_data_i32(self.data[2]))
        }else{
            None
        }
    }
    pub (self) fn iter_hits_from_idx(&self,hits_idx: usize,num_items: usize) -> AtreeEffectIter<T::SpellPart,u32>{
        AtreeEffectIter::make(self.data,hits_idx+1,num_items,u32::MAX)
    }
    pub (self) fn iter_mults_from_idx(&self,mults_idx: usize,num_items: usize) -> AtreeEffectIter<EffectKey,i32>{
        AtreeEffectIter::make(self.data,mults_idx+1,num_items,EffectKey::Parts as u32)
    }
    pub (self) fn iter_parts_from_idx(&self,parts_idx: usize,num_items: usize) -> AtreeEffectIter<EffectPartKey,i32>{
        AtreeEffectIter::make(self.data,parts_idx+1,num_items,EffectPartKey::Multipliers as u32)
    }
    pub (self) fn iter_bonuses_from_idx(&self,bonuses_idx: usize,num_items: usize) -> AtreeEffectIter<EffectPartKey,i32>{
        AtreeEffectIter::make(self.data,bonuses_idx+1,num_items,u32::MAX)
    }
    pub (self) fn iter_data<N>(&self) -> AtreeEffectIter<EffectKey,N>{
        AtreeEffectIter::make(self.data,0,self.data.len(),EffectKey::Parts as u32)
    }
}

#[derive(Default,PartialEq)]
pub struct AtreeBuild{
    // spells: [Spell; 5],
    stat_bonuses: Vec<(Atrs,i32)>,
    stat_scalers: Vec<(Vec<(Atrs,f32)>,Atrs,f32)>, // <((stat, scale), output stat, max)>. todo - do something else this is ugly
    pub total_spell_mults: [[f32; 6]; 5],
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
                    for (bk, bv, bidx) in effect.iter_bonuses_from_idx(effect_idx, v as usize){
                        match bk{
                            EffectPartKey::Name => todo!(),
                            EffectPartKey::Type => todo!(),
                            EffectPartKey::Power => todo!(),
                            EffectPartKey::Multipliers => todo!(),
                            EffectPartKey::Hits => todo!(),
                        }
                    }
                },
                _ => panic!("Found unknown data inside of raw_stat effect")
            }
        }
        // for bonus in effect.iter_bonuses_from_idx(bonuses_idx, num_items){

        // }
    }
}
// impl std::convert::From<&[AtreeItem]> for AtreeBuild{
//     fn from(value: &[AtreeItem]) -> Self {
//         // match value[0].class(){
//         //     Class::Archer => todo!(),
//         //     Class::Warrior => todo!(),
//         //     Class::Mage => todo!(),
//         //     Class::Assassin => todo!(),
//         //     Class::Shaman => todo!(),
//         // }
//         todo!()
//     }
// }

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