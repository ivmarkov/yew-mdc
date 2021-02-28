pub mod list_item;
pub mod list_item_text;
pub mod list_divider;
pub mod list_group;

pub use list_item::Item;
pub use list_item_text::ItemText;
pub use list_divider::Divider;
pub use list_group::Group;

use core::fmt;
use yew::prelude::*;
use crate::mdc_sys::MDCList;

pub struct List {
    props: Props,
    inner: Option<MDCList>,
    node_ref: NodeRef,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Role {
    None,
    ListBox,
    RadioGroup,
    Group,
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
            Role::ListBox => "listbox",
            Role::RadioGroup => "radiogroup",
            Role::Group => "group",
        };
        write!(f, "{}", role)
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,

    #[prop_or_default]
    pub id: String,

    #[prop_or_default]
    pub classes: String,

    #[prop_or_default]
    pub raw_css: String,

    #[prop_or_default]
    pub role: Role,

    #[prop_or_default]
    pub dense: bool,

    // #[prop_or_default]
    // pub vertical: bool,

    // #[prop_or_default]
    // pub wrapFocus: bool,

    // #[prop_or_default]
    // pub typeaheadInProgress: bool,

    // #[prop_or_default]
    // pub hasTypeahead: bool,

    // #[prop_or_default]
    // pub singleSelection: bool,

    // #[prop_or_default]
    // pub selectedIndex: Option<u32>,
}

impl Component for List {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,
            inner: None,
            node_ref: NodeRef::default(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(list) = self.node_ref.cast::<web_sys::Element>().map(MDCList::new) {
                self.inner = Some(list);
            }
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
        let classes = format!("mdc-list {}", self.props.classes);
        html! {
            <ul class=classes
                 role=self.props.role
                 ref=self.node_ref.clone()
                 id=&self.props.id
                 onclick=Callback::noop()
                >
                { self.props.children.clone() }
            </ul>
        }
    }

    fn destroy(&mut self) {
        if let Some(ref inner) = self.inner {
            inner.destroy();
        }
    }
}
