mod set_bonus_data;
pub mod enums;
pub use enums::*;
use set_bonus_data::SET_BONUSES;
use super::{I12x5, WynnDataIter, items::enums::Atrs};

pub fn get_set_bonuses(set: Sets, num_items: usize) -> WynnDataIter<'static, Atrs, i32> {
    let s = set as usize;
    if num_items==0{
        WynnDataIter::make(SET_BONUSES[s][0].1,0,0)
    }else{
        WynnDataIter::make(SET_BONUSES[s][num_items-1].1,0,SET_BONUSES[s][num_items-1].1.len())
    }
}
pub fn get_set_skill_bonuses(set: Sets, num_items: usize) ->  I12x5{
    let s = set as usize;
    if num_items==0 || SET_BONUSES[s].len()==0{
        I12x5::ZERO
    }else{
        I12x5{data: SET_BONUSES[s][(num_items-1).min(SET_BONUSES[s].len()-1)].0}
    }
}