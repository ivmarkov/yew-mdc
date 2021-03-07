use yew::prelude::*;

pub struct ItemSecondaryText {
    props: Props,
    node_ref: NodeRef,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,

    #[prop_or_default]
    pub id: String,

    #[prop_or_default]
    pub classes: String,
}

impl Component for ItemSecondaryText {
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
        let classes = format!("mdc-list-item__secondary-text {}", self.props.classes);
        html! {
            <span class=classes
                 ref=self.node_ref.clone()
                 id=&self.props.id
                 onclick=Callback::noop()
                >
                { self.props.children.clone() }
            </span>
        }
    }
}
