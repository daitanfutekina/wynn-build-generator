use std::rc::Rc;
use web_sys::{DomRect, Element, HtmlInputElement};
use yew::{prelude::*, virtual_dom::AttrValue};
use gloo::timers::callback::Timeout;

#[derive(Properties, PartialEq)]
pub struct AutocompleteInputProps{
    #[prop_or("".into())]
    /// Applies the `class` html tag to the wrapper div and its direct children
    pub class: AttrValue,
    #[prop_or("".into())]
    /// Applies the `id` html tag to the wrapper div
    pub id: AttrValue,
    #[prop_or("".into())]
    /// Applies the `name` html tag to the input 
    pub name: AttrValue,
    #[prop_or("".into())]
    /// Applies the `placeholder` html tag to the input
    pub placeholder: AttrValue,
    #[prop_or_default]
    /// Applies the `disabled` html tag to the input
    pub disabled: bool,
    #[prop_or("".into())]
    /// Gives a starting value to the input. Note that this will not update the value if something has already been inputted. 
    pub value: AttrValue,
    #[prop_or_default]
    /// Clears the content of this input after it loses focus. 
    pub reset: bool,
    #[prop_or(10)]
    /// Limit the number of autocompletion options that are displayed.
    pub limit: usize,
    #[prop_or(1)]
    /// Number of characters required to display the autocomplete options
    pub char_req: usize,
    #[prop_or(true)]
    /// Ignore case when autocompleting, default is true
    pub ignore_case: bool,
    #[prop_or(0)]
    /// Sets a delay before hiding the autocomple options, giving time for animations.
    /// 
    /// If you want to just use css to control how the autocomplete option results are displayed, set this to u32::MAX
    pub unfocus_delay: u32,
    #[prop_or_default]
    /// Forces a selection to be made
    pub force: bool,
    #[prop_or(true)]
    /// Control whether the input field is editable by typing into it. Defaults to true. 
    pub editable: bool,
    #[prop_or(false)]
    /// Makes the autocomplete options box stick to the bottom edge of the input (this also matches the width of the input)
    pub stick_to_input: bool,
    #[prop_or_default]
    /// Set a callback for when the autocomplete input gets exited (ie, after something has been inputted)
    /// 
    /// The callback should accept in a Option<usize>, representing the index of the input from the options property (or None if a value not from options has been set),
    /// and a String, representing the actual content of the options property
    pub on_leave: Callback<(Option<usize>,String)>,
    #[prop_or_default]
    /// Set a callback for when an autocomplete option has been selected
    pub on_select: Callback<(usize,String)>,
    #[prop_or_default]
    /// Defines a 'class' tag to apply to the option at the respective index
    /// 
    /// This class also gets applied to the input its content matches an option
    pub options_classes: Rc<Vec<String>>,
    /// All the options to check for autocompletion
    pub options: Rc<Vec<String>>,
}

pub enum AutocompleteInputMsg{
    OnFocus(Option<DomRect>),
    OnInput(String, Option<DomRect>),
    OnSelect(usize),
    OnLeave,
    Unfocus,
    None
}
/// ```
/// pub struct AutocompleteInputProps{
/// #[prop_or("".into())]
/// pub class: AttrValue,
/// #[prop_or("".into())]
/// pub id: AttrValue,
/// #[prop_or("".into())]
/// pub name: AttrValue,
/// #[prop_or("".into())]
/// pub placeholder: AttrValue,
/// #[prop_or_default]
/// pub disabled: bool,
/// #[prop_or("".into())]
/// pub value: AttrValue,
/// #[prop_or_default]
/// pub reset: bool,
/// #[prop_or(10)]
/// pub limit: usize,
/// #[prop_or(true)]
/// pub ignore_case: bool,
/// #[prop_or(0)]
/// pub unfocus_delay: u32,
/// #[prop_or_default]
/// pub force: bool,
/// #[prop_or(true)]
/// pub editable: bool,
/// #[prop_or(false)]
/// pub stick_to_input: bool,
/// #[prop_or_default]
/// pub on_leave: Callback<(Option<usize>,String)>,
/// #[prop_or_default]
/// pub on_select: Callback<(usize,String)>,
/// #[prop_or_default]
/// pub options_classes: Rc<Vec<String>>,
/// pub options: Rc<Vec<String>>,
/// }
/// ```
/// <hr>
/// Creates an text input field with auto-completion
/// 
/// Set an on_leave callback to retrieve the input content and option index (if the result is an option) when the input field has been unfocused
/// 
/// The class prop will get applied to the outside div, input, and options div<br>
/// The id prop will only get applied to the outside div<br>
/// The name prop will only get applied to the input
/// 
/// Additionally, the outside div will come with the class `autocomplete-wrapper`, the input will come with `autocomplete-input`, 
/// and the options div will come with the class `autocomplete-options`. Use the selector `.autocomplete-options button` to modify
/// the options results.
pub struct AutocompleteInput{
    content: String,
    selected: Option<usize>,
    is_focused: bool,
    display_options: Vec<(usize,String)>,
    options: Vec<String>,
    unfocus_callback_handle: Option<Timeout>,
    input_rect: DomRect // this is a really stupid solution
}
impl Component for AutocompleteInput{
    type Message = AutocompleteInputMsg;

    type Properties = AutocompleteInputProps;

    fn create(ctx: &Context<Self>) -> Self {
        // setup autocomplete options depending on if ignore_case is enabled
        let options = if ctx.props().ignore_case{ctx.props().options.iter().map(|v| v.to_uppercase()).collect()} else {ctx.props().options.to_vec()};
        let mut content = String::new();
        let mut selected = None;
        // let mut handle = None;
        if !ctx.props().value.is_empty(){ // used to set 'selected' if a starting value has been set and it matches an option
            content = ctx.props().value.to_string();
            let content_upper = content.to_uppercase();
            match options.iter().position(|opt| content_upper.eq(opt)){
                Some(idx) => selected = Some(idx),
                None => ()
            }
        }else if ctx.props().force{ // cases for forcing valid selection
            content = ctx.props().options[0].clone();
            selected = Some(0);
        }
        Self{content, selected, is_focused: false, display_options: if ctx.props().char_req==0{ctx.props().options.iter().take(ctx.props().limit).enumerate().map(|(i,v)| (i,v.clone())).collect()}else{Vec::new()}, 
            options, unfocus_callback_handle: None, input_rect: DomRect::new().unwrap()}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            AutocompleteInputMsg::OnFocus(d) => {
                self.unfocus_callback_handle = None;
                self.is_focused=true;
                match d{Some(r) => self.input_rect=r, None => ()};
                match self.selected{
                    Some(i) => ctx.props().on_select.emit((i,self.content.clone())),
                    None => ()
                }
            }
            AutocompleteInputMsg::OnInput(content, input_rect) => {
                match input_rect{Some(r) => self.input_rect=r, None => ()};
                if !ctx.props().editable{
                    return true
                }
                self.selected=None;
                self.is_focused=true;
                let content_clone = if ctx.props().ignore_case{
                    content.to_uppercase()
                }else{
                    content.clone()
                };
                
                // display options that start with the input content before displaying options that just contain input content
                let mut options1 = Vec::new();
                let mut options2 = Vec::new();
                if content.len()>=ctx.props().char_req{
                    for (i, v) in self.options.iter().enumerate(){
                        if v.contains(&content_clone){
                            if v.starts_with(&content_clone){
                                if v.len()==content_clone.len(){
                                    self.selected=Some(i);
                                    ctx.props().on_select.emit((i,ctx.props().options[i].clone()));
                                }
                                options1.push((i,ctx.props().options[i].clone()));
                            }else{
                                options2.push((i,ctx.props().options[i].clone()));
                            }
                            if options1.len() >= ctx.props().limit {break}
                        }
                    }
                }
                options2.truncate(ctx.props().limit-options1.len());
                options1.append(&mut options2);
                self.content=content;
                self.display_options=options1;
            },
            AutocompleteInputMsg::OnSelect(option_id) => {
                // when an option is selected, set the content to match the selected option
                self.unfocus_callback_handle=None;
                self.content=ctx.props().options[option_id].clone();
                self.selected=Some(option_id);
                ctx.props().on_select.emit((option_id,self.content.clone()));
            },
            AutocompleteInputMsg::OnLeave => {
                // when the autocomplete input field is exited, set a timeout for removing the options div, and call the on_leave callback
                if self.is_focused && ctx.props().unfocus_delay!=u32::MAX{
                    let link = ctx.link().clone();
                    self.unfocus_callback_handle = Some(Timeout::new(ctx.props().unfocus_delay, move || link.send_message(AutocompleteInputMsg::Unfocus)));    
                    
                    match self.selected{
                        Some(i) => self.content=ctx.props().options[i].clone(),
                        None => if ctx.props().force{
                            let temp=self.display_options.first().unwrap_or(&(0,ctx.props().options[0].clone())).clone();
                            self.selected=Some(temp.0);
                            self.content=temp.1;
                            ctx.props().on_select.emit((temp.0,self.content.clone()));
                        }
                    }

                    ctx.props().on_leave.emit((self.selected,self.content.clone()));

                    // self.display_options=Vec::new();
                }
            },
            AutocompleteInputMsg::Unfocus => {
                self.is_focused=false;
                self.unfocus_callback_handle = None;
            },
            AutocompleteInputMsg::None => {
                return false
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html{
        let link = ctx.link();
        if ctx.props().reset && !self.is_focused &&!self.content.is_empty(){link.send_message(AutocompleteInputMsg::OnInput(String::new(),None))}
        let unfocusing = if self.unfocus_callback_handle.is_some() {"unfocusing"} else {""};
        let stick_to_input = ctx.props().stick_to_input;
        let div_content = html!{
            <>
                <input class = {format!("autocomplete-input {} {}",ctx.props().class,ctx.props().options_classes.get(self.selected.unwrap_or(usize::MAX)).unwrap_or(&String::new()))} 
                name = {ctx.props().name.clone()} disabled={ctx.props().disabled} placeholder = {ctx.props().placeholder.clone()} onfocus={link.callback(move |e: FocusEvent| {let temp: Element = e.target_unchecked_into();AutocompleteInputMsg::OnFocus(if stick_to_input{Some(temp.get_bounding_client_rect())}else{None})})} 
                oninput={link.callback(move |event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); AutocompleteInputMsg::OnInput(input.value(),if stick_to_input{Some(input.get_bounding_client_rect())}else{None})})} 
                value={self.content.clone()}/>
                if self.content.len() >= ctx.props().char_req && self.is_focused{
                    <div class={format!("autocomplete-options-wrapper {} {}",ctx.props().class,unfocusing)} style={if stick_to_input{format!("position:fixed;top:{}px;left:{}px;width:{}px;",self.input_rect.bottom(),self.input_rect.left(),self.input_rect.width())}else{String::new()}}> //style={format!("top:{}px;left:{}px;width:{}px;",self.input_rect.bottom(),self.input_rect.left(),self.input_rect.width())}
                        <div class={format!("autocomplete-options {} {}",ctx.props().class,unfocusing)}>
                            {self.display_options.iter().enumerate().map(|(_,(i,v))|
                                {let temp = i.clone();
                                html!{
                                    <button class={ctx.props().options_classes.get(*i).unwrap_or(&String::new())} onfocus={link.callback(move |_| AutocompleteInputMsg::OnSelect(temp.clone()))} onclick={link.callback(move |_| AutocompleteInputMsg::OnLeave)}>{v.clone()}</button>
                                }
                            }
                            ).collect::<Html>()}
                        </div>
                    </div>
                }
            </>
        };

        // this is just here to prevent bugs arising from a 'id = ""' in the wrapper div
        if ctx.props().id.is_empty(){
            html!{
                <div class={format!("autocomplete-wrapper {}",ctx.props().class)} onblur={link.callback(|_| AutocompleteInputMsg::OnLeave)} onkeypress={link.callback(|key:KeyboardEvent| {if key.char_code()==13 {AutocompleteInputMsg::OnLeave} else {AutocompleteInputMsg::None}})}>
                    {div_content}
                </div>
            }
        }else{
            html!{
                <div class={format!("autocomplete-wrapper {}",ctx.props().class)} id={ctx.props().id.clone()} onblur={link.callback(|_| AutocompleteInputMsg::OnLeave)} onkeypress={link.callback(|key:KeyboardEvent| {if key.char_code()==13 {AutocompleteInputMsg::OnLeave} else {AutocompleteInputMsg::None}})}>
                    {div_content}
                </div>
            }
        }
    }
}