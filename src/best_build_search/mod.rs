pub mod helper_enums;

use std::rc::Rc;

use crate::wynn_data::{builder::WynnBuild, items::*, I12x5, atree::AtreeBuild};
use helper_enums::SearchReq;
use crate::make_build;

/// Structure used for calculating best builds
pub struct BestBuildSearch{
    items: [Vec<WynnItem>; 8],
    weapon: WynnItem,
    curr: u64,
    counter_mults: [u64; 9],
    curr_bests: Vec<(i32,WynnBuild)>,
    best_skills: Vec<I12x5>,
    best_ids: Vec<[i32; Atrs::NUM_STATS]>,
    min_stat_reqs: Vec<SearchReq>,
    max_stat_reqs: Vec<SearchReq>,
    atree: Rc<AtreeBuild>,
    calc_ord: fn(&WynnBuild) -> i32
}
impl BestBuildSearch{
    // At this algorithmn's center is a simple brute force 'try every combination of the given items' approach 
    // Various optimizations are used to try to speed up this process (there are ~10^19 combinations of items, as of me writing this)
    // These optimizations attempt to make this algorithmn resemble alpha-beta search/pruning, though not perfect. 
    // This description is probably too confusing to understand, so just look at the code or smthn idk

    // 1) Prune builds that require too many skill points
    // Save the best skill point bonuses for every item type among the given items 
    // If a given combination of part of a build (ie, just the armor) becomes impossible to complete even with the best skill points, 
    // prune that combination and move onto the next item
    // *insert example*

    // 2) Prune builds significantly worse than the best found build
    // Keep track of the best identification value for every damage-related identification among every item type
    // If part of a build + the best identifications for the remaining item types is worse than another found build, prune that combination
    
    // 3) User-provided restrictions
    // In addition to damage-related identifications (from 2), if a user-provided restriction (ie: ehp, mana regen) is impossible to
    // achieve with part of a build + best identifications of remaining item types, prune that combination. 
    
    // In theory, these rules should dramatically reduce the number of item combinations that have to be searched, while
    // similtaneously ensuring that the best combination does not get skipped. 

    /// Sets up the best build calculator
    /// 
    /// `items` should be formatted in order [[helmet], [chestplate], [leggings], [boots], [ring1], [ring2], [bracelet], [necklace]]
    pub fn make(weapon: WynnItem, items: [Vec<WynnItem>; 8], atree: Rc<AtreeBuild>, build_ord: fn(&WynnBuild) -> i32) -> Self{
        // used to store the highest skill points an item type can provide. 
        // for example; given Prowess, Anya's Penumbra, and Esclavage, [5,4,12,5,4] gets stored 
        // (prowess provides highest dex and agi values, anya provides highest int value, and esclavage provides highest str and def values)
        // note that for optimization in the main search loop, these values automatically get summed with following item types
        // ie, best_skills.last stores the maximum skills for necklaces, best_skills[-2] stores maximum skills for bracelets + necklaces, etc...
        let mut best_skills: Vec<I12x5> = Vec::new();
        best_skills.push(I12x5::ZERO);

        // used to store the highest stat values in the same way best_skills stores the highest skills provided 
        // by items of a certain item type
        let mut best_ids: Vec<[i32; Atrs::NUM_STATS]> = Vec::new();
        best_ids.push([0; Atrs::NUM_STATS]);

        // to take full advantage of alpha-beta pruning, this function attempts to order items so that 'better' items come first
        // currently, this function sorts items by the amount of damage they do alone with the provided weapon (given 150 of all skill points)
        // the way items are sorted is subject to change. 
        let mut items_optimized_order: [Vec<WynnItem>; 8] = Default::default();

        // To easily pause and resume progress of the build search, one u64 is used to define what item combination we're on. 
        // For example, a combination consisting of all items[0] will be defined as 0
        // a combination consisting of all items[0] except necklace[1] will be defined as 1
        // a combination consisting of all items.last() will be defined as `num_helmets` * `num_chestplates` * ... * `num_necklaces` - 1
        // note that the order of items in the items[] array gets re-sorted and stored into items_optimized_order, so all 
        // items_optimized_order[0] won't necessarily be the same as the items[] argument provided to this function. 
        // 
        // this variable is used to store the multiplication factor to get the next item type at the given index
        // ie, if there's 4 necklaces and 2 bracelets (8 combinations):
        // necklace[0] + bracelet[0] = 0, necklace[1] + bracelet[0] = 1, necklace[0] + bracelet[1] = 4
        // This means that the combination # will have to progress by 1 for the next necklace, and 4 for the next bracelet
        // Thus, counter_mults[8] = 1, counter_mults[7] = 4, counter_mults[0..6] = 8
        // Note that this means counter_mults[0] will always be the maximum number of combinations
        let mut counter_mults: [u64; 9] = [0; 9];
        counter_mults[0]=1;
        
        // note this is reversed, so going in order necklace > bracelet > ring2 ...
        for (n, i) in items.iter().rev().enumerate(){
            counter_mults[n+1]=i.len() as u64 * counter_mults[n];

            let mut best = I12x5::ZERO; 
            let mut best_id = [0; Atrs::NUM_STATS];
            for itm in i{
                best = itm.get_skill_bonuses().max(best); // bubble up highest skill bonuses
                for id in itm.iter_ids(){ // bubble up highest stat bonuses
                    if id.1 > best_id[id.0 as usize - Atrs::NUM_NON_STATS] {best_id[id.0 as usize - Atrs::NUM_NON_STATS] = id.1}
                }
            }

            best_skills.push(best+*best_skills.last().unwrap_or(&I12x5::ZERO)); // sum highest skill bonuses with next item type
            match best_ids.last(){
                Some(n) => for j in 0_usize..Atrs::NUM_STATS{best_id[j]+=n[j]}, // do the same for stats/ids
                None => ()
            };
            best_ids.push(best_id);
            
            // re-sorting items to improve alpha-beta search performance (see comment for items_optimized_order)
            let mut temp = i.clone();
            temp.sort_by(|itm1, itm2| build_ord(&make_build!(&[*itm1, weapon], 106, I12x5::fill_data(150)).unwrap()).cmp(
                &build_ord(&make_build!(&[*itm2, weapon], 106, I12x5::fill_data(150)).unwrap())));
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
        let mut temp = Self{items: items_optimized_order, weapon, curr: 1, counter_mults, curr_bests: Vec::new(), best_skills, best_ids, min_stat_reqs: Vec::new(), max_stat_reqs: Vec::new(), atree, calc_ord: build_ord}; // clone?
        
        // for some reason the first combination needs to be checked here in the constructor (i forget why)
        match temp.make_build_if_reqs_met(&[temp.weapon,temp.items[0][0],temp.items[1][0],temp.items[2][0],temp.items[3][0],temp.items[4][0],temp.items[5][0],temp.items[6][0],temp.items[7][0]]){
            Some(b) => temp.curr_bests.push((build_ord(&b), b)),
            None => ()
        }
        // for i in 0..10{temp.curr_bests.push((22000, WynnBuild::make_from_names(&["Dune Storm", "Dondasch", "Dizzy Spell", "Revenant", "Dispersion", "Dispersion", "Knucklebones", "Diamond Static Necklace", "Oak Wood Spear"], 200).unwrap()))};
        // console::log_1(&format!("start test {}",start).into());
        temp
    }
    // /// Defines 'best build' (ie: spell_dam, melee_dam, ehp)
    // fn calc_ord(bld: &WynnBuild) -> i32 {
    //     // bld.calc_spell_dam() as i32
    //     bld.calc_melee_dam() as i32
    // }
    /// Makes a build if user-requirements are met and the build is better than the previous best found builds. 
    fn make_build_if_reqs_met(&self, bld: &[WynnItem]) -> Option<WynnBuild>{
        match make_build!(bld, 106, self.best_skills[bld.len()-1], self.atree.clone(), &self.best_ids[bld.len()-1]){
            Some(b) => if self.min_stat_reqs.iter().all(|req| match req { 
                    SearchReq::Stat(atr, val) => b.get_stat(*atr)>=*val, 
                    SearchReq::Calc(calc, val) => (calc.ord_fn_f32())(&b) >= *val }) && // user-defined minimum stat reqs met
                (self.curr_bests.is_empty() || (self.calc_ord)(&b)>self.curr_bests.last().unwrap().0) && // better than previously found best builds
                (bld.len() < 9 || self.max_stat_reqs.iter().all(|req| match req { // user-defined maximum stat reqs met
                    SearchReq::Stat(atr, val) => b.get_stat(*atr)<=*val, 
                    SearchReq::Calc(calc, val) => (calc.ord_fn_f32())(&b) <= *val }) )
                    {Some(b)} 
                else 
                    {None},
            None => {None}
        }
    }
    /// Given our combination #, returns what item idx we're on for that item type
    /// 
    /// # Example:
    /// ```
    /// // assume self.items contains 4 bracelets and 4 necklaces, and self.curr = 9
    /// asserteq!(get_curr_item_idx(Type::Bracelet as usize), 2);
    /// ```
    fn get_curr_item_idx(&self, idx: usize) -> usize{
        (self.curr/self.counter_mults[idx+1]) as usize % self.items[idx].len()
    }
    /// The main loop for calculating the best build.
    /// 
    /// This loop stops after `stop` item combinations have been tested, and will resume from where it stopped when called again.
    pub fn calc_best_build(&mut self, stop: u32) -> bool{
        // console::log_1(&format!("calcing... {}",(self.curr[0]*self.items[1].len()+self.curr[1]) as f32 / (self.items[0].len()*self.items[1].len()) as f32).into());
        let mut counter = 0;
        // console::log_1(&format!("pct {}",self.curr as f32/self.counter_mults[0] as f32).into());
        // println!("pct {}",self.curr as f32/self.counter_mults[0] as f32);
        let mut bld_items = [self.weapon, self.items[0][self.get_curr_item_idx(0)], self.items[1][self.get_curr_item_idx(1)], self.items[2][self.get_curr_item_idx(2)], self.items[3][self.get_curr_item_idx(3)], self.items[4][self.get_curr_item_idx(4)], self.items[5][self.get_curr_item_idx(5)], self.items[6][self.get_curr_item_idx(6)], self.items[7][self.get_curr_item_idx(7)]];
        
        // I coded this 2 months ago, no clue what's going on here. 
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
                                let ord_val = (self.calc_ord)(&b);
                                if self.curr_bests.len()<10{
                                    let idx = match self.curr_bests.binary_search_by(|b| ord_val.cmp(&b.0)){Ok(v) => v, Err(v) => v};
                                    // println!("inserting at {}",idx);
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
        true
    }
    /// Returns the progress the search has made so far. 
    pub fn progress(&self) -> f64{
        self.curr as f64/self.counter_mults[0] as f64
    }
    /// Returns the progress as a tuple where the first result is the numerator and the second result is the denominator
    pub fn progress_frac(&self) -> (u64,u64){
        (self.curr, self.counter_mults[0])
    }
    /// The total number of combinations to complete the search
    pub fn total_combinations(&self) -> u64{
        self.counter_mults[0]
    }
    pub fn peek_curr_bests(&self) -> std::slice::Iter<'_, (i32, WynnBuild)>{
        self.curr_bests.iter()
    }
    pub fn peek_curr_best(&self) -> &(i32, WynnBuild){
        self.curr_bests.first().unwrap()
    }
    pub fn set_min_stat_requirements(&mut self, reqs: Vec<SearchReq>){
        self.min_stat_reqs = reqs
    }
    pub fn set_max_stat_requirements(&mut self, reqs: Vec<SearchReq>){
        self.max_stat_reqs = reqs;
    }
    pub fn skip_combinations(&mut self, skip_to: u64){
        if skip_to>0{self.curr=skip_to}
    }
    /// Sets 'curr_bests' (the currently found best builds). You should set build_ord before calling this.
    pub fn set_best_builds(&mut self, bests: Vec<WynnBuild>){
        self.curr_bests=bests.iter().map(|b| ((self.calc_ord)(b),b.clone())).collect::<Vec<(i32, WynnBuild)>>();
    }
    
}
impl Default for BestBuildSearch{
    fn default() -> Self {
        Self { items: Default::default(), weapon: WynnItem::NULL, curr: Default::default(), counter_mults: Default::default(), curr_bests: Default::default(), best_skills: Default::default(), best_ids: Default::default(), min_stat_reqs: Default::default(), max_stat_reqs: Default::default(), atree: Default::default(), calc_ord: |bld| bld.calc_melee_dam(true) as i32}
    }
}