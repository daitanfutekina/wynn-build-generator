use std::rc::Rc;

use yew::{prelude::*, virtual_dom::AttrValue};

use crate::wynn_data::{items::Type, builder::WynnBuild};
use crate::best_build_search::helper_enums::SearchParam;
use super::{STAT_RENAMES,STAT_COLOR_CLASSES};

#[derive(Properties, PartialEq)]
pub struct BuildDispProps{
    #[prop_or("".into())]
    pub title: AttrValue,
    pub build: WynnBuild,
    #[prop_or_default]
    pub disp_stats: Rc<Vec<SearchParam>>
}
pub struct BuildDisp {}

impl Component for BuildDisp{
    type Message = ();

    type Properties = BuildDispProps;

    fn create(_ctx: &Context<Self>) -> Self {
        BuildDisp{}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let build = &ctx.props().build;
        let weapon = build.get_item(8);
        let disp_stats = &ctx.props().disp_stats;
        html!{
            <a href={format!("https://wynnbuilder.github.io/builder/#9_{}",build.wynnbuilder_hash())} target="_blank">
                <div class="build-display">
                    <h2>{&ctx.props().title}</h2>
                    <fieldset class={format!("build-weapon")}>
                    <legend>{"Weapon"}</legend>
                    <h3 class={weapon.get_tier().to_string()}>{weapon.name()}</h3>
                    </fieldset>
                    <div class="build-stats">
                        <h3>{"Stat Summary"}</h3>
                        {
                            disp_stats.iter().map(|stat| html!{
                                <div class="build-stat">
                                    <div class={format!("stat-type {}",STAT_COLOR_CLASSES[stat.usize_id()])}>{STAT_RENAMES[stat.usize_id()]}</div>
                                    <div class="stat-comparator">{":"}</div>
                                    <div class="stat-value">{match stat{SearchParam::Calc(c) => format!("{:.2}",(c.ord_fn_f32())(build)), SearchParam::Stat(a) => build.get_stat(*a).to_string()}}</div>
                                </div>
                            }).collect::<Html>()
                        }
                    </div>
                    <div class="build-items">
                    {
                        (0..8).map(|i| 
                        {
                            let itm = build.get_item(i);
                            html!{
                                <fieldset class={format!("build-item")}>
                                    <legend>{format!("{}",Type::try_from(if i<5{i}else{i-1}).unwrap())}</legend>
                                    <h3 class={itm.get_tier().to_string()}>{itm.name()}</h3>
                                </fieldset>
                            }
                        }
                        ).collect::<Html>()
                    }
                    </div>
                </div>
            </a>
        }
    }
}