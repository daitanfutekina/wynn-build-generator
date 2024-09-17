use crate::wynn_data::{items::Atrs, WynnEnum, builder::WynnBuild};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SearchParam{
    Stat(Atrs),
    Calc(CalcStat)
}
impl SearchParam{
    pub fn all_varients() -> Vec<SearchParam>{
        let mut res: Vec<SearchParam> = CalcStat::VARIENTS.into_iter().map(|stat| Self::Calc(stat)).collect();
        res.extend(Atrs::iter().skip(Atrs::NUM_NON_STATS).map(|atr| Self::Stat(atr.clone())));
        res
    }
    pub fn usize_id(&self) -> usize{
        match self{
            Self::Stat(a) => CalcStat::NUM_VARIENTS + *a as usize - Atrs::NUM_NON_STATS,
            Self::Calc(b) => b.clone() as usize
        }
    }
    pub fn from_usize(id: usize) -> Self{
        if id < CalcStat::NUM_VARIENTS{
            Self::Calc(CalcStat::VARIENTS[id])
        }else{
            Self::Stat(Atrs::VARIENTS[id-CalcStat::NUM_VARIENTS+Atrs::NUM_NON_STATS])
        }
    }
}
impl std::fmt::Display for SearchParam{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",match self{
            Self::Stat(atr) => atr.to_string(), 
            Self::Calc(calc) => calc.to_string()
        })
    }
}
#[derive(Clone, Copy, PartialEq)]
pub enum SearchReq{
    Stat(Atrs, i32),
    Calc(CalcStat, f32)
}
impl SearchReq{
    pub fn stat_eq(&self, other: &Self) -> bool{
        match self{
            Self::Stat(a, _) => match other{Self::Stat(b, _) => a==b, _ => false}
            Self::Calc(a, _) => match other{Self::Calc(b, _) => a==b, _ => false}
        }
    }
    pub fn name_and_val(&self) -> (String, f32){
        match self{
            Self::Stat(a, v) => (a.to_string(), *v as f32),
            Self::Calc(c, v) => (c.to_string(), *v)
        }
    }
    pub fn debug_name_and_val(&self) -> (String, f32){
        match self{
            Self::Stat(a, v) => (a.to_string(), *v as f32),
            Self::Calc(c, v) => (format!("{:#?}",c), *v)
        }
    }
    pub fn usize_id(&self) -> usize{
        match self{
            Self::Stat(a, _) => {let temp: usize = (*a).into(); CalcStat::NUM_VARIENTS + temp - Atrs::NUM_NON_STATS},
            Self::Calc(b, _) => b.clone() as usize
        }
    }
    pub fn from_usize_and_f32(id: usize, val: f32) -> Self{
        if id < CalcStat::NUM_VARIENTS{
            Self::Calc(CalcStat::VARIENTS[id], val)
        }else{
            Self::Stat(Atrs::VARIENTS[id-CalcStat::NUM_VARIENTS+Atrs::NUM_NON_STATS], val as i32)
        }
    }
    pub fn from_usize_and_i32(id: usize, val: i32) -> Self{
        if id < CalcStat::NUM_VARIENTS{
            Self::Calc(CalcStat::VARIENTS[id], val as f32)
        }else{
            Self::Stat(Atrs::VARIENTS[id-CalcStat::NUM_VARIENTS+Atrs::NUM_NON_STATS], val)
        }
    }
}
impl std::fmt::Debug for SearchReq{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",
        match self{
            Self::Stat(a, v) => format!("{} : {}", a, v),
            Self::Calc(s, v) => format!("{} : {}", s,v)
        })
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CalcStat{
    MeleeHit,
    MeleeDps,
    /// General spell damage (ie, the spell damage you would do without any spell-specific damage multipliers)
    SpGenDmg,
    Sp1Dmg,
    Sp2Dmg,
    Sp3Dmg,
    Sp4Dmg,
    Sp1PerSec,
    Sp2PerSec,
    Sp3PerSec,
    Sp4PerSec,
    Emr,
    Ehp,
    Ehpr
}

impl CalcStat{
    pub const VARIENTS: [Self; 14] = [Self::MeleeHit, Self::MeleeDps, Self::SpGenDmg, Self::Sp1Dmg, Self::Sp2Dmg, Self::Sp3Dmg, Self::Sp4Dmg, 
    Self::Sp1PerSec, Self::Sp2PerSec, Self::Sp3PerSec, Self::Sp4PerSec, Self::Emr, Self::Ehp, Self::Ehpr];
    pub const NUM_VARIENTS: usize = Self::VARIENTS.len();
    pub fn ord_fn_f32(&self) -> fn(&WynnBuild) -> f32{
        match self{
            Self::MeleeHit => |bld| bld.calc_melee_dam(false),
            Self::MeleeDps => |bld| bld.calc_melee_dam(true),
            Self::SpGenDmg => |bld| bld.calc_spell_dam(100),
            Self::Sp1Dmg => |bld| bld.calc_spell_dam(0),
            Self::Sp2Dmg => |bld| bld.calc_spell_dam(1),
            Self::Sp3Dmg => |bld| bld.calc_spell_dam(2),
            Self::Sp4Dmg => |bld| bld.calc_spell_dam(3),
            Self::Sp1PerSec => |bld| bld.spell_per_second(0, false),
            Self::Sp2PerSec => |bld| bld.spell_per_second(1, false),
            Self::Sp3PerSec => |bld| bld.spell_per_second(2, false),
            Self::Sp4PerSec => |bld| bld.spell_per_second(3, false),
            Self::Emr => |bld| bld.calc_emr(),
            Self::Ehp => |bld| bld.calc_max_ehp(),
            Self::Ehpr => |bld| bld.calc_ehpr(),
        }
    }
    pub fn ord_fn_i32(&self) -> fn(&WynnBuild) -> i32{
        match self{
            Self::MeleeHit => |bld| bld.calc_melee_dam(false) as i32,
            Self::MeleeDps => |bld| bld.calc_melee_dam(true) as i32,
            Self::SpGenDmg => |bld| bld.calc_spell_dam(100) as i32,
            Self::Sp1Dmg => |bld| bld.calc_spell_dam(0) as i32,
            Self::Sp2Dmg => |bld| bld.calc_spell_dam(1) as i32,
            Self::Sp3Dmg => |bld| bld.calc_spell_dam(2) as i32,
            Self::Sp4Dmg => |bld| bld.calc_spell_dam(3) as i32,
            Self::Sp1PerSec => |bld| (bld.spell_per_second(0, false) * 100.0) as i32,
            Self::Sp2PerSec => |bld| (bld.spell_per_second(1, false) * 100.0) as i32,
            Self::Sp3PerSec => |bld| (bld.spell_per_second(2, false) * 100.0) as i32,
            Self::Sp4PerSec => |bld| (bld.spell_per_second(3, false) * 100.0) as i32,
            Self::Emr => |bld| bld.calc_emr() as i32,
            Self::Ehp => |bld| bld.calc_max_ehp() as i32,
            Self::Ehpr => |bld| bld.calc_ehpr() as i32,
        }
    }
}
impl std::fmt::Display for CalcStat{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",match self{
            Self::MeleeHit=>"Melee Single Hit",
            Self::MeleeDps => "Melee Dps",
            Self::SpGenDmg => "Base Spell Damage",
            Self::Sp1Dmg => "Spell 1 Damage",
            Self::Sp2Dmg => "Spell 2 Damage",
            Self::Sp3Dmg => "Spell 3 Damage",
            Self::Sp4Dmg => "Spell 4 Damage",
            Self::Sp1PerSec => "Spell 1 / sec",
            Self::Sp2PerSec => "Spell 2 / sec",
            Self::Sp3PerSec => "Spell 3 / sec",
            Self::Sp4PerSec => "Spell 4 / sec",
            Self::Emr => "Effective Mana Regen",
            Self::Ehp => "Effective HP", 
            Self::Ehpr => "Effective HP Regen", 
        })
    }
}