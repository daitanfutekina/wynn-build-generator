//! This module stores general functions and definitions used throughout wynn_data<br>
//! All the definitions here should be imported by the crate for easy access
mod enums;
mod macros;
mod plzdontlook;
pub use enums::*;
pub use plzdontlook::*;

pub(super) const DIGITS: [char; 64] = ['0','1','2','3','4','5','6','7','8','9','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','+','-'];
/// A lot of data I generate is stored in the format `(key: 8 bits, value: 24 bits)`<br>
/// This method is used to extract the unparsed key (first 8 bits).<br>
/// Use parse_data_key instead if a specific type is expected. 
pub(super) const fn parse_data_k(n: u32)->u32{n>>24}
/// A lot of data I generate is stored in the format `(key: 8 bits, value: 24 bits)`<br>
/// This method is used to extract the value (last 24 bits) as a u32.<br>
/// Use parse_data_uval instead if a specific type is expected, and the expected type can be constructed from u32's.
pub(super) const fn parse_data_u32(n: u32)->u32{n&0xFFFFFF}
/// A lot of data I generate is stored in the format `(key: 8 bits, value: 24 bits)`<br>
/// This method is used to extract the value (last 24 bits) as a i32.<br>
/// Use parse_data_val instead if a specific type is expected, and the expected type can be constructed from i32's.
pub(super) const fn parse_data_i32(n: u32)->i32{(n as i32)<<8>>8}
/// A lot of data I generate is stored in the format `(key: 8 bits, value: 24 bits)`<br>
/// This method is used to extract the key (first 8 bits), and tries to cast it to a given type.<br>
pub(super) fn parse_data_key<T: std::convert::TryFrom<u32>>(n: u32)->Result<T, <T as TryFrom<u32>>::Error>{T::try_from(n>>24)}
/// See `parse_data_u32()`
pub(super) fn parse_data_uval<T: std::convert::TryFrom<u32>>(n: u32)->Result<T, <T as TryFrom<u32>>::Error>{T::try_from(n&0xFFFFFF)}
/// See `parse_data_i32()`
pub(super) fn parse_data_val<T: std::convert::TryFrom<i32>>(n: u32)->Result<T, <T as TryFrom<i32>>::Error>{T::try_from(parse_data_i32(n))}

pub struct TryIntoWynnEnumError<F:std::fmt::Debug,T:WynnEnum>{pub (crate) from:F,pub(crate) to:T}
impl <F:std::fmt::Debug,T:WynnEnum+std::default::Default> TryIntoWynnEnumError<F,T>{fn make(from: F) -> Self{Self{from, to: Default::default()}}}
impl <F: std::fmt::Debug, T: WynnEnum> std::fmt::Debug for TryIntoWynnEnumError<F,T>{fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {write!(f,"Could not convert {:#?} into {:#?}", &self.from, std::any::type_name::<T>())}}
pub trait WynnEnum: Sized + std::fmt::Debug + std::fmt::Display + Clone + Copy + Default + PartialEq + std::convert::TryFrom<u32, Error=TryIntoWynnEnumError<u32,Self>> + std::convert::Into<u32> + 'static{
    const VARIENTS: &'static[Self];
    const NUM_VARIENTS:usize = Self::VARIENTS.len();
    fn iter() -> std::slice::Iter<'static, Self>{
        Self::VARIENTS.into_iter()
    }
}

/// Converts a number of skill points to a percent (in range 0-1)
/// (ie, 150 strength = 80.8% damage boost)
/// 
/// Note that Int, Def, and Agi have slightly different %'s from the default given here
/// 
/// Use `skill_to_pct()` instead for the actual % bonus for respective skills
/// # Examples
/// ```
/// asserteq!(raw_skill_pct(150),0.808);
/// asserteq!(raw_skill_pct(100),0.399);
/// ```
pub fn raw_skill_pct(pts: i32) -> f32 {
    if pts <= 0 {
        return 0.0;
    };
    let r: f32 = 0.9908;
    (r / (1.0 - r) * (1.0 - r.powi(pts))) / 100.0
}

/// Handles all skill point to percent conversions. Converts to a f32 in range 0-1
/// # Examples
/// ```
/// asserteq!(skill_to_pct(Skill::Dex,50),39.9); // get the %crit given 50 dex
/// asserteq!(skill_to_pct(Skill::Int,50),24.7); // get the %cost red. given 50 int
/// asserteq!(skill_to_pct(Skill::Agi,50),37.9); // get the %dodge. given 50 agi
/// ```
pub fn skill_to_pct(skill: Skill, amt: i32) -> f32 {
    match skill{
        Skill::Str => raw_skill_pct(amt.min(150)),
        Skill::Dex => raw_skill_pct(amt.min(150)),
        Skill::Int => raw_skill_pct(amt.min(150))*0.6190092383711963,
        Skill::Def => raw_skill_pct(amt.min(150))*0.867,
        Skill::Agi => raw_skill_pct(amt.min(150))*0.951,
    }
}

/// Given a level, returns the health bonus one would recieve at that level.
/// 
/// This function follows the standard formula for base health at level given by:
/// 
/// `base health = (level+1)*5`
/// 
/// Note that this function is unbounded, so impossible levels (106+) will still return values.
/// # Examples
/// ```
/// asserteq!(get_health_at_level(1),10);
/// asserteq!(get_health_at_level(105),530);
/// ```
pub const fn get_health_at_level(level: i32) -> i32{
    ((level+1)*5) as i32
}

/// Given a level, returns the health bonus one would recieve at that level.
/// 
/// This function follows the standard formula for base health at level given by:
/// 
/// `base health = (level+1)*5`
/// 
/// Note that this function is unbounded, so impossible levels (106+) will still return values.
/// # Examples
/// ```
/// asserteq!(get_health_at_level(1),10);
/// asserteq!(get_health_at_level(105),530);
/// ```
pub const fn get_spts_at_level(level: i32) -> i32{
    if level < 100 {level*2} else {200}
}

/// Given a level, returns the health bonus one would recieve at that level.
/// 
/// This function uses the attack rate array (from slowest to fastest) given by:
/// 
/// `[0.51, 0.83, 1.5, 2.05, 2.5, 3.1, 4.3]`
/// # Examples
/// ```
/// asserteq!(atk_spd_mult(AtkSpd::Normal),2.05);
/// asserteq!(atk_spd_mult(AtkSpd::SuperSlow),0.51);
/// ```
pub const fn atk_spd_mult(spd: super::items::AtkSpd) -> f32{
    let temp = spd as usize;
    [0.51, 0.83, 1.5, 2.05, 2.5, 3.1, 4.3][if temp>6{6} else {temp}] // (spd as usize).clamp(0,6) -- clamp is not a constant function...
}

/// Used to transform data into the hashing format used by Wynnbuilder's build URL
/// 
/// `val` is the raw data being transformed, `n` is the number of characters each value should be converted to
pub(crate) fn url_hash_val(val: i32, n: u8) -> String {
    let mut result = String::new();
    let mut local_val = val;
    for i in 0..n {
        result = String::from(DIGITS[(local_val & 0x3f) as usize]) + &result;
        local_val >>= 6;
    }
    result
}

/// A lot of data I generate is stored in the format `(key: 8 bits, value: 24 bits)`<br>
/// This struct provides a bunch of iterators to easily iterate over some provided data
pub struct WynnDataIter<'a, K, V> {
    pub(crate) data: &'a [u32],
    pub(crate) curr: usize,
    pub(crate) end: usize,
    phantom: std::marker::PhantomData<(K,V)>
}
impl <'a, K, V> WynnDataIter<'a, K, V>{
    pub const fn make(data: &'a [u32], curr: usize, end: usize) -> Self{
        Self{data, curr, end, phantom: std::marker::PhantomData}
    }
}
impl <K: WynnEnum> Iterator for WynnDataIter<'_, K, i32> {
    type Item = (K, i32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.end {
            return None;
        }
        let res = (parse_data_key(self.data[self.curr]).unwrap(),parse_data_i32(self.data[self.curr]));
        // let res = (
        //     Atrs::try_from(parse_data_k(self.data[self.curr])).unwrap_or_default(),
        //     parse_data_i32(self.data[self.curr]),
        // );
        self.curr += 1;
        Some(res)
    }
}
impl <K: WynnEnum> Iterator for WynnDataIter<'_, K, u32> {
    type Item = (K, u32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.end {
            return None;
        }
        let res = (parse_data_key(self.data[self.curr]).unwrap(),parse_data_u32(self.data[self.curr]));
        self.curr += 1;
        Some(res)
    }
}
impl Iterator for WynnDataIter<'_, u32, i32> {
    type Item = (u32, i32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.end {return None;}
        let res = (parse_data_k(self.data[self.curr]),parse_data_i32(self.data[self.curr]));
        self.curr += 1;
        Some(res)
    }
}
impl Iterator for WynnDataIter<'_, u32, u32> {
    type Item = (u32, u32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.end {return None;}
        let res = (parse_data_k(self.data[self.curr]),parse_data_u32(self.data[self.curr]));
        self.curr += 1;
        Some(res)
    }
}
impl <K: WynnEnum> Iterator for WynnDataIter<'_, K, (u32,u32)> {
    type Item = (K, (u32,u32));
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.end {return None;}
        let temp = parse_data_u32(self.data[self.curr]);
        let res = (parse_data_key(self.data[self.curr]).unwrap(),(temp & 0xFFF, (temp & 0xFFF000) >> 12));
        self.curr += 1;
        Some(res)
    }
}
impl <V: WynnEnum> Iterator for WynnDataIter<'_, bool, V> {
    type Item = V;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.end {return None;}
        let res = parse_data_uval(self.data[self.curr]).unwrap();
        self.curr += 1;
        Some(res)
    }
}
impl Iterator for WynnDataIter<'_, bool, i32> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.end {return None;}
        let res = parse_data_i32(self.data[self.curr]);
        self.curr += 1;
        Some(res)
    }
}