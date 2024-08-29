mod items_list;
pub mod item_consts;
pub mod enums;
pub use enums::*;
use super::{Skill, DamType, I12x5, sets::Sets, WynnDataIter, url_hash_val, parse_data_k, parse_data_uval, parse_data_u32, parse_data_key, parse_data_i32};
use std::vec::IntoIter;

struct WynnItemData<'a> {
    name: &'a str,
    sps_req_data: i64,
    sps_bonus_data: i64,
    partitions: (usize, usize, usize, usize),
    data: &'a [u32],
}
impl<'a> WynnItemData<'a> {
    fn data_atr(&self, idx: usize) -> u32 {
        super::parse_data_k(self.data[idx])
    }
    fn data_ival(&self, idx: usize) -> i32 {
        super::parse_data_i32(self.data[idx])
    }
    fn data_uval(&self, idx: usize) -> u32 {
        super::parse_data_u32(self.data[idx])
    }
}
/// Represents a wynncraft item and provides an interface for accessing the item's data.
/// 
/// Note that the only thing you can modify about a WynnItem is its (identifications) quality and powders.
/// <br>Also note that this only keeps track of t6 powders 
/// 
/// The recommended way to get WynnItems is by using items::with_name();
/// # Example
/// ```
/// let warp: WynnItem = items::with_name("Warp");
/// asserteq!(warp.name(), "Warp");
/// asserteq!(warp.get_type(), Type::Wand);
/// ```
#[derive(Clone, Copy)] // I heard that it's recommended to pass by value if size <= 2x register size (ie, 128 bits for 64 bit system), and reference otherwise?
pub struct WynnItem {
    item: &'static WynnItemData<'static>,
    quality: f32,
    powders: u32, // because i'm lazy, i'm only counting t6 powders because why would you use anything else for a real build. also this keeps wynnitems more efficient to copy.
}
impl WynnItem {
    /// Returns an invalid 'null' item, which can be used as a placeholder for an actual item. 
    pub const NULL: WynnItem = WynnItem{
        item: items_list::ALL_ITEMS[0],
        quality: 1.0,
        powders: 0
    };
    /// Finds a WynnItem using its index from this module's internal constant array (items_list::ALL_ITEMS)
    pub const fn from_idx(idx: usize) -> WynnItem {
        WynnItem {
            item: items_list::ALL_ITEMS[idx],
            quality: 1.0,
            powders: 0
        }
    }
    /// Finds a wynn item with the given id
    pub fn from_id(id: u32) -> Option<WynnItem>{
        iter().find(|itm| itm.id()==id)
    }
    pub fn get_type(&self) -> Type {
        Type::try_from(if self.is_null() {
            7
        } else {
            self.item.data_uval(Atrs::Type as usize)
        })
        .unwrap_or(Type::Helmet)
    }
    /// Gets the numeric identifier of this item
    /// 
    /// Useful to check if two items are the same (ignorning identification quality and powders)
    pub fn id(&self) -> u32{
        self.item.data_uval(Atrs::Id as usize)
    }
    pub fn get_category(&self) -> Category {
        Category::try_from(if self.is_null() {
            0
        } else {
            self.item.data_uval(Atrs::Category as usize)
        })
        .unwrap_or(Category::Armor)
    }
    pub fn is_null(&self) -> bool {
        self.item.data.is_empty()
    }
    /// Returns whether this item has fixed identifications
    pub fn fixed_id(&self) -> bool {
        self.item.data.len() >= (Atrs::FixID as usize + 1) && self.item.data_atr(Atrs::FixID as usize) == Atrs::FixID as u32
    }
    pub fn name(&self) -> &str {
        self.item.name
    }
    pub fn atk_spd(&self) -> AtkSpd {
        if self.item.data_atr((Atrs::AtkSpd as usize - 1).min(self.item.data.len() - 1)) == Atrs::AtkSpd as u32 {
            AtkSpd::try_from(self.item.data_uval(Atrs::AtkSpd as usize - 1)).unwrap_or(AtkSpd::Normal)
        } else if self.item.data_atr((Atrs::AtkSpd as usize).min(self.item.data.len() - 1)) == Atrs::AtkSpd as u32 {
            AtkSpd::try_from(self.item.data_uval(Atrs::AtkSpd as usize)).unwrap_or(AtkSpd::Normal)
        } else {
            AtkSpd::Normal
        }
    }
    pub fn get_tier(&self) -> Tier {
        Tier::try_from(if self.is_null() {
            0
        } else {
            self.item.data_uval(Atrs::Tier as usize)
        })
        .unwrap_or(Tier::Common)
    }
    fn calc_id(&self, idx: usize) -> i32 {
        let base_value = self.item.data_ival(idx);
        if self.fixed_id() || self.item.data_atr(idx) < Atrs::NUM_NON_IDS as u32{
            base_value
        } else if base_value > 0 {
            (base_value as f32 * (self.quality + 0.3)).round() as i32
        } else {
            (base_value as f32 * ((1.0 - self.quality) * 0.6 + 0.7) + 4.000001 * f32::EPSILON)
                .round() as i32
        }
    }
    pub fn get_hash(&self) -> String {
        url_hash_val(self.item.data_ival(Atrs::Id as usize), 3)
    }
    fn get_data_i32(&self, idx: usize) -> Option<(Atrs, i32)> {
        if idx >= self.item.data.len() {
            None
        } else {
            Some((
                Atrs::try_from(self.item.data_atr(idx)).unwrap(),
                self.calc_id(idx),
            ))
        }
    }
    /// Get an identification from this item
    pub fn get_ident(&self, ident: Atrs) -> Option<i32> {
        let id_u32 = ident as u32;
        match self.item.data.binary_search(&(id_u32 << 24)) {
            Ok(n) => Some(self.calc_id(n)),
            Err(n) => {
                if n >= self.item.data.len() || self.item.data_atr(n) != id_u32 {
                    None
                } else {
                    Some(self.calc_id(n))
                }
            }
        }
    }
    pub fn get_set(&self) -> Sets {
        if self
            .item
            .data_atr((self.item.partitions.0 - 1).min(self.item.data.len() - 1))
            == 12
        {
            Sets::try_from(self.item.data_uval(self.item.partitions.0 - 1)).unwrap_or(Sets::None)
        } else {
            Sets::None
        }
    }
    pub fn set_quality(&mut self, qual: f32) {
        self.quality = qual
    }
    /// Iterate through the identifications of this item
    pub fn iter_ids(&self) -> IdsIter<'_> {
        IdsIter {
            item: self,
            curr: self.item.partitions.3,
            end: self.item.data.len(),
        }
    }
    pub fn iter_data(&self) -> WynnDataIter<Atrs, i32> {
        WynnDataIter::make(self.item.data,0,self.item.data.len())
    }
    pub fn iter_skill_reqs(&self) -> WynnDataIter<Atrs, i32> {
        WynnDataIter::make(self.item.data,self.item.partitions.0,self.item.partitions.1)
    }
    pub fn iter_skill_bonus(&self) -> WynnDataIter<Atrs, i32> {
        WynnDataIter::make(self.item.data,self.item.partitions.1,self.item.partitions.2)
    }
    pub fn iter_damages(&self) -> DamsIter<'_> {
        DamsIter {
            item: self,
            curr: self.item.partitions.2,
            end: self.item.partitions.3,
        }
    }
    /// Iterates over the skill requirements *and* skill bonuses for the item.
    ///
    /// The iterator yields a tuple of the form (skill: Skill, requirement: i32, bonus: i32)
    /// # Example
    /// ```
    /// let spring: WynnItem = items::with_name("Spring");
    /// let skill_iter = pure.iter_skills();
    /// asserteq!(skill_iter.next(),(Skill::Str, 0, 15));
    /// asserteq!(skill_iter.next(),(Skill::Dex, 0, -40));
    /// asserteq!(skill_iter.next(),(Skill::Int, 120, 15));
    /// ```
    pub fn iter_skills(&self) -> SkillsIter<'_> {
        SkillsIter {
            item: self,
            req_idx: self.item.partitions.0,
            req_end_idx: self.item.partitions.1,
            bonus_idx: self.item.partitions.1,
            bonus_end_idx: self.item.partitions.2,
        }
    }
    /// Gets the skill requirements for this item.
    /// 
    /// Note that skills without requirements will have their requirement set to -1024
    /// # Example
    /// ```
    /// let spring: WynnItem = items::with_name("Spring");
    /// asserteq!(spring.get_skill_reqs(),SkillPts::from([-1024,-1024,120,-1024,-1024]));
    /// ```
    /// <hr><br>🙏 Praying that +-1024 skill point reqs or bonus (with only items) never becomes possible. 
    pub fn get_skill_reqs(&self) -> I12x5{
        I12x5{data: self.item.sps_req_data}
    }
    pub fn get_skill_bonuses(&self) -> I12x5{
        I12x5{data: self.item.sps_bonus_data}
    }
}
impl std::fmt::Debug for WynnItem{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"WynnItem{{{}}}",self.name())
    }
}
impl PartialEq for WynnItem{
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id() && self.quality == other.quality && self.powders == other.powders
    }
}

/// TODO: Powders
pub struct DamsIter<'a> {
    item: &'a WynnItem,
    curr: usize,
    end: usize,
}
impl Iterator for DamsIter<'_> {
    type Item = (DamType, (u32, u32));
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.end || self.curr >= self.item.item.data.len() {
            return None;
        };
        let dam_data = self.item.item.data[self.curr];
        let res = (
            DamType::try_from(self.item.item.data_atr(self.curr) - Atrs::NDam as u32).unwrap_or(DamType::Neutral),
            ((dam_data & 0xFFF000) >> 12, dam_data & 0xFFF),
        );
        self.curr += 1;
        Some(res)
    }
}
pub struct SkillsIter<'a> {
    item: &'a WynnItem,
    req_idx: usize,
    bonus_idx: usize,
    req_end_idx: usize,
    bonus_end_idx: usize,
}
impl Iterator for SkillsIter<'_> {
    type Item = (Skill, i32, i32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.req_idx >= self.req_end_idx && self.bonus_idx >= self.bonus_end_idx {
            return None;
        };
        let req = if self.req_idx < self.item.item.data.len() {
            self.item.item.data_atr(self.req_idx) - 13
        } else {
            6
        };
        let bon = if self.bonus_idx < self.item.item.data.len() {
            self.item.item.data_atr(self.bonus_idx) - 18
        } else {
            6
        };
        let mut res = (Skill::try_from(req.min(bon)).unwrap_or(Skill::Str), 0, 0);
        if req < bon {
            res.1 = self.item.item.data_ival(self.req_idx);
            self.req_idx += 1;
        } else if req > bon {
            res.2 = self.item.item.data_ival(self.bonus_idx);
            self.bonus_idx += 1;
        } else {
            res.1 = self.item.item.data_ival(self.req_idx);
            self.req_idx += 1;
            res.2 = self.item.item.data_ival(self.bonus_idx);
            self.bonus_idx += 1;
        }
        Some(res)
    }
}

pub struct IdsIter<'a>{
    item: &'a WynnItem,
    curr: usize,
    end: usize
}
impl Iterator for IdsIter<'_> {
    type Item = (Atrs, i32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.end {
            return None;
        }
        let res = (
            Atrs::try_from(self.item.item.data_atr(self.curr)).unwrap_or_default(),
            self.item.calc_id(self.curr),
        );
        self.curr += 1;
        Some(res)
    }
}

pub fn iter() -> IntoIter<WynnItem> {
    items_list::ALL_ITEMS
        .iter()
        .enumerate()
        .map(|(idx, v)| WynnItem::from_idx(idx))
        .collect::<Vec<WynnItem>>()
        .into_iter()
}
pub const fn get(idx: usize) -> WynnItem {
    WynnItem::from_idx(idx)
}
pub fn with_prop_value(prop: Atrs, value: u32) -> Vec<WynnItem> {
    let mut res: Vec<WynnItem> = Vec::new();
    let prop_u32 = prop as u32;
    for (i, u) in items_list::ALL_ITEMS.iter().skip(1).enumerate() {
        let search = u.data.binary_search(&(prop_u32 << 24));
        let idx = match search {
            Ok(v) => v,
            Err(v) => v,
        };
        if u.data_atr(idx) == prop_u32 && u.data_uval(idx) == value {
            res.push(WynnItem::from_idx(i + 1))
        }
    }
    res
}
pub fn with_name(s: &str) -> Option<WynnItem> {
    let s_trimmed = s.trim();
    for item in items_list::ALL_ITEMS.iter().enumerate() {
        if item.1.name.eq_ignore_ascii_case(s_trimmed) {
            return Some(WynnItem::from_idx(item.0));
        }
    }
    return None;
}
