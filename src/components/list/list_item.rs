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
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,

    #[prop_or_default]
    pub id: String,

    #[prop_or_default]
    pub role: Role,

    #[prop_or_default]
    pub selected: bool,
}

impl Component for Item {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,
            node_ref: NodeRef::default(),
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

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let classes = format!("mdc-list-item {}", if self.props.selected {"mdc-list-item--selected"} else {""});
        html! {
            <li class=classes
                 ref=self.node_ref.clone()
                 role=self.props.role
                 aria_selected=self.props.selected
                 id=&self.props.id
                 onclick=Callback::noop()
                >
                <span class="mdc-list-item__ripple"></span>
                { self.props.children.clone() }
            </li>
        }
    }
}
