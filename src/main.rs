// mod items;
// mod wynn_calcs;
use gloo::timers::callback::Timeout;
// use items::{Atrs, Category, Sets, Skill, Tier, Type, WynnItem, NUM_ITEM_IDS, NUM_NON_IDS};
use std::{future::Future, mem::size_of, ops::{Add, AddAssign, Neg, Sub, SubAssign}, task::Poll};
use web_sys::{console, Event, HtmlInputElement};
use yew::prelude::*;
// use gloo::timers::callback::{Interval, Timeout};

// use crate::WynnBuild;

mod wynn_data;
use wynn_data::{*, builder::WynnBuild, items::*, I12x5};

const AHH: &'static [Test2] = &[
    Test2 {
        a: "hello",
        b: &[2, 3, 4],
        c: Test::C,
    },
    Test2 {
        a: "world",
        b: &[5, 6, 7],
        c: Test::C,
    },
];
// new URLSearchParams(window.location.search);
fn main() {

    let start_localhost = false;
    if start_localhost{
        yew::start_app::<RootComponent>();
        return
    }

    println!("Hello, world!");
    println!("{}", size_of::<Atrs>());
    println!("{}", size_of::<(u8, i32)>());

    // let mut items_with_type: [Vec<usize>;13] = [Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new()];
    // for i in 0..12_usize{
    //     items_with_type[i]=items::with_prop_value(Atrs::Type(Type::from_num(i).unwrap()));
    // }

    // //value: (999999^0x00FFFFFF)+1; //(!999999_u32&0x7FFFFF)+1_u32

    let id = 250_u32; // first 8 bits represent the ID (unsigned)
    let value = -7999999_i32 as u32 & 0xFFFFFF; // last 24 bits is the value (signed)
    let idval = (id << 24) | value;
    let idval2 = idval as i32;
    println!(
        "id: {}, value: {}",
        idval >> 24,
        ((idval & 0x800000) * 0x1FF | idval & 0xFFFFFF) as i32
    );
    println!("2: id: {}, value: {}",idval >> 24,(idval as i32)<<8>>8);

    let sps = [16,28,39,21,32];
    let sps2 = [-23,-2,-40,-1,-2];
    let sps3 = [23,2,40,1,2];

    let mut compressed_sps: i64 = (sps[0] as i64 & 0x7FF)<<48 | (sps[1] as i64 & 0x7FF)<<36 | (sps[2] as i64 & 0x7FF)<<24 | (sps[3] as i64 & 0x7FF)<<12 | (sps[4] as i64 & 0x7FF);
    let mut compressed_sps2: i64 = (sps2[0] as i64 & 0x7FF)<<48 | (sps2[1] as i64 & 0x7FF)<<36 | (sps2[2] as i64 & 0x7FF)<<24 | (sps2[3] as i64 & 0x7FF)<<12 | (sps2[4] as i64 & 0x7FF);

    let mut compressed_sps3: i64 = (sps[0] as i64 & 0xFFF)<<52 | (sps[1] as i64 & 0xFFF)<<39 | (sps[2] as i64 & 0xFFF)<<26 | (sps[3] as i64 & 0xFFF)<<13 | (sps[4] as i64 & 0xFFF);
    let mut compressed_sps4: i64 = (sps2[0] as i64 & 0xFFF)<<52 | (sps2[1] as i64 & 0xFFF)<<39 | (sps2[2] as i64 & 0xFFF)<<26 | (sps2[3] as i64 & 0xFFF)<<13 | (sps2[4] as i64 & 0xFFF);

    // compressed_sps=(compressed_sps+compressed_sps2); // & 0x7FF7FF7FF7FF7FF
    compressed_sps3=(compressed_sps3+compressed_sps4) & -2252074725150721; //0xFFF7FFBFFDFFEFFF;
// ((compressed_sps>>12)&0x800*-1)&0xFFF
    // println!("{} {} {} {} {}",(compressed, ((compressed_sps>>36)&0xFFF) - 2048, ((compressed_sps>>24)&0xFFF) - 2048, ((compressed_sps>>12)&0xFFF) - 2048, (compressed_sps&0xFFF) - 2048);

    // println!("{} {} {} {} {}",((compressed_sps2>>48) & 0xFFF) - 2048, ((compressed_sps2>>36)&0xFFF) - 2048, ((compressed_sps2>>24)&0xFFF) - 2048, ((compressed_sps2>>12)&0xFFF) - 2048, (compressed_sps2&0xFFF) - 2048);

    let temp = [compressed_sps3>>52,(compressed_sps3>>39)&0xFFF,(compressed_sps3>>26)&0xFFF,(compressed_sps3>>39)&0xFFF,(compressed_sps3>>39)&0xFFF];
    println!("{} {} {} {} {}",((compressed_sps3>>52) & 0x800) * 0x1FFFFFFFFFFFF | (compressed_sps3>>52) & 0xFFF, ((compressed_sps3>>39)&0xFFF), ((compressed_sps3>>26)&0xFFF), ((compressed_sps3>>13)&0xFFF), (compressed_sps3&0xFFF));
    println!("{} {} {} {} {} {}",(compressed_sps3<<52>>52)+(compressed_sps3<<39>>52)+(compressed_sps3<<26>>52)+(compressed_sps3<<13>>52)+(compressed_sps3>>52),compressed_sps3>>52, compressed_sps3<<13>>52, compressed_sps3<<26>>52, compressed_sps3<<39>>52, compressed_sps3<<52>>52);

    // let test = SkillPts::from(sps);
    // let test2 = SkillPts::from(sps2);
    // let test3 = SkillPts::from(sps3);

    // let test4 = -test;

    // println!("{}",test+test2);
    // println!("{}, {}",test-test3,(test-test3).get_skill(Skill::Str));
    // println!("{}",test4);
    // println!("{:?}",sps);
    println!("{}",-wynn_data::I12x5::from([2047,-23,-43,-12,98]).get_pos());
    println!("{}",wynn_data::I12x5::from([5,-23,43,-12,98]).get_negs());
    println!("{}",wynn_data::I12x5::from([0,0,0,0,0]).is_neg());

    println!("{}",wynn_data::I12x5::from([148,151,-15,151,151]).with_max(150));
    println!("{}",wynn_data::I12x5::from([-1,151,-15,151,-25]).with_min(0));
    println!("ahhh {}",(wynn_data::I12x5::from([-1024, 30, 30, -1024, 30])-wynn_data::I12x5::from([33, 33, 33, 33, 33])).get_negs());



    let mut tesbld = WynnBuild::make_from_names(&"Dune Storm,Dondasch,Horizon,Revenant,Dispersion,Dispersion,Knucklebones,Achromatic Gloom,Oak Wood Spear".split(",").collect::<Vec<&str>>(),106).unwrap();
    println!("made test bld {}",tesbld.item_names());
    println!("test? dam {} {:?}",tesbld.calc_melee_dam(),tesbld.skills);
    println!("test2w? {}",items::with_name("Spring").unwrap().get_skill_reqs());
    println!("{}",wynn_data::I12x5::from([148,-151,50,151,28]).max(wynn_data::I12x5::from([120,30,-15,24,11])));


    tesbld = WynnBuild::from_test(&[WynnItem::from_idx(2988),WynnItem::from_idx(3632),WynnItem::from_idx(2793),WynnItem::from_idx(983),WynnItem::from_idx(1678),WynnItem::from_idx(2864),WynnItem::from_idx(2641),WynnItem::from_idx(3110),WynnItem::from_idx(1302)],201,I12x5::ZERO).unwrap();
    println!("tesbld1 skills: {:?} {}",tesbld.skills,tesbld.generate_hash());
    // !n + 1 = !(n-1)
    tesbld = WynnBuild::make_with_free_spts(&[WynnItem::from_idx(2988),WynnItem::from_idx(3632),WynnItem::from_idx(2793),WynnItem::from_idx(983),WynnItem::from_idx(1678),WynnItem::from_idx(2864),WynnItem::from_idx(2641),WynnItem::from_idx(3110),WynnItem::from_idx(1302)],201).unwrap();
    println!("tesbld1 skills 2: {:?} {} {}",tesbld.skills, tesbld.skills.iter::<i32>().sum::<i32>(),tesbld.generate_hash());

    tesbld = WynnBuild::from_test(&[items::with_name("Aphotic").unwrap(),items::with_name("Starglass").unwrap(),items::with_name("Vaward").unwrap(),items::with_name("Memento").unwrap(),items::with_name("Draoi Fair").unwrap(),items::with_name("Yang").unwrap(),items::with_name("Diamond Hydro Bracelet").unwrap(),items::with_name("Contrast").unwrap(),items::with_name("Spring").unwrap()],106,I12x5::ZERO).unwrap();
    println!("tesbld aphotic 1 (negatives): {:?} {}",tesbld.skills,tesbld.generate_hash());
    tesbld = WynnBuild::make_with_free_spts(&[items::with_name("Aphotic").unwrap(),items::with_name("Starglass").unwrap(),items::with_name("Vaward").unwrap(),items::with_name("Memento").unwrap(),items::with_name("Draoi Fair").unwrap(),items::with_name("Yang").unwrap(),items::with_name("Diamond Hydro Bracelet").unwrap(),items::with_name("Contrast").unwrap(),items::with_name("Spring").unwrap()],106).unwrap();
    println!("tesbld aphotic 2 (negatives): {:?} {}",tesbld.skills,tesbld.generate_hash());

    tesbld = WynnBuild::from_test(&[items::with_name("Dune Storm").unwrap(),items::with_name("Elysium-Engraved Aegis").unwrap(),items::with_name("Barbarian").unwrap(),items::with_name("Revenant").unwrap(),items::with_name("Dispersion").unwrap(),items::with_name("Dispersion").unwrap(),items::with_name("Knucklebones").unwrap(),items::with_name("Incendiary").unwrap(),items::with_name("Oak Wood Spear").unwrap()],106,I12x5::ZERO).unwrap();
    println!("tesbld dune storm 1 (negatives): {:?} {}",tesbld.skills,tesbld.generate_hash());
    tesbld = WynnBuild::make_with_free_spts(&[items::with_name("Dune Storm").unwrap(),items::with_name("Elysium-Engraved Aegis").unwrap(),items::with_name("Barbarian").unwrap(),items::with_name("Revenant").unwrap(),items::with_name("Dispersion").unwrap(),items::with_name("Dispersion").unwrap(),items::with_name("Knucklebones").unwrap(),items::with_name("Incendiary").unwrap(),items::with_name("Oak Wood Spear").unwrap()],106).unwrap();
    println!("tesbld dune storm 2 (negatives): {:?} {}",tesbld.skills,tesbld.generate_hash());

    tesbld = WynnBuild::from_names_test(&"Morph-Stardust,Morph-Steel,Morph-Iron,Morph-Gold,Morph-Emerald,Morph-Topaz,Morph-Amethyst,Morph-Ruby,Oak Wood Spear".split(",").collect::<Vec<&str>>(),106).unwrap();
    println!("tesbld morph 1 (negatives): {:?} {} {}",tesbld.skills,tesbld.calc_melee_dam(),tesbld.generate_hash());
    tesbld = WynnBuild::make_from_names(&"Morph-Stardust,Morph-Steel,Morph-Iron,Morph-Gold,Morph-Emerald,Morph-Topaz,Morph-Amethyst,Morph-Ruby,Oak Wood Spear".split(",").collect::<Vec<&str>>(),106).unwrap();
    println!("tesbld morph 2 (negatives): {:?} {} {}",tesbld.skills,tesbld.calc_melee_dam(),tesbld.generate_hash());

    tesbld = WynnBuild::from_names_test(&"Elf Cap,Elf Robe,Elf Pants,Elf Shoes,Oak Wood Spear".split(",").collect::<Vec<&str>>(),106).unwrap();
    println!("tesbld elph 1: {:?} {} {}",tesbld.skills,tesbld.calc_melee_dam(),tesbld.generate_hash());
    tesbld = WynnBuild::make_from_names(&"Elf Cap,Elf Robe,Elf Pants,Elf Shoes,Oak Wood Spear".split(",").collect::<Vec<&str>>(),106).unwrap();
    println!("tesbld elph 2: {:?} {} {}",tesbld.skills,tesbld.calc_melee_dam(),tesbld.generate_hash());

    tesbld = make_build!(&"Elf Cap,Elf Robe,Elf Pants,Elf Shoes,Oak Wood Spear".split(",").collect::<Vec<&str>>(),106,I12x5::ZERO).unwrap();
    tesbld = make_build!((ELF_CAP,ELF_ROBE,ELF_PANTS,ELF_SHOES,OAK_WOOD_SPEAR),106).unwrap();
    tesbld = make_build!((ELF_CAP,ELF_ROBE,ELF_PANTS,ELF_SHOES,OAK_WOOD_SPEAR),106).unwrap();

    let atreetemp: std::rc::Rc<atree::AtreeBuild> = atree::AtreeBuild::default().into();
    let temp = make_build!({Morph Stardust,Libra,Aleph Null,Stardew,Prism,Summa,Succession,Diamond Fusion Necklace,Nirvana},106,I12x5::ZERO,atreetemp.clone()).unwrap();

    let stuff = atreetemp.iter_stat_bonuses();

    tesbld = WynnBuild::from_names_test(&"Morph-Stardust,Libra,Aleph Null,Stardew,Prism,Summa,Succession,Diamond Fusion Necklace,Nirvana".split(",").collect::<Vec<&str>>(),106).unwrap();
    println!("tesbld nirvana 1: {:?} {} {}",tesbld.skills,tesbld.calc_melee_dam(),tesbld.generate_hash());
    tesbld = WynnBuild::make_from_names(&"Morph-Stardust,Libra,Aleph Null,Stardew,Prism,Summa,Succession,Diamond Fusion Necklace,Nirvana".split(",").collect::<Vec<&str>>(),106).unwrap();
    println!("tesbld nirvana 2: {:?} {} {}",tesbld.skills,tesbld.calc_spell_dam(),tesbld.generate_hash());

    if true{
        return;
    }
    // let stats = [-12_i32 as u64, 16 as u64, 21 as u64, 34 as u64, 17 as u64];
    // let stat_format: u64 = stats[0] << 48 + stats[1] << 36 + stats[2] << 24 + stats[3] << 12 + stats[4];
    // println!("{}", 1_i32 << 5_usize);

    // println!("{}, {} YES:{} {:b} {:b} {:b} {:b} {}",idval, idval>>24,(((idval & 0x800000) * 0x1FF)|(idval&0x7FFFFF)) as i32,value, -999999, idval, -1, ((idval & 0x800000) * 0x1FF | idval & 0x7FFFFF) as i32);

    // for i in &items_with_type[Type::Bow.to_num()]{
    //     if items::get(*i).props[1]==Atrs::Tier(Tier::Mythic){
    //         println!("{}",items::get(*i).name);
    //     }
    // }
    // println!("{}",items::Atrs::Agi(3).varient_eq(&items::Atrs::Str(10)));
    // println!("{}",items::with_name("Warp").unwrap_or((0,&items::get(0))).1.name);

    // let sets = items::with_prop_value(Atrs::Tier(Tier::Set));

    // for i in sets{
    //     println!("{}",items::get(i).props.last().unwrap());
    // }

    // println!("{}",items::WynnBuild::can_build_real(&[2988,3632,2793,983,1678,2864,2641,3110,1302],106));
                    // for i in 0..8_usize{
                    //     for j in items_with_type[i].iter(){
                    //         let item = items::get(*j);
                    //         // let hp =
                    //         // let temp = item.get_ids(&[Atrs::HpBonus(0)]);
                    //     }
                    // }
    let mut items_with_type: [Vec<WynnItem>; 13] = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];
    for i in 0..12_usize {
        let mut thing = items::with_prop_value(Atrs::Type, i as u32);
        thing.retain(|itm| itm.get_ident(Atrs::Lvl).unwrap()>80);
        items_with_type[i] = thing;
        // println!("before {}, {}, {}",items_with_type[i][0].name(),items_with_type[i][0].get_ident(Atrs::Hp).unwrap_or(0),items_with_type[i][0].get_ident(Atrs::HpBonus).unwrap_or(0));
        items_with_type[i].sort_by(|a,b| (a.get_ident(Atrs::Hp).unwrap_or(0)+a.get_ident(Atrs::HpBonus).unwrap_or(0)).cmp(&(b.get_ident(Atrs::Hp).unwrap_or(0)+b.get_ident(Atrs::HpBonus).unwrap_or(0))));
        items_with_type[i].reverse();


        println!("{}",items_with_type[i].iter().any(|itm| itm.name()=="Revenant"));
        // println!("after {}, {}, {}",items_with_type[i][0].name(),items_with_type[i][0]get_ident(Atrs::Hp).unwrap_or(0),items_with_type[i][0].get_ident(Atrs::HpBonus).unwrap_or(0));
    }
    items_with_type[6][19]=items::with_name("Derecho").unwrap();
    let num_iter: usize = 20;
    let mut bld = WynnBuild::make_with_free_spts(&[items_with_type[0][0].clone(),items_with_type[4][5].clone()],106).unwrap();
    bld = WynnBuild::make_with_free_spts(&[WynnItem::from_idx(2988),WynnItem::from_idx(3632),WynnItem::from_idx(2793),WynnItem::from_idx(983),WynnItem::from_idx(1678),WynnItem::from_idx(2864),WynnItem::from_idx(2641),WynnItem::from_idx(3110),WynnItem::from_idx(1302)],201).unwrap();
    bld = WynnBuild::make_with_free_spts(&[items::with_name("Aphotic").unwrap(),items::with_name("Starglass").unwrap(),items::with_name("Vaward").unwrap(),items::with_name("Memento").unwrap(),items::with_name("Draoi Fair").unwrap(),items::with_name("Yang").unwrap(),items::with_name("Diamond Hydro Bracelet").unwrap(),items::with_name("Contrast").unwrap(),items::with_name("Spring").unwrap()],106).unwrap();
    bld = WynnBuild::make_from_names(&"Morph-Stardust,Libra,Aleph Null,Stardew,Prism,Summa,Succession,Diamond Fusion Necklace,Nirvana".split(",").collect::<Vec<&str>>(),106).unwrap();
    let bld_mdam = bld.calc_melee_dam();
    let bld_sdam = bld.calc_spell_dam();
    // println!("melee dams: n({},{}) e({},{}) t({},{}) w({},{}) f({},{}) a({},{})",bld_dams[0].0,bld_dams[0].1,bld_dams[1].0,bld_dams[1].1,bld_dams[2].0,bld_dams[2].1,bld_dams[3].0,bld_dams[3].1,bld_dams[4].0,bld_dams[4].1,bld_dams[5].0,bld_dams[5].1);
    println!("average melee damage: {}",bld_mdam);
    println!("average spell damage: {}",bld_sdam);


    println!("{}",bld.generate_hash());
    bld = WynnBuild::make_with_free_spts(&[items::with_name("Cancer֎").unwrap(),items::with_name("Leo").unwrap(),items::with_name("Atomizer").unwrap(),items::with_name("Statue").unwrap(),items::with_name("Diamond Solar Ring").unwrap(),items::with_name("Diamond Solar Ring").unwrap(),items::with_name("Diamond Steam Bracelet").unwrap(),items::with_name("Diamond Solar Necklace").unwrap(),items::with_name("Guardian").unwrap()],106).unwrap();
    // bld = WynnBuild::make_with_free_spts(&[items::with_name("Heroism").unwrap(),items::with_name("Leo").unwrap(),items::with_name("Second Wind").unwrap(),items::with_name("Statue").unwrap(),items::with_name("Archaic").unwrap(),items::with_name("Archaic").unwrap(),items::with_name("Diamond Solar Bracelet").unwrap(),items::with_name("Derecho").unwrap(),items::with_name("Guardian").unwrap()],106).unwrap();
    println!("my ehp {}",bld.calc_ehp());
    println!("my ehp maxed {}",bld.calc_max_ehp());

    let guardian = items::with_name("Guardian").unwrap();

    let mut best_skills: Vec<[i32;5]> = Vec::new();
    for i in items_with_type.iter().rev().skip(1){
        if i[0].get_type() as u8 <=6{
            let mut best = [0,0,0,0,0];
            for itm in i{
                for skill in itm.iter_skill_bonus(){
                    if skill.1>best[skill.0 as usize-18]{best[skill.0 as usize-18]=skill.1};
                }
            }
            match best_skills.last(){
                Some(n) => for j in 0_usize..5{best[j]+=n[j]},
                None => ()
            };
            best_skills.push(best);
        }
    }
    best_skills.reverse();
    println!("{} {} {} {} {}",best_skills[3][0],best_skills[3][1],best_skills[3][2],best_skills[3][3],best_skills[3][4]);
    let do_loop = false;
    if do_loop{
        for helm in items_with_type[0].iter().take(num_iter) {
            println!("helmet: {}, max ehp: {}",helm.name(),bld.calc_max_ehp());
            for chest in items_with_type[1].iter().take(num_iter) {
                for legs in items_with_type[2].iter().take(num_iter) {
                    if WynnBuild::make_with_free_spts(&[*helm, *chest, *legs,guardian],370).is_none(){continue;}
                    for boots in items_with_type[3].iter().take(num_iter) {
                        let test_build = WynnBuild::make_with_base_skills(&[*helm,*chest,*legs,*boots,guardian], 200,best_skills[3].into());
                        match test_build{
                            Some(b) => if b.calc_max_ehp()>(bld.calc_max_ehp()).max(0.0){
                                for r1 in items_with_type[4].iter().take(num_iter) {
                                    for r2 in items_with_type[4].iter().take(num_iter) {
                                        if WynnBuild::make_with_free_spts(&[*helm, *chest, *legs, *boots, *r1,guardian],260).is_none()
                                        {
                                            continue;
                                        }
                                        for brace in items_with_type[5].iter().take(num_iter) {
                                            for neck in items_with_type[6].iter().take(num_iter)
                                            {
                                                let e = [*helm, *chest, *legs, *boots, *r1, *r2, *brace, *neck,guardian];
                                                let test_build = WynnBuild::make_with_free_spts(&e, 200);
                                                match test_build {
                                                    Some(c) => {
                                                        if c.calc_max_ehp()>bld.calc_max_ehp()
                                                        {
                                                            bld = c;
                                                        }
                                                    }
                                                    None => (),
                                                }
                                            }
                                        }
                                    }
                                }
                            }, None => ()
                        }
                    }
                }
            }
        }
    }
    println!("{}",bld.calc_max_ehp());
    println!("{}",bld.generate_hash());


    let mut tesbld = WynnBuild::make_from_names(&"Dune Storm,Dondasch,Horizon,Revenant,Dispersion,Dispersion,Knucklebones,Achromatic Gloom,Oak Wood Spear".split(",").collect::<Vec<&str>>(),106).unwrap();
    println!("test? {} {:?}",tesbld.calc_melee_dam(),tesbld.skills);
    println!("testing sets {}",items::with_name("Contrast").unwrap().get_set());
    println!("starting...");
    let mut items_with_type: [Vec<WynnItem>; 8] = [Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new()];
    let weapon = items::with_name("Oak Wood Spear").unwrap();
    for i in 0..8_usize {
        items_with_type[i] = items::with_prop_value(Atrs::Type, if i>4{i - 1}else{i} as u32);
        items_with_type[i].sort_by(|itm1, itm2| BestBuildCalc::calc_ord(&WynnBuild::make_with_base_skills(&[*itm1, weapon], 0, I12x5::fill_data(150)).unwrap()).cmp(
            &BestBuildCalc::calc_ord(&WynnBuild::make_with_base_skills(&[*itm2, weapon], 0, I12x5::fill_data(150)).unwrap())));
        items_with_type[i].reverse();
        
        println!("there are {} items of type {}",items_with_type[i].len(),Type::try_from(if i>4{i - 1}else{i}).unwrap());

        items_with_type[i].resize(20, WynnItem::NULL);
        // println!("horizon {}",items_with_type[i].iter().any(|itm| itm.name()=="Horizon"));
        // println!("dune storm {}",items_with_type[i].iter().any(|itm| itm.name()=="Dune Storm"));
        // println!("knucklebones {}",items_with_type[i].iter().any(|itm| itm.name()=="Knucklebones"));
        
        for idx in 0..items_with_type[i].len(){
            if items_with_type[i][idx].name()=="Dune Storm"{
                println!("Dune Storm at {}",idx);
            }
            if items_with_type[i][idx].name()=="Luminiferous Aether"{
                println!("lumi at {}",idx);
            }
            if items_with_type[i][idx].is_null(){
                println!("null at {}, removing...",idx);
                items_with_type[i].remove(idx);
                println!("new len is {}",items_with_type[i].len());
            }
        }
    }
    items_with_type[3].push(items::with_name("Horizon").unwrap());
    items_with_type[6].push(items::with_name("Knucklebones").unwrap());
    let mut calc_future = BestBuildCalc::make(items::with_name("Oak Wood Spear").unwrap(),items_with_type);

    while !calc_future.calc_best_build(1000000){
        println!("best so far: {}, hashval: {} \nnames {}",calc_future.curr_bests[0].0,calc_future.curr_bests[0].1.generate_hash(),calc_future.curr_bests[0].1.item_names());
    }
    println!("{}",calc_future.curr_bests.len());
    println!("ehp {}",calc_future.curr_bests[0].1.calc_ehp());
    println!("dam {}",calc_future.curr_bests[0].0);
    println!("dam2 {}",calc_future.curr_bests[0].1.calc_melee_dam());
    println!("a {}",calc_future.curr_bests[0].1.overall_atk_spd());
    println!("{}",calc_future.curr_bests[0].1.item_names());
    println!("{:?}",calc_future.curr_bests[0].1.skills);
    println!("{}",calc_future.curr_bests[0].1.generate_hash());

    let mut testitm = items::with_name("Dispersion").unwrap();
    testitm.set_quality(0.7);
    println!("{}",testitm.get_ident(Atrs::MdRaw).unwrap());
    println!("85 * 1.3 = {}",85.0_f32*1.3);
    let mut tesbld = WynnBuild::make_from_names(&"Dune Storm,Dondasch,Horizon,Revenant,Dispersion,Dispersion,Knucklebones,Achromatic Gloom,Oak Wood Spear".split(",").collect::<Vec<&str>>(),106).unwrap();
    println!("test? {} {:?}",tesbld.calc_melee_dam(),tesbld.skills);

    // println!("again? {}",WynnBuild::make_from_names(&"Venison,Admiral,Aleph Null,Cursed Jackboots,Breezehands,Breezehands,Misalignment,Diamond Fusion Necklace,Nirvana".split(",").collect::<Vec<&str>>(),106).unwrap().calc_spell_dam());


    // let mut test = vec![(5,3),(3,8),(4,9)];
    // test.sort();
    // println!("{:?}",test);
}



enum GetItemMsgs {
    Content(String),
    Search,
    Calc,
    CalcBest,
    CalcDone,//(Vec<WynnBuild>),
    None,
}
struct RootComponent {
    input_content: Vec<String>,
    item_display: String,
    items: [WynnItem; 9],
    url_hash: String,
    extra: String,
    calc_future: BestBuildCalc,
    res_builds: Vec<WynnBuild>,
    handle: Option<Timeout>
    // selectors: ItemSelector
}
impl Component for RootComponent {
    type Message = GetItemMsgs;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            input_content: Vec::new(),
            item_display: String::new(),
            items: [WynnItem::NULL; 9],
            url_hash: String::new(),
            extra: String::new(),
            calc_future: BestBuildCalc::default(),
            res_builds: Vec::new(),
            handle: None
            // selectors: ItemSelector::create(Context::from(value))
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GetItemMsgs::Content(s) => {
                self.input_content = s
                    .split(",")
                    .map(|v| v.trim().to_string())
                    .collect::<Vec<String>>();
                true
            }
            GetItemMsgs::Calc => {
                for i in 0..9_usize.min(self.input_content.len()) {
                    self.items[i] = match items::with_name(&self.input_content[i]) {
                        Some(r) => r,
                        None => WynnItem::NULL,
                    }
                }

                let mut items_with_type: [Vec<WynnItem>; 13] = [
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ];

                for i in 0..12_usize {
                    items_with_type[i] = items::with_prop_value(Atrs::Type, i as u32);
                }
                let mut build = WynnBuild::make_with_free_spts(&self.items, 400);
                if false {
                    let mut bld = build.unwrap();
                    let num_iter: usize = 20;
                    bld = WynnBuild::make_with_free_spts(&[items_with_type[0][0].clone(),items_with_type[4][5].clone()],106).unwrap();
                    let guardian = items::with_name("Guardian").unwrap();
                    let mut best_skills: Vec<[i32;5]> = Vec::new();
                    for i in items_with_type.iter().rev().skip(1){
                        if i[0].get_type() as u8 <=6{
                            let mut best = [0,0,0,0,0];
                            for itm in i{
                                for skill in itm.iter_skill_bonus(){
                                    if skill.1>best[skill.0 as usize-18]{best[skill.0 as usize-18]=skill.1};
                                }
                            }
                            match best_skills.last(){
                                Some(n) => for j in 0_usize..5{best[j]+=n[j]},
                                None => ()
                            };
                            best_skills.push(best);
                        }
                    }
                    for helm in items_with_type[0].iter().take(num_iter) {
                        println!("helmet: {}, max ehp: {}",helm.name(),bld.calc_max_ehp());
                        for chest in items_with_type[1].iter().take(num_iter) {
                            for legs in items_with_type[2].iter().take(num_iter) {
                                if WynnBuild::make_with_free_spts(&[*helm, *chest, *legs,guardian],370).is_none(){continue;}
                                for boots in items_with_type[3].iter().take(num_iter) {
                                    let test_build = WynnBuild::make_with_base_skills(&[*helm,*chest,*legs,*boots,guardian], 200,best_skills[3].into());
                                    match test_build{
                                        Some(b) => if b.calc_max_ehp()>(bld.calc_max_ehp()).max(0.0){
                                            for r1 in items_with_type[4].iter().take(num_iter) {
                                                for r2 in items_with_type[4].iter().take(num_iter) {
                                                    if WynnBuild::make_with_free_spts(&[*helm, *chest, *legs, *boots, *r1,guardian],260).is_none()
                                                    {
                                                        continue;
                                                    }
                                                    for brace in items_with_type[5].iter().take(num_iter) {
                                                        for neck in items_with_type[6].iter().take(num_iter)
                                                        {
                                                            let e = [*helm, *chest, *legs, *boots, *r1, *r2, *brace, *neck,guardian];
                                                            let test_build = WynnBuild::make_with_free_spts(&e, 200);
                                                            match test_build {
                                                                Some(c) => {
                                                                    if c.calc_max_ehp()>bld.calc_max_ehp()
                                                                    {
                                                                        bld = c;
                                                                    }
                                                                }
                                                                None => (),
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }, None => ()
                                    }
                                }
                            }
                        }
                    }
                    build=Some(bld);
                }
                self.item_display =
                    if WynnBuild::make_with_free_spts(&self.items, 201).is_some() {String::from("Works!")} 
                    else {String::from("Not enough skill points")};
                if build.is_some() {
                    let b = build.unwrap();
                    self.url_hash = b.generate_hash();
                    // self.url_hash += &items::url_hash_val(106, 2);
                    // self.url_hash += "000000"; //TODO: powders
                    // self.url_hash += "z0z0+0+0+0+0-"; // TODO: tomes
                }

                // self.url_hash += items::get(items::with_prop_value(Atrs::Id(1641))[0]).name;

                // self.item_display = match items::with_name(&self.input_content){
                //     Some(r) => format!("#{}: {}, {}",r.0,r.1.name,r.1.props.iter().map(|v| v.to_string()+", ").collect::<String>()),
                //     None => String::from("Could not find item")
                // }; Cumulonimbus, Libra, Vaward, Pro Tempore, Warp
                // Cumulonimbus, Libra, Vaward, Pro Tempore, Warp, Diamond Steam Necklace
                // Cumulonimbus, Starglass, Vaward, Stardew, Moon Pool Circlet, Diamond Hydro Ring, Diamond Hydro Bracelet, Contrast, Spring
                // Aphotic, Starglass, Vaward, Memento, Draoi Fair, Yang, Diamond Hydro Bracelet, Contrast, Spring
                // Cumulonimbus, Libra, The Ephemeral, Stardew, Yang, Intensity, Succession, Diamond Fusion Necklace, Nirvana --- REAL TEST
                // id for last: [2988,3632,2793,983,1678,2864,2641,3110,1302]
                true
            }
            GetItemMsgs::Search => {
                // _ctx.link().callback(|b| GetItemMsgs::Calc).emit(false);
                let mut items_with_type: [Vec<WynnItem>; 8] = [Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new()];

                for i in 0..8_usize {
                    items_with_type[i] = items::with_prop_value(Atrs::Type, if i>4{i - 1}else{i} as u32);
                    items_with_type[i].resize(20, WynnItem::NULL);
                }
                self.calc_future = BestBuildCalc::make(items::with_name("Nirvana").unwrap(),items_with_type);
                // _ctx.link().callback_future(async {GetItemMsgs::CalcBest});
                // _ctx.link().send_future(async {GetItemMsgs::CalcBest});
                // _ctx.link().callback_future_once(function)
                let link = _ctx.link().clone();
                console::log_1(&format!("starting").into());
                // self.handle = Some(Timeout::new(10, move || link.send_message(GetItemMsgs::CalcDone)));
                for i in (0..20).skip(19){
                    console::log_1(&format!("testing {} ",i).into());
                }
                console::log_1(&format!("actually starting").into());
                // let link = _ctx.link().clone();
                self.handle = Some(Timeout::new(0, move || link.send_message(GetItemMsgs::CalcBest)));
                
                // _ctx.link().send_future(async{self.calc_future.calc_best_build(1000).await; GetItemMsgs::CalcDone});
                //_ctx.link().send_future(BestBuildCalc::make(items::with_name("Guardian").unwrap(),items_with_type)).emit(false);
                // self.calc_future = BestBuildCalc::make(items::with_name("Guardian").unwrap(),items_with_type);
                // _ctx.link().send_future(BestBuildCalc::make(items::with_name("Oak Wood Spear").unwrap(),items_with_type));
                
                true
            }
            GetItemMsgs::CalcBest => {
                if self.calc_future.calc_best_build(u32::MAX){
                    _ctx.link().send_message(GetItemMsgs::CalcDone);
                }else{
                    let link = _ctx.link().clone();
                    self.handle = Some(Timeout::new(0, move || link.send_message(GetItemMsgs::CalcBest)));
                }
                false
            }
            GetItemMsgs::CalcDone => {
                console::log_1(&format!("done?").into());

                self.url_hash = String::new();
                for (a,b) in self.calc_future.curr_bests.iter(){
                    // self.url_hash=items::get(b.item_idxs[0]).get_url_hash();
                    // self.url_hash=b.item_idxs[0].to_string();
                    self.url_hash += &b.generate_hash();
                    // self.url_hash += &items::url_hash_val(106, 2);
                    // self.url_hash += "000000"; // TODO: powders
                    // self.url_hash += "z0z0+0+0+0+0-\n"; // TODO: tomes
                }
                true
            }
            GetItemMsgs::None => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="content">
                <h1>{"HELO WORLD"}</h1>
                <input oninput={link.callback(|event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); GetItemMsgs::Content(input.value())})} onkeypress={link.callback(|key:KeyboardEvent| {if key.char_code()==13 {GetItemMsgs::Search} else{GetItemMsgs::None}})}/>
                <p>{self.item_display.clone()}</p>
                {
                    self.items.iter().map(|item| html!{<p>{format!("{} - {}",item.name(),item.iter_data().map(|v| format!("{}: {}",v.0.to_string(),&v.1.to_string())+", ").collect::<String>())}</p>}).collect::<Html>()
                }
                <p>{self.url_hash.clone()}</p>
                <button>{"Tester"}</button>
                <a href={format!("https://wynnbuilder.github.io/builder/#8_{}",self.url_hash.clone())} target={"_blank"}>{"Wynnbuilder Link"}</a>
            </div>
        }
    }
}

enum ItemSelectorMsg{
    AddItem(String),
    None
}
#[derive(PartialEq, Clone, Properties)]
struct ItemSelectorProps{
    input_content: Vec<String>
}
struct ItemSelector {
    // input_content: Vec<WynnItem>,
}
impl Component for ItemSelector {
    type Message = ItemSelectorMsg;

    type Properties = ItemSelectorProps;
    
    fn create(ctx: &Context<Self>) -> Self {
        ItemSelector{}
    }
    fn update(&mut self,  ctx: &Context<Self>, msg: Self::Message) -> bool{
        match msg{
            ItemSelectorMsg::AddItem(s) => {
                true
            }
            ItemSelectorMsg::None => false
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html!{
            <h1>{format!("TEST STUFF {}",ctx.props().input_content.len())}</h1>
        }
    }
}

struct BestBuildCalc{
    items: [Vec<WynnItem>; 8],
    weapon: WynnItem,
    curr: u64,
    counter_mults: [u64; 9],
    curr_bests: Vec<(i32,WynnBuild)>,
    best_skills: Vec<I12x5>,
    best_ids: Vec<[i32; Atrs::NUM_STATS]>,
    min_stat_reqs: Vec<(Atrs,i32)>,
    ehp_req: f32
}
impl BestBuildCalc{
    fn make(weapon: WynnItem, items: [Vec<WynnItem>; 8]) -> Self{
        let mut best_skills: Vec<I12x5> = Vec::new();
        best_skills.push(I12x5::ZERO);
        let mut best_ids: Vec<[i32; Atrs::NUM_STATS]> = Vec::new();
        best_ids.push([0; Atrs::NUM_STATS]);
        let mut items_optimized_order: [Vec<WynnItem>; 8] = Default::default();
        let mut counter_mults: [u64; 9] = [0; 9];
        counter_mults[0]=1;
        for (n, i) in items.iter().rev().enumerate(){
            counter_mults[n+1]=i.len() as u64 * counter_mults[n];
            let mut best = I12x5::ZERO;
            let mut best_id = [0; Atrs::NUM_STATS];
            for itm in i{
                best = itm.get_skill_bonuses().max(best);
                for id in itm.iter_ids(){
                    if id.1 > best_id[id.0 as usize - Atrs::NUM_NON_STATS] {best_id[id.0 as usize - Atrs::NUM_NON_STATS] = id.1}
                }
            }
            best_skills.push(best+*best_skills.last().unwrap_or(&I12x5::ZERO));
            match best_ids.last(){
                Some(n) => for j in 0_usize..Atrs::NUM_STATS{best_id[j]+=n[j]},
                None => ()
            };
            best_ids.push(best_id);
            let mut temp = i.clone();
            temp.sort_by(|itm1, itm2| Self::calc_ord(&WynnBuild::make_with_base_skills(&[*itm1, weapon], 0, wynn_data::I12x5::fill_data(150)).unwrap()).cmp(
                &Self::calc_ord(&WynnBuild::make_with_base_skills(&[*itm2, weapon], 0, wynn_data::I12x5::fill_data(150)).unwrap())));
            temp.reverse();
            // temp.sort_by(|itm1, itm2| Self::calc_ord(&WynnBuild::make_with_base_skills(&[&itm2, &weapon], 0, &[150,150,150,150,150]).unwrap()).cmp(
            //     &Self::calc_ord(&WynnBuild::make_with_base_skills(&[&itm1, &weapon], 0, &[150,150,150,150,150]).unwrap())));
            items_optimized_order[n]=temp;
        }
        best_skills.reverse();
        best_ids.reverse();
        counter_mults.reverse();
        items_optimized_order.reverse();
        // console::log_1(&format!("skills[8] {:?} ids[8] {:?}",best_skills[8], best_ids[8]).into());
        let mut temp = Self{items: items_optimized_order, weapon, curr: 1, counter_mults, curr_bests: Vec::new(), best_skills, best_ids, min_stat_reqs: Vec::new(), ehp_req: 0.0};
        match temp.make_build_if_reqs_met(&[temp.weapon,temp.items[0][0],temp.items[1][0],temp.items[2][0],temp.items[3][0],temp.items[4][0],temp.items[5][0],temp.items[6][0],temp.items[7][0]]){
            Some(b) => temp.curr_bests.push((Self::calc_ord(&b), b)),
            None => ()
        }
        for i in 0..10{temp.curr_bests.push((22000, WynnBuild::make_from_names(&["Dune Storm", "Dondasch", "Dizzy Spell", "Revenant", "Dispersion", "Dispersion", "Knucklebones", "Diamond Static Necklace", "Oak Wood Spear"], 200).unwrap()))};
        // console::log_1(&format!("start test {}",start).into());
        temp
    }
    fn calc_ord(bld: &WynnBuild) -> i32 {
        // bld.calc_spell_dam() as i32
        bld.calc_melee_dam() as i32
    }
    fn make_build_if_reqs_met(&self, bld: &[WynnItem]) -> Option<WynnBuild>{
        match WynnBuild::make_with_skills_and_stats(bld, 200, self.best_skills[bld.len()-1], &self.best_ids[bld.len()-1]){
            Some(b) => if b.calc_max_ehp()>=self.ehp_req && self.min_stat_reqs.iter().all(|(id,val)| b.get_stat(*id)>=*val) && (self.curr_bests.is_empty() || Self::calc_ord(&b)>self.curr_bests.last().unwrap().0){Some(b)} else {None},
            None => {None}
        }
    }
    fn get_curr_item_idx(&self, idx: usize) -> usize{
        (self.curr/self.counter_mults[idx+1]) as usize%self.items[idx].len()
    }
    fn calc_best_build(&mut self, stop: u32) -> bool{
        // console::log_1(&format!("calcing... {}",(self.curr[0]*self.items[1].len()+self.curr[1]) as f32 / (self.items[0].len()*self.items[1].len()) as f32).into());
        let mut counter = 0;
        // console::log_1(&format!("pct {}",self.curr as f32/self.counter_mults[0] as f32).into());
        println!("pct {}",self.curr as f32/self.counter_mults[0] as f32);
        let mut bld_items = [self.weapon, self.items[0][self.get_curr_item_idx(0)], self.items[1][self.get_curr_item_idx(1)], self.items[2][self.get_curr_item_idx(2)], self.items[3][self.get_curr_item_idx(3)], self.items[4][self.get_curr_item_idx(4)], self.items[5][self.get_curr_item_idx(5)], self.items[6][self.get_curr_item_idx(6)], self.items[7][self.get_curr_item_idx(7)]];
        while self.curr<self.counter_mults[0]{
            let mut start = 0;
            for i in (0..8).rev(){
                if self.curr/self.counter_mults[i]==(self.curr-1)/self.counter_mults[i]{start=i; break;}
            }
            for i in start..8{
                let curr_item_idx = self.get_curr_item_idx(i);
                bld_items[i+1]=self.items[i][curr_item_idx];
                if i>1 { // i > 1 or i==2 || i==3 || i==5 || i==7
                    match self.make_build_if_reqs_met(&bld_items[0..(i+2)]){
                        Some(b) => {
                            if i==7{ 
                                let ord_val = Self::calc_ord(&b);
                                if self.curr_bests.len()<10{
                                    let idx = match self.curr_bests.binary_search_by(|b| ord_val.cmp(&b.0)){Ok(v) => v, Err(v) => v};
                                    println!("inserting at {}",idx);
                                    self.curr_bests.insert(idx,(ord_val,b));
                                }
                                else{ //  if ord_val > match self.curr_bests.last(){Some(n) => n.0, None => i32::MIN}
                                    self.curr_bests.pop();
                                    let idx = match self.curr_bests.binary_search_by(|b| ord_val.cmp(&b.0)){Ok(v) => v, Err(v) => v};
                                    self.curr_bests.insert(idx,(ord_val,b));
                                }
                                self.curr+=1;
                            }
                        },
                        None => {self.curr+=self.counter_mults[i+1]; break;}
                    }
                }
            }
            counter+=1;
            if counter>=stop{
                return false
            }
        }
        // console::log_1(&format!("skipping {} ",self.curr/self.counter_mults[0]).into());
        // for (h, helm) in self.items[0].iter().enumerate().skip(self.get_curr_item_idx(0)) {
        //     for (c, chest) in self.items[1].iter().enumerate().skip(self.get_curr_item_idx(1)) {
        //         for (l, legs) in self.items[2].iter().enumerate().skip(self.get_curr_item_idx(2)) {
        //             // if self.make_build_if_reqs_met(&[helm,chest,legs,&self.weapon]).is_none(){continue}
        //             if WynnBuild::make_with_free_spts(&[helm, chest, legs,&self.weapon],370).is_none(){continue;}
        //             for (b, boots) in self.items[3].iter().enumerate().skip(self.get_curr_item_idx(3)) {
        //                 // let test_build = WynnBuild::make_with_base_skills(&[helm,chest,legs,boots,&self.weapon], 200,&best_skills[3]);

        //                 match self.make_build_if_reqs_met(&[helm,chest,legs,boots,&self.weapon]){
        //                     Some(_) => {
        //                         for (r1, ring1) in self.items[4].iter().enumerate().skip(self.get_curr_item_idx(4)) {
        //                             for (r2, ring2) in self.items[5].iter().enumerate().skip(self.get_curr_item_idx(5)) {
        //                                 if WynnBuild::make_with_free_spts(&[helm, chest, legs, boots, ring1,&self.weapon],260).is_none()
        //                                 {
        //                                     continue;
        //                                 }
        //                                 for (br, brace) in self.items[6].iter().enumerate().skip(self.get_curr_item_idx(6)) {
        //                                     for (n, neck) in self.items[7].iter().enumerate().skip(self.get_curr_item_idx(7))
        //                                     {
        //                                         let e = [helm, chest, legs, boots, ring1, ring2, brace, neck,&self.weapon];
        //                                         let test_build = WynnBuild::from_refs(&e, 200);
        //                                         match test_build {
        //                                             Some(c) => {
        //                                                 let ord_val = Self::calc_ord(&c);
        //                                                 if self.curr_bests.len()<10{
        //                                                     let idx = match self.curr_bests.binary_search_by(|b| b.0.cmp(&ord_val)){Ok(v) => v, Err(v) => v};
        //                                                     self.curr_bests.insert(idx,(ord_val,c));
        //                                                 }
        //                                                 else if ord_val > match self.curr_bests.last(){Some(n) => n.0, None => i32::MIN}{
        //                                                     self.curr_bests.pop();
        //                                                     let idx = match self.curr_bests.binary_search_by(|b| b.0.cmp(&ord_val)){Ok(v) => v, Err(v) => v};
        //                                                     self.curr_bests.insert(idx,(ord_val,c));
        //                                                 }
        //                                             }
        //                                             None => (),
        //                                         }
        //                                         counter+=1;
        //                                         // if counter>=stop{
        //                                         //     self.curr=[h,c,l,b,r1,r2,br,n];
        //                                         //     return false
        //                                         // }
        //                                     }
        //                                 }
        //                             }
        //                         }
        //                     }, None => ()
        //                 }
        //             }
        //         }
        //     }
        //     self.curr=[h+1,0,0,0,0,0,0,0];
        //     return false
        // }
        // for i in 0..self.curr.len(){
        //     self.curr[i]=self.items[i].len();
        // }
        true
    }
}
impl Default for BestBuildCalc{
    fn default() -> Self {
        Self { items: Default::default(), weapon: WynnItem::NULL, curr: Default::default(), counter_mults: Default::default(), curr_bests: Default::default(), best_skills: Default::default(), best_ids: Default::default(), min_stat_reqs: Default::default(), ehp_req: Default::default() }
    }
}
// impl Future for BestBuildCalc{
//     type Output = GetItemMsgs;

//     fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
//         console::log_1(&format!("polled... {}",(self.curr[0]*self.items[1].len()+self.curr[1]) as f32 / (self.items[0].len()*self.items[1].len()) as f32).into());
//         if self.curr[7]>=self.items[7].len(){
//             // Poll::Ready(self.curr_bests.iter().map(|(i,v)| v.clone()).collect::<Vec<WynnBuild>>())
//             Poll::Ready(GetItemMsgs::CalcDone(self.curr_bests.iter().map(|(i,v)| v.clone()).collect::<Vec<WynnBuild>>()))
//         }else{
//             self.calc_best_build(1000);
//             // cx.waker().clone().wake();
//             Poll::Pending
//         }
//     }
// }
#[derive(PartialEq, Clone)]
#[repr(usize)]
enum Test {
    A,
    B,
    C,
    D,
}

struct Test2<'a> {
    a: &'a str,
    b: &'a [i32],
    c: Test,
}

// pub fn can_build(idxs: &[usize], extra_sps: i32) -> bool{
//     let mut extra_skill_pts = extra_sps;
//     let mut skill_pts = [0,0,0,0,0];
//     let mut max_reqs = [0,0,0,0,0];
//     let mut assigned_skills = [0,0,0,0,0];
//     let mut max_boost = [0,0,0,0,0];
//     for i in idxs.iter(){
//         if *i==0_usize{continue}
//         let item = items_list::ALL_ITEMS[*i];
//         let req = item.get_reqs();
//         let boosts = item.get_skill_bonuses();
//         for (i,r) in req.iter().enumerate(){
//             if max_reqs[i]<*r {max_reqs[i]=*r; max_boost[i]=boosts[i]}
//         }
//         for (i, b) in boosts.iter().enumerate(){skill_pts[i]+=b;}
//     }for i in 0..5_usize{
//         let diff = skill_pts[i]-max_boost[i]-max_reqs[i];if max_reqs[i]<=0 {continue}
//         if diff<0{extra_skill_pts+=diff;skill_pts[i]-=diff+max_boost[i];assigned_skills[i]=-diff;}
//         if extra_skill_pts<0 || -99>diff{return false}}true
//     }

// pub fn can_build_real(idxs:&[usize],extra_sps: i32) -> bool{
//     let mut v: [BinaryHeap<(i32,usize,i32)>;5] = [BinaryHeap::with_capacity(9),BinaryHeap::with_capacity(9),BinaryHeap::with_capacity(9),BinaryHeap::with_capacity(9),BinaryHeap::with_capacity(9)];
//     let mut extra_skill_pts = extra_sps;
//     let mut skills = [0,0,0,0,0];
//     for item_id in idxs.iter(){
//         let item = items_list::ALL_ITEMS[*item_id];
//         let req = item.get_reqs();
//         let boosts = item.get_skill_bonuses();
//         for (i,r) in req.iter().enumerate(){
//             if *r>0{v[i].push((-r,*item_id,boosts[i]))}else{skills[i]+=boosts[i]}
//         }
//     }
//     for skill_type in 0..5_usize{
//         for item in 0..v[skill_type].len(){
//             let temp = v[skill_type].pop().unwrap();
//             let diff = skills[skill_type]+temp.0;
//             if diff<0{extra_skill_pts+=diff;skills[skill_type]-=diff;}
//             if extra_skill_pts<0 || -99>diff{return false}
//             skills[skill_type]+=temp.2;
//         }
//     }
//     true
// }
