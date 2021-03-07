use crate::mdc_sys::MDCRipple;
use web_sys::Element;
use core::fmt;
use yew::prelude::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Role {
    None,
    Option,
    Radio,
    Checkbox,
}

impl Default for Role {
    fn default() -> Role {
        Role::None
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let role = match self {
            Role::None => "",
            Role::Option => "option",
            Role::Radio => "radio",
            Role::Checkbox => "checkbox",
        };
        write!(f, "{}", role)
    }
}

pub struct Item {
    props: Props,
    node_ref: NodeRef,
    ripple: Option<MDCRipple>,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,

    #[prop_or_default]
    pub id: String,

    #[prop_or_default]
    pub classes: String,

    #[prop_or_default]
    pub role: Role,

    #[prop_or_default]
    pub selected: bool,

    #[prop_or_default]
    pub value: String,

    #[prop_or(-1)]
    pub tabindex: i32,

    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
}

pub enum Msg {
    Clicked(MouseEvent),
}

impl Component for Item {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            ripple: None,
            node_ref: NodeRef::default(),
            link,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(ripple) = self.ripple.take() {
                ripple.destroy();
            }
            self.ripple = self.node_ref.cast::<Element>().map(MDCRipple::new);
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked(ev) => {
                self.props.onclick.emit(ev);
            }
        }
        false
    }

    fn view(&self) -> Html {
        let classes = format!("mdc-list-item {}{}",
            self.props.classes,
            if self.props.selected {" mdc-list-item--selected"} else {""});
        html! {
            <li class=classes
                 ref=self.node_ref.clone()
                 role=self.props.role
                 tabindex=self.props.tabindex
                 aria-selected=self.props.selected
                 id=&self.props.id
                 data-value=self.props.value
                 onclick=self.link.callback(Msg::Clicked)
                >
                <span class="mdc-list-item__ripple"></span>
                { self.props.children.clone() }
            </li>
        }
    }

    fn destroy(&mut self) {
        if let Some(ripple) = &self.ripple {
            ripple.destroy();
        }
    }
}
