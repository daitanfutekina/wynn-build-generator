use std::rc::Rc;

use web_sys::{console, Event, HtmlInputElement};
use yew::{prelude::*, virtual_dom::AttrValue};
use gloo::timers::callback::Timeout;

use crate::wynn_data::{items, items::{WynnItem, Type, Category, Tier}, Class, atree::{AtreeBuild}, builder::WynnBuild};

#[derive(Properties, PartialEq)]
pub struct BuildDispProps{
    pub build: WynnBuild
}
pub struct BuildDisp {}

impl Component for BuildDisp{
    type Message = ();

    type Properties = BuildDispProps;

    fn create(ctx: &Context<Self>) -> Self {
        BuildDisp{}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            <a href={format!("https://wynnbuilder.github.io/builder/#9_{}",ctx.props().build.generate_hash())} target="_blank">
                <div class="build-display">
                    <div class="build-items">
                    {
                        ctx.props().build.iter_items().map(|itm|
                            html!{<div class={format!("{}",itm.get_tier())}>
                                <h2>{itm.name()}</h2>
                            </div>}
                        ).collect::<Html>()
                    }
                    </div>
                </div>
            </a>
        }
    }
}