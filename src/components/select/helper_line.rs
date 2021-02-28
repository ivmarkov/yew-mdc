use yew::prelude::*;

pub struct HelperLine {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub persistent: bool,
    #[prop_or_default]
    pub validation_msg: bool,
}

impl Component for HelperLine {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn change(&mut self, props: Props) -> ShouldRender {
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
        let persistent = if self.props.persistent {
            " mdc-select-helper-text--persistent"
        } else {
            ""
        };
        let validation_msg = if self.props.validation_msg {
            " mdc-select-helper-text--validation-msg"
        } else {
            ""
        };
        let classes = format!("mdc-select-helper-text{}{}", persistent, validation_msg);
        html! {
            <div class=classes aria-hidden="true">
                { self.props.children.clone() }
            </div>
        }
    }
}
