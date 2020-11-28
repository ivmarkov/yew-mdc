use yew::prelude::*;

pub struct Group {
    props: Props,
    node_ref: NodeRef,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,

    #[prop_or_default]
    pub id: String,

    #[prop_or_default]
    pub sub_header: Option<String>,
}

impl Component for Group {
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
        let maybe_display_header = move || -> Html {
            if let Some(ref text) = self.props.sub_header {
                html! {<h3 class="mdc-list-group__subheader">{text}</h3>}
            } else {
                html! {}
            }
        };

        html! {
            <div class="mdc-list-group"
                 ref=self.node_ref.clone()
                 id=&self.props.id
                 onclick=Callback::noop()
                >
                { maybe_display_header() }
                { self.props.children.clone() }
            </div>
        }
    }
}
