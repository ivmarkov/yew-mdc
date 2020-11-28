use core::fmt;
use yew::prelude::*;

pub struct ItemText {
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
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Role {
    None,
    Primary,
    Secondary,
}

impl Default for Role {
    fn default() -> Role {
        Role::None
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let role = match self {
            Role::None => "text",
            Role::Primary => "primary-text",
            Role::Secondary => "secondary-text",
        };
        write!(f, "{}", role)
    }
}

impl Component for ItemText {
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
        let classes = format!("mdc-list-item__{}", self.props.role);
        html! {
            <span class=classes
                 ref=self.node_ref.clone()
                 role=self.props.role
                 id=&self.props.id
                 onclick=Callback::noop()
                >
                { self.props.children.clone() }
            </span>
        }
    }
}
