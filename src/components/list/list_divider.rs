use yew::prelude::*;

pub struct Divider {
    props: Props,
    node_ref: NodeRef,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub id: String,

}

impl Component for Divider {
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
        let classes = format!("mdc-list-divider");
        html! {
            <li class=classes
                 ref=self.node_ref.clone()
                 role="divider"
                 id=&self.props.id
                 onclick=Callback::noop()
                >
            </li>
        }
    }
}
