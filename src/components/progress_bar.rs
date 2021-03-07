use yew::prelude::*;
use crate::mdc_sys::MDCLinearProgress;

pub struct ProgressBar {
    props: Props,
    inner: Option<MDCLinearProgress>,
    node_ref: NodeRef,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub id: String,

    #[prop_or_default]
    pub classes: String,

    #[prop_or_default]
    pub value: Option<f64>,

    #[prop_or_default]
    pub closed: bool,
}

impl Component for ProgressBar {
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
            if let Some(progress) = self.inner.take() {
                progress.destroy();
            }
            if let Some(progress) = self.node_ref.cast::<web_sys::Element>().map(MDCLinearProgress::new) {
                self.inner = Some(progress);
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            if let Some(progress) = &self.inner {
                if self.props.closed {
                    progress.close();
                } else {
                    progress.open();
                }
                progress.set_determinate(self.props.value != None);
            }
            true
        } else {
            false
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let classes = format!(
            "mdc-linear-progress {}{}{}",
            self.props.classes,
            if self.props.value == None { " mdc-linear-progress--indeterminate"} else {""},
            if self.props.closed { " mdc-linear-progress--closed"} else {""});
        html! {
            <div
                role="progressbar"
                id=&self.props.id
                class=classes
                aria-valuemin=0
                aria-valuemax=1
                aria-valuenow=self.props.value.or(Some(0 as f64)).unwrap()
                ref=self.node_ref.clone()>
                <div class="mdc-linear-progress__buffering-dots"></div>
                <div class="mdc-linear-progress__buffer"></div>
                <div class="mdc-linear-progress__bar mdc-linear-progress__primary-bar">
                    <span class="mdc-linear-progress__bar-inner"></span>
                </div>
                <div class="mdc-linear-progress__bar mdc-linear-progress__secondary-bar">
                    <span class="mdc-linear-progress__bar-inner"></span>
                </div>
            </div>
        }
    }

    fn destroy(&mut self) {
        if let Some(ref inner) = self.inner {
            inner.destroy();
        }
    }
}
