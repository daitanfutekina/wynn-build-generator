// use std::{collections::BinaryHeap, default};

// use crate::items::{self, AtkSpd, Atrs, Category, Sets, Skill, Type, WynnItem, NUM_ITEM_IDS};

// /// convert # skill points to percent (ie, 150 strength = 80.8% damage boost)
// pub fn raw_skill_pct(pts: i32) -> f32 {
//     if pts <= 0 {
//         return 0.0;
//     };
//     let r: f32 = 0.9908;
//     return (r / (1.0 - r) * (1.0 - r.powi(pts))) / 100.0;
// }

// /// handles all skill point to percent conversions (int, def, and agi have slightly different curves from the default)
// pub fn skill_to_pct(skill: Skill, amt: i32) -> f32 {
//     match skill{
//         Skill::Str => raw_skill_pct(amt.min(150)),
//         Skill::Dex => raw_skill_pct(amt.min(150)),
//         Skill::Int => raw_skill_pct(amt.min(150))*0.6190092383711963,
//         Skill::Def => raw_skill_pct(amt.min(150))*0.867,
//         Skill::Agi => raw_skill_pct(amt.min(150))*0.951,
//     }
// }

// pub fn get_health_at_level(level: u32) -> i32{
//     ((level+1)*5) as i32
// }

// pub fn atk_spd_mult(spd: AtkSpd) -> f32{
//     [0.51, 0.83, 1.5, 2.05, 2.5, 3.1, 4.3][(spd as usize).clamp(0,6)]
// }

// #[derive(Clone)]
// pub struct WynnBuild {
//     ids: [i32; items::NUM_ITEM_IDS],
//     dams: [(u32, u32); 6],
//     pub skills: [i32; 5],
//     assigned_skills: [i32; 5],
//     free_sps: i32,
//     items: [WynnItem; 9],
// }
// impl WynnBuild {
//     pub fn from_names(item_names: &[&str], free_sps: i32) -> Option<WynnBuild>{
//         let items: Vec<WynnItem> = item_names.iter().map(|name| items::with_name(name).unwrap()).collect();
//         Self::from(&items, free_sps)
//     }
//     pub fn from_refs(items: &[&WynnItem], free_sps: i32) -> Option<WynnBuild> {
//         match WynnBuild::setup_skillpoints_refs(items, free_sps) {
//             Some(s) => {
//                 let mut res: WynnBuild = WynnBuild {
//                     ids: [0; items::NUM_ITEM_IDS],
//                     dams: [(0, 0); 6],
//                     skills: s.1,
//                     free_sps: s.0,
//                     assigned_skills: s.2,
//                     items: [WynnItem::null(),WynnItem::null(),WynnItem::null(),WynnItem::null(),WynnItem::null(),WynnItem::null(),WynnItem::null(),WynnItem::null(),WynnItem::null()],
//                 };
//                 for item in items {
//                     if !item.is_null(){
//                         res.add_item(item)
//                     }
//                 }
//                 Some(res)
//             }
//             None => None,
//         }
//     }
//     pub fn from(items: &[WynnItem], free_sps: i32) -> Option<WynnBuild> {
//         match WynnBuild::setup_skillpoints(items, free_sps) {
//             Some(s) => {
//                 let mut res: WynnBuild = WynnBuild {
//                     ids: [0; items::NUM_ITEM_IDS],
//                     dams: [(0, 0); 6],
//                     skills: s.1,
//                     free_sps: s.0,
//                     assigned_skills: s.2,
//                     items: [WynnItem::null(),WynnItem::null(),WynnItem::null(),WynnItem::null(),WynnItem::null(),WynnItem::null(),WynnItem::null(),WynnItem::null(),WynnItem::null()],
//                 };
//                 let mut num_of_sets: Vec<(Sets, u8)> = Vec::new();
//                 for item in items {
//                     if !item.is_null() {
//                         let itm_set = item.get_set();
//                         if itm_set != Sets::None{
//                             match num_of_sets.iter().find(|(s, q)| itm_set==*s){
//                                 Some((s,q)) => {

//                                 }None => {
                                    
//                                 }
//                             }
//                         }
//                         res.add_item(item)
//                     }
//                 }
//                 Some(res)
//             }
//             None => None,
//         }
//     }
//     pub fn from_with_skills_and_ids(items: &[&WynnItem], free_sps: i32, base_skills: &[i32; 5], base_ids: &[i32; NUM_ITEM_IDS]) -> Option<WynnBuild> {
//         match WynnBuild::setup_skillpoints_with_base_skills(items, free_sps, base_skills) {
//             Some(s) => {
//                 let mut res: WynnBuild = WynnBuild {
//                     ids: base_ids.clone(),
//                     dams: [(0, 0); 6],
//                     skills: s.1,
//                     free_sps: s.0,
//                     assigned_skills: s.2,
//                     items: [WynnItem::null(),WynnItem::null(),WynnItem::null(),WynnItem::null(),WynnItem::null(),WynnItem::null(),WynnItem::null(),WynnItem::null(),WynnItem::null()],
//                 };
//                 for item in items {
//                     if !item.is_null(){
//                         res.add_item(item)
//                     }
//                 }
//                 Some(res)
//             }
//             None => None,
//         }
//     }
//     pub fn from_with_skills(items: &[&WynnItem], free_sps: i32, base_skills: &[i32; 5]) -> Option<WynnBuild> {
//         match WynnBuild::setup_skillpoints_with_base_skills(items, free_sps,base_skills) {
//             Some(s) => {
//                 let mut res: WynnBuild = WynnBuild {
//                     ids: [0; items::NUM_ITEM_IDS],
//                     dams: [(0, 0); 6],
//                     skills: s.1,
//                     free_sps: s.0,
//                     assigned_skills: s.2,
//                     items: [WynnItem::null(),WynnItem::null(),WynnItem::null(),WynnItem::null(),WynnItem::null(),WynnItem::null(),WynnItem::null(),WynnItem::null(),WynnItem::null()],
//                 };
//                 for item in items {
//                     if !item.is_null() {
//                         res.add_item(item)
//                     }
//                 }
//                 Some(res)
//             }
//             None => None,
//         }
//     }
//     fn add_item(&mut self, item: &WynnItem) {
//         for (i, d) in item.iter_damages() {
//             self.dams[i as usize].0 += d.0;
//             self.dams[i as usize].1 += d.1;
//         }
//         for d in item.iter_ids() {
//             self.ids[d.0 as usize - items::NUM_NON_IDS] += d.1;
//         }
//         let type_usize = item.get_type() as usize;
//         self.items[if type_usize>4 || type_usize==4 && !self.items[4].is_null() {(type_usize+1).min(8)} else {type_usize}] = item.clone();
//     }
//     pub fn setup_skillpoints_refs(items: &[&WynnItem], extra_sps: i32) -> Option<(i32,[i32; 5],[i32;5])> {
//         let mut v: [BinaryHeap<(i32, i32)>; 5] = [
//             BinaryHeap::with_capacity(8),
//             BinaryHeap::with_capacity(8),
//             BinaryHeap::with_capacity(8),
//             BinaryHeap::with_capacity(8),
//             BinaryHeap::with_capacity(8),
//         ];
//         let mut extra_skill_pts = extra_sps;
//         let mut skills = [0, 0, 0, 0, 0];
//         let mut assigned_skills = [0,0,0,0,0];
//         let mut weapon_skill_data = [(0,0);5];
//         let mut set_sp_bonus: Vec<(Sets,Vec<[u32; 5]>)> = Vec::new();
//         for item in items {
//             if item.is_null(){continue}
//             let set = item.get_set();
//             if set!=Sets::None{
//                 // match set_sp_bonus.iter().find(|(s, v)| set==*s){
//                 //     Some((s, v)) => v.append,
//                 //     None =>
//                 // }
//             }
//             if item.get_category()==Category::Weapon{
//                 for (skill, req, bonus) in item.iter_skills(){
//                     weapon_skill_data[skill as usize] = (req,bonus);
//                 }
//                 continue
//             }
//             for (skill, req, bonus) in item.iter_skills(){
//                 if req>0{
//                     v[skill as usize].push((-req,bonus))
//                 } else{
//                     skills[skill as usize]+=bonus;
//                 }
//             }
//         }
//         for skill_type in 0..5_usize {
//             while !v[skill_type].is_empty() {
//                 let temp = v[skill_type].pop().unwrap();
//                 let diff = skills[skill_type] + temp.0;
//                 if diff < 0 {
//                     extra_skill_pts += diff;
//                     skills[skill_type] -= diff;
//                     assigned_skills[skill_type]-=diff;
//                 }
//                 if extra_skill_pts < 0 || assigned_skills[skill_type]>100 {
//                     return None;
//                 }
//                 skills[skill_type] = (skills[skill_type]+temp.1).min(150);
//             }
//             let weap_diff = skills[skill_type]-weapon_skill_data[skill_type].0;
//             if weapon_skill_data[skill_type].0>0 && weap_diff<0{
//                 extra_skill_pts += weap_diff;
//                 skills[skill_type] -= weap_diff;
//                 assigned_skills[skill_type]-=weap_diff;
//             }
//             if extra_skill_pts < 0 || assigned_skills[skill_type]>100{
//                 return None;
//             }
//             skills[skill_type]=(skills[skill_type]+weapon_skill_data[skill_type].1).min(150);
//         }
//         Some((extra_skill_pts,skills,assigned_skills))
//     }
//     pub fn setup_skillpoints(items: &[WynnItem], extra_sps: i32) -> Option<(i32,[i32; 5],[i32;5])> {
//         let mut v: [BinaryHeap<(i32, i32)>; 5] = [
//             BinaryHeap::with_capacity(8),
//             BinaryHeap::with_capacity(8),
//             BinaryHeap::with_capacity(8),
//             BinaryHeap::with_capacity(8),
//             BinaryHeap::with_capacity(8),
//         ];
//         let mut extra_skill_pts = extra_sps;
//         let mut skills = [0, 0, 0, 0, 0];
//         let mut assigned_skills = [0,0,0,0,0];
//         let mut weapon_skill_data = [(0,0);5];
//         for item in items {
//             if item.is_null(){continue}
//             if item.get_category()==Category::Weapon{
//                 for (skill, req, bonus) in item.iter_skills(){
//                     weapon_skill_data[skill as usize] = (req,bonus);
//                 }
//                 continue
//             }
//             for (skill, req, bonus) in item.iter_skills(){
//                 if req>0{
//                     v[skill as usize].push((-req,bonus))
//                 } else{
//                     skills[skill as usize]+=bonus;
//                 }
//             }
//         }
//         for skill_type in 0..5_usize {
//             while !v[skill_type].is_empty() {
//                 let temp = v[skill_type].pop().unwrap();
//                 let diff = skills[skill_type] + temp.0;
//                 if diff < 0 {
//                     extra_skill_pts += diff;
//                     skills[skill_type] -= diff;
//                     assigned_skills[skill_type]-=diff;
//                 }
//                 if extra_skill_pts < 0 || assigned_skills[skill_type]>100 {
//                     return None;
//                 }
//                 skills[skill_type] = (skills[skill_type]+temp.1).min(150);
//             }
//             let weap_diff = skills[skill_type]-weapon_skill_data[skill_type].0;
//             if weapon_skill_data[skill_type].0>0 && weap_diff<0{
//                 extra_skill_pts += weap_diff;
//                 skills[skill_type] -= weap_diff;
//                 assigned_skills[skill_type]-=weap_diff;
//             }
//             if extra_skill_pts < 0 || assigned_skills[skill_type]>100{
//                 return None;
//             }
//             skills[skill_type]=(skills[skill_type]+weapon_skill_data[skill_type].1).min(150);
//         }
//         Some((extra_skill_pts,skills,assigned_skills))
//     }
//     pub fn setup_skillpoints_with_base_skills(items: &[&WynnItem], extra_sps: i32, base_skills: &[i32; 5]) -> Option<(i32,[i32; 5],[i32;5])> {
//         let mut v: [BinaryHeap<(i32, i32)>; 5] = [
//             BinaryHeap::with_capacity(8),
//             BinaryHeap::with_capacity(8),
//             BinaryHeap::with_capacity(8),
//             BinaryHeap::with_capacity(8),
//             BinaryHeap::with_capacity(8),
//         ];
//         let mut extra_skill_pts = extra_sps;
//         let mut skills = base_skills.clone();
//         let mut assigned_skills = [0,0,0,0,0];
//         let mut weapon_skill_data = [(0,0);5];
//         for item in items {
//             if item.is_null(){continue}
//             if item.get_category()==Category::Weapon{
//                 for (skill, req, bonus) in item.iter_skills(){
//                     weapon_skill_data[skill as usize] = (req,bonus);
//                 }
//                 continue
//             }
//             for (skill, req, bonus) in item.iter_skills(){
//                 if req>0{
//                     v[skill as usize].push((-req,bonus))
//                 } else{
//                     skills[skill as usize]+=bonus;
//                 }
//             }
//         }
//         for skill_type in 0..5_usize {
//             while !v[skill_type].is_empty() {
//                 let temp = v[skill_type].pop().unwrap();
//                 let diff = skills[skill_type] + temp.0;
//                 if diff < 0 {
//                     extra_skill_pts += diff;
//                     skills[skill_type] -= diff;
//                     assigned_skills[skill_type]-=diff;
//                 }
//                 if extra_skill_pts < 0 || assigned_skills[skill_type]>100 {
//                     return None;
//                 }
//                 skills[skill_type] = (skills[skill_type]+temp.1).min(150);
//             }
//             let weap_diff = skills[skill_type]-weapon_skill_data[skill_type].0;
//             if weapon_skill_data[skill_type].0>0 && weap_diff<0{
//                 extra_skill_pts += weap_diff;
//                 skills[skill_type] -= weap_diff;
//                 assigned_skills[skill_type]-=weap_diff;
//             }
//             if extra_skill_pts < 0 || assigned_skills[skill_type]>100{
//                 return None;
//             }
//             skills[skill_type]=(skills[skill_type]+weapon_skill_data[skill_type].1).min(150);
//         }
//         Some((extra_skill_pts,skills,assigned_skills))
//     }
//     pub fn get_stat(&self, atr: Atrs) -> i32 {
//         return self.ids[atr as usize-items::NUM_NON_IDS];
//     }
//     pub fn calc_ehp(&self, use_air: bool) -> f32{
//         let hp = (self.get_stat(Atrs::Hp)+self.get_stat(Atrs::HpBonus)+get_health_at_level(106)) as f32;
//         let class_mult: f32 = match self.items.last().unwrap_or(&WynnItem::from(1034)).get_type(){
//             Type::Wand => 5.0/6.0,
//             Type::Relik => 5.0/7.0,
//             Type::Bow => 10.0/13.0,
//             _ => 1.0
//         };
//         (hp/(0.1*skill_to_pct(Skill::Agi, self.skills[4])+(1.0-skill_to_pct(Skill::Agi, self.skills[4]))*(1.0-skill_to_pct(Skill::Def, self.skills[3]))))*class_mult
//     }
//     /// Calculates maximum ehp of the build by dumping all free skill points into defence, or into agility if def < 0
//     /// 
//     /// Instead, this function should determine the exact def and agi to maximize ehp, but that's more work and would run slower...
//     pub fn calc_max_ehp(&self, use_air: bool) -> f32{
//         let hp = (self.get_stat(Atrs::Hp)+self.get_stat(Atrs::HpBonus)+get_health_at_level(106)) as f32;
//         let class_mult: f32 = match self.items.last().unwrap().get_type(){
//             Type::Wand => 5.0/6.0,
//             Type::Relik => 5.0/7.0,
//             Type::Bow => 10.0/13.0,
//             _ => 1.0
//         };
//         if self.skills[3]<0{
//             // let agi = (self.skills[4]+(100-self.assigned_skills[4]).min(self.free_sps).max(0)).min(150);
//             let agi = (self.skills[4]+(100-self.assigned_skills[4]).clamp(0,self.free_sps)).min(150);
//             let def = (self.free_sps-agi+self.assigned_skills[4]).clamp(0,150);
//             (hp/(0.1*skill_to_pct(Skill::Agi, agi)+(1.0-skill_to_pct(Skill::Agi, agi))*(1.0-skill_to_pct(Skill::Def, def))))*class_mult
//         }else{
//             // let def = (self.skills[3]+(100-self.assigned_skills[3]).min(self.free_sps).max(0)).min(150);
//             let def = (self.skills[3]+(100-self.assigned_skills[3]).clamp(0,self.free_sps)).min(150);
//             // let agi = (self.skills[4]+(100-self.assigned_skills[4]).min(self.free_sps-def+self.skills[3]).max(0)).min(150);
//             let agi = (self.skills[4]+(100-self.assigned_skills[4]).clamp(0,self.free_sps-def+self.skills[3])).min(150);
//             (hp/(0.1*skill_to_pct(Skill::Agi, agi)+(1.0-skill_to_pct(Skill::Agi, agi))*(1.0-skill_to_pct(Skill::Def, def))))*class_mult
//         }
//     }
//     pub fn calc_max_ehp2(&self, use_air: bool) -> f32{
//         let hp = (self.get_stat(Atrs::Hp)+self.get_stat(Atrs::HpBonus)+get_health_at_level(106)) as f32;
//         let class_mult: f32 = match self.items.last().unwrap().get_type(){
//             Type::Wand => 5.0/6.0,
//             Type::Relik => 5.0/7.0,
//             Type::Bow => 10.0/13.0,
//             _ => 1.0
//         };
//         if self.skills[3]<0{
//             let agi = (self.skills[4]+(100-self.assigned_skills[4]).min(self.free_sps).max(0)).min(150);
//             let def = (self.free_sps-agi+self.assigned_skills[4]).max(0);
//             println!("{}, {}",def,agi);
//             (hp/(0.1*skill_to_pct(Skill::Agi, agi)+(1.0-skill_to_pct(Skill::Agi, agi))*(1.0-skill_to_pct(Skill::Def, def))))*class_mult
//         }else{
//             let def = (self.skills[3]+(100-self.assigned_skills[3]).min(self.free_sps).max(0)).min(150);
//             let agi = (self.skills[4]+(100-self.assigned_skills[4]).min(self.free_sps-def+self.skills[3]).max(0)).min(150);
//             println!("{}, {}, skills: {}, {}, assinged: {}, {}, hp: {}",def,agi,self.skills[3],self.skills[4],self.assigned_skills[3],self.assigned_skills[4], hp);
//             (hp/(0.1*skill_to_pct(Skill::Agi, agi)+(1.0-skill_to_pct(Skill::Agi, agi))*(1.0-skill_to_pct(Skill::Def, def))))*class_mult
//         }
//     }
//     fn calc_max_skill(&self, skill_idx: usize, free_sps: i32) -> i32{
//         ((100-self.assigned_skills[skill_idx]).clamp(0,free_sps)).min(150)
//             // let agi = (self.skills[4]+(100-self.assigned_skills[4]).min(self.free_sps-def+self.skills[3]).max(0)).min(150);
//             // let agi = (self.skills[4]+(100-self.assigned_skills[4]).clamp(0,self.free_sps-def+self.skills[3])).min(150);
//     }
//     pub fn calc_melee_dam(&self) -> f32{
//         // let mut avg = (0.0,0.0);
//         let m: DamageData = DamageData::melee(self,false);
//         self.calc_dam(m, false,[1.0,0.0,0.0,0.0,0.0,0.0],1) * atk_spd_mult(self.overall_atk_spd())
//         // println!("pcts {:#?}",m.ele_dam_pcts);
//         // println!("skills {:#?}",self.skills);
//         // println!("melee percent: {}, {}, melee raw: {}, rainbow {}",m.pct, self.get_stat(Atrs::MdPct),self.get_stat(Atrs::MdRaw),self.get_stat(Atrs::RDamPct));
//         // println!("test 40 {} {}",raw_skill_pct(40), skill_to_pct(Skill::Str, self.skills[0]));
//         // for i in 0_usize..6{
//         //     let mut pct_bonus = 1.0+m.pct+m.ele_dam_pcts[i];
//         //     let raw = (m.raw*(m.dams[i].0/m.total_dam.0)+m.ele_dam_raws[i],m.raw*(m.dams[i].1/m.total_dam.1)+m.ele_dam_raws[i]);
//         //     if i>0{ // adds rainbow damage
//         //         pct_bonus+=m.ele_dam_pcts[6]+m.skill_dam_bonus[i-1];
//         //         println!("skill dam bon {}",m.skill_dam_bonus[i-1]);
//         //     }
//         //     println!("pcts bonus {}",pct_bonus);
//         //     // res[i]=((dams[i].0*pct_bonus+raw.0)*(1.0+skill_dam_bonus[0]),(dams[i].1*pct_bonus+raw.1)*(1.0+skill_dam_bonus[0]));
//         //     // res_crit[i]=((dams[i].0*pct_bonus+raw.0)*(2.0+skill_dam_bonus[0]),(dams[i].1*pct_bonus+raw.1)*(2.0+skill_dam_bonus[0]));
//         //     // res_avg[i]=((res_crit[i].0*skill_dam_bonus[1])+(res[i].0*(1.0-skill_dam_bonus[1])),(res_crit[i].1*skill_dam_bonus[1])+(res[i].1*(1.0-skill_dam_bonus[1])));
//         //     // avg.0+=res_avg[i].0;
//         //     // avg.1+=res_avg[i].1;
//         //     let mult = 1.0+m.skill_dam_bonus[0]+m.skill_dam_bonus[1];
//         //     avg.0+=(m.dams[i].0*pct_bonus+raw.0)*mult;
//         //     avg.1+=(m.dams[i].1*pct_bonus+raw.1)*mult;
//         //     // x * (1.0 + y) * c% + x * (2.0+y) * (1-c%) 
//         //     // res_crit[i]=(res[i].0*(1.0+skill_dam_bonus[1]),res[i].1*(1.0+skill_dam_bonus[1]))
//         // }
//         // // println!("{}",(avg.0+avg.1)/2.0);
//         // (avg.0+avg.1)/2.0
//     }
//     pub fn calc_spell_dam(&self) -> f32{
//         // let mut avg = (0.0,0.0);
//         let s: DamageData = DamageData::spell(self);
//         self.calc_dam(s, true, [1.2,0.0,0.3,0.0,0.0,0.0], 1)
//         // println!("pcts {:#?}",s.ele_dam_pcts);
//         // println!("skills {:#?}",self.skills);
//         // println!("melee percent: {}, {}, melee raw: {}, rainbow {}",s.pct, self.get_stat(Atrs::MdPct),self.get_stat(Atrs::MdRaw),self.get_stat(Atrs::RDamPct));
//         // println!("test 40 {} {}",raw_skill_pct(40), skill_to_pct(Skill::Str, self.skills[0]));
//         // let atk_spd_mul = atk_spd_mult(self.atk_spd());
//         // // let sp_mults = [1.2,0.0,0.3,0.0,0.0,0.0];
//         // let sp_mults = [0.25,0.05,0.0,0.0,0.0,0.05];
//         // let num_hits = 10.0;
//         // let mults_total: f32 = sp_mults.iter().sum();
//         // for i in 0_usize..6{
//         //     let mut pct_bonus = 1.0+s.pct+s.ele_dam_pcts[i];
//         //     let raw = (s.raw*(s.dams[i].0/s.total_dam.0)+s.ele_dam_raws[i],s.raw*(s.dams[i].1/s.total_dam.1)+s.ele_dam_raws[i]);
//         //     let mut weap_dam = (s.dams[i].0*sp_mults[0],s.dams[i].1*sp_mults[0]);
//         //     if i>0{ // adds rainbow damage
//         //         weap_dam.0+=sp_mults[i]*s.total_dam.0;
//         //         weap_dam.1+=sp_mults[i]*s.total_dam.1;
//         //         pct_bonus+=s.ele_dam_pcts[6]+s.skill_dam_bonus[i-1];
//         //     }
//         //     let mult = 1.0+s.skill_dam_bonus[0]+s.skill_dam_bonus[1];
//         //     avg.0+=(weap_dam.0*atk_spd_mul*pct_bonus+raw.0*mults_total)*mult;
//         //     avg.1+=(weap_dam.1*atk_spd_mul*pct_bonus+raw.1*mults_total)*mult;
//         //     // x * (1.0 + y) * c% + x * (2.0+y) * (1-c%) 
//         //     // res_crit[i]=(res[i].0*(1.0+skill_dam_bonus[1]),res[i].1*(1.0+skill_dam_bonus[1]))
//         // }
//         // // println!("{}",(avg.0+avg.1)/2.0);
//         // num_hits*(avg.0+avg.1)/2.0
//     }
//     fn calc_dam(&self, d: DamageData, sp_dam: bool, mults: [f32; 6], num_hits: i32) -> f32{
//         let mut avg = (0.0,0.0);
//         let atk_spd_mul = if sp_dam {atk_spd_mult(self.weapon_atk_spd())} else {1.0};
//         let mults_total: f32 = mults.iter().sum();
//         for i in 0_usize..6{
//             let mut pct_bonus = 1.0+d.pct+d.ele_dam_pcts[i];
//             let raw = (d.raw*(d.dams[i].0/d.total_dam.0)+d.ele_dam_raws[i],d.raw*(d.dams[i].1/d.total_dam.1)+d.ele_dam_raws[i]);
//             let mut weap_dam = (d.dams[i].0*mults[0],d.dams[i].1*mults[0]);
//             if i>0{ // adds rainbow damage
//                 weap_dam.0+=mults[i]*d.total_dam.0;
//                 weap_dam.1+=mults[i]*d.total_dam.1;
//                 pct_bonus+=d.ele_dam_pcts[6]+d.skill_dam_bonus[i-1];
//             }
//             let mult = 1.0+d.skill_dam_bonus[0]+d.skill_dam_bonus[1];
//             // avg.0+=(weap_dam.0*atk_spd_mul*pct_bonus+raw.0*mults_total)*mult;
//             // avg.1+=(weap_dam.1*atk_spd_mul*pct_bonus+raw.1*mults_total)*mult;
//             avg.0+=(weap_dam.0*atk_spd_mul*pct_bonus+raw.0*mults_total)*mult;
//             avg.1+=(weap_dam.1*atk_spd_mul*pct_bonus+raw.1*mults_total)*mult;
//             // x * (1.0 + y) * c% + x * (2.0+y) * (1-c%) 
//             // res_crit[i]=(res[i].0*(1.0+skill_dam_bonus[1]),res[i].1*(1.0+skill_dam_bonus[1]))
//         }
//         // println!("{}",(avg.0+avg.1)/2.0);
//         num_hits as f32*(avg.0+avg.1)/2.0
//     }
//     /// Returns a splice of length len containing the ids starting from the given id (ie, `ids_splice(Atrs::EDamPct, 6)` returns all the **elemental damage percents** (including rainbow at index 6))
//     /// You should make sure you know the order of the Atrs enum to use this function
//     fn ids_splice(&self, start: Atrs, len: usize) -> &[i32]{
//         let from = start as usize - items::NUM_NON_IDS;
//         &self.ids[from..from+len]
//     }
//     pub fn overall_atk_spd(&self) -> AtkSpd{
//         AtkSpd::from_u32((self.items.last().unwrap_or(&WynnItem::null()).atk_spd() as i32 + self.get_stat(Atrs::AtkTier)).clamp(0,6) as u32).unwrap_or(AtkSpd::Normal)
//     }
//     pub fn weapon_atk_spd(&self) -> AtkSpd{
//         self.items.last().unwrap_or(&WynnItem::null()).atk_spd()
//     }
//     pub fn generate_hash(&self) -> String {
//         self.items.iter().enumerate().map(|(t, item)| {if item.is_null() {items::url_hash_val(10000 + t as i32, 3)} else {item.get_hash()}
//             }).collect::<String>()+ &self.skills.iter().map(|s| items::url_hash_val(*s, 2)).collect::<String>()
//     }
//     pub fn item_names(&self) -> String{
//         self.items.iter().enumerate().map(|(t, item)| {if item.is_null() {"null ".to_string()} else {item.name().to_string()+" "}}).collect::<String>()
//     }
// }

// struct DamageData{
//     /// elemental damage percents, including neutral at [0] and rainbow at [6]
//     ele_dam_pcts: [f32; 7],
//     /// raw elemental damages, including neutral at [0] and rainbow at [6]
//     ele_dam_raws: [f32; 7],
//     dams: [(f32,f32); 6],
//     total_dam: (f32, f32),
//     skill_dam_bonus: [f32; 5],
//     raw: f32,
//     pct: f32
// }
// impl DamageData{
//     fn melee(bld: &WynnBuild, use_atk_spd: bool) -> Self{
//         let dam_pcts_splice = bld.ids_splice(Atrs::NDamPct, 7);
//         let dam_raws_splice = bld.ids_splice(Atrs::NDamRaw, 7);
//         let mdam_pcts_splice = bld.ids_splice(Atrs::NMdPct,7);
//         let mdam_raws_splice = bld.ids_splice(Atrs::NMdRaw,7);
//         let mut dam_pcts: [f32; 7] = [0.0;7];
//         let mut dam_raws: [f32; 7] = [0.0;7];
//         let mut dams: [(f32,f32); 6] = [(0.0,0.0);6];
//         let mut total_dam: (f32, f32) = (0.0,0.0);
//         let mut skill_dam_bonus: [f32; 5] = [0.0; 5];
//         let mut sp_bonus = [bld.calc_max_skill(0, bld.free_sps), 0];
//         sp_bonus[1] = bld.calc_max_skill(1, bld.free_sps-sp_bonus[0]);
//         for i in 0_usize..7{
//             if i<6{
//                 dams[i]=(bld.dams[i].0 as f32, bld.dams[i].1 as f32);
//                 total_dam.0+=dams[i].0;
//                 total_dam.1+=dams[i].1;
//                 if i<5{
//                     skill_dam_bonus[i]=skill_to_pct(Skill::from_usize(if i==2{0}else{i}).unwrap(), if i<2{bld.skills[i]+sp_bonus[i]}else{bld.skills[i]});
//                 }
//             }
//             dam_pcts[i]=(dam_pcts_splice[i] + mdam_pcts_splice[i]) as f32 / 100.0;
//             dam_raws[i]=(dam_raws_splice[i]+mdam_raws_splice[i]) as f32;
//         }
//         let melee_raw = bld.get_stat(Atrs::MdRaw) as f32;
//         let melee_pct = bld.get_stat(Atrs::MdPct) as f32 / 100.0;
//         Self{ele_dam_pcts: dam_pcts, ele_dam_raws: dam_raws, dams, total_dam, skill_dam_bonus, raw: melee_raw, pct: melee_pct}
//     }
//     fn spell(bld: &WynnBuild) -> Self{
//         let dam_pcts_splice = bld.ids_splice(Atrs::NDamPct, 7);
//         let dam_raws_splice = bld.ids_splice(Atrs::NDamRaw, 7);
//         let sdam_pcts_splice = bld.ids_splice(Atrs::NSdPct,7);
//         let sdam_raws_splice = bld.ids_splice(Atrs::NSdRaw,7);
//         let mut dam_pcts: [f32; 7] = [0.0;7];
//         let mut dam_raws: [f32; 7] = [0.0;7];
//         let mut dams: [(f32,f32); 6] = [(0.0,0.0);6];
//         let mut total_dam: (f32, f32) = (0.0,0.0);
//         let mut skill_dam_bonus: [f32; 5] = [0.0; 5];
//         let mut sp_bonus = [bld.calc_max_skill(0, bld.free_sps), 0];
//         sp_bonus[1] = bld.calc_max_skill(1, bld.free_sps-sp_bonus[0]);
//         // let atk_spd_mul = atk_spd_mult(bld.atk_spd());
//         for i in 0_usize..7{
//             if i<6{
//                 dams[i]=(bld.dams[i].0 as f32, bld.dams[i].1 as f32);
//                 total_dam.0+=dams[i].0;
//                 total_dam.1+=dams[i].1;
//                 if i<5{
//                     skill_dam_bonus[i]=skill_to_pct(Skill::from_usize(if i==2{0}else{i}).unwrap(), if i<2{bld.skills[i]+sp_bonus[i]}else{bld.skills[i]});
//                 }
//             }
//             dam_pcts[i]=(dam_pcts_splice[i] + sdam_pcts_splice[i]) as f32 / 100.0;
//             dam_raws[i]=(dam_raws_splice[i]+sdam_raws_splice[i]) as f32;
//         }
//         let spell_raw = bld.get_stat(Atrs::SdRaw) as f32;
//         let spell_pct = bld.get_stat(Atrs::SdPct) as f32 / 100.0;
//         Self{ele_dam_pcts: dam_pcts, ele_dam_raws: dam_raws, dams, total_dam, skill_dam_bonus, raw: spell_raw, pct: spell_pct}
//     }
// }
