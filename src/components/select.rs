use crate::mdc_sys::MDCSelect;

use web_sys::Element;

use wasm_bindgen::{prelude::*, JsCast};
use yew::prelude::*;

pub mod helper_line;
pub use helper_line::HelperLine;

pub struct Select {
    node_ref: NodeRef,
    inner: Option<MDCSelect>,
    props: Props,
    link: ComponentLink<Self>,
    change_callback: Closure<dyn FnMut(web_sys::Event)>,
}

#[derive(PartialEq, Properties, Clone, Debug)]
pub struct Props {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub classes: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub hint: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub outlined: bool,
    #[prop_or_default]
    pub nolabel: bool,
    #[prop_or_default]
    pub valid: Option<bool>,
    #[prop_or_else(Callback::noop)]
    pub onchange: Callback<String>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub evil_gimme_focus_callback: Option<Callback<Callback<()>>>,
}

pub enum Msg {
    ValueChanged(String),
    FocusRequested,
}

impl Component for Select {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        if let Some(ref callback) = props.evil_gimme_focus_callback {
            let my_callback = link.callback(|_| Msg::FocusRequested);
            callback.emit(my_callback);
        }

        let callback = link.callback(Msg::ValueChanged);
        let closure = Closure::wrap(Box::new(move |e: web_sys::Event| {
            if let Some(e) = e.dyn_ref::<web_sys::CustomEvent>() {
                e.stop_propagation();
                if let Ok(value) = e.detail().into_serde::<serde_json::Value>() {
                    if let Some(value) = value.get("value").and_then(|v| v.as_str()) {
                        let string = value.to_owned();
                        callback.emit(string);
                    }
                }
            }
            e.stop_propagation();
        }) as Box<dyn FnMut(web_sys::Event)>);

        Self {
            node_ref: NodeRef::default(),
            props,
            inner: None,
            change_callback: closure,
            link,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(inner) = self.inner.take() {
                inner.unlisten("MDCSelect:change", &self.change_callback);
                inner.destroy();
            }
            self.inner = self.node_ref.cast::<Element>().map(MDCSelect::new);
            if let Some(inner) = &self.inner {
                inner.listen("MDCSelect:change", &self.change_callback);
                inner.set_value(&self.props.value);
            }
        }
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        if props != self.props {
            if let Some(ref callback) = props.evil_gimme_focus_callback {
                let my_callback = self.link.callback(|_| Msg::FocusRequested);
                callback.emit(my_callback);
            }
            self.props = props;
            if let Some(inner) = &self.inner {
                if inner.value() != self.props.value {
                    // Avoid calling MDCSelect::set_value() for no reason as it leads to event firing
                    // which in turn recursively calls the ValueChanged callback closure, which is not allowed
                    inner.set_value(&self.props.value);
                }
            }
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ValueChanged(s) => {
                self.props.onchange.emit(s);
            }
            Msg::FocusRequested => {
                if let Some(ref inner) = self.inner {
                    inner.focus();
                }
            }
        };
        false
    }

    fn view(&self) -> Html {
        let disabled = if self.props.disabled {
            "" //" mdc-select--disabled"
        } else {
            ""
        };
        let outlined = if self.props.outlined {
            " mdc-select--outlined"
        } else {
            " mdc-select--filled"
        };
        let nolabel = if self.props.nolabel {
            " mdc-select--no-label"
        } else {
            ""
        };
        let classes = format!(
            "mdc-select{}{}{} {}",
            disabled, outlined, nolabel, self.props.classes
        );
        let label = if self.props.nolabel {
            html! {}
        } else {
            html! {
                <>
                    {
                        if !self.props.outlined {
                            html! {<span class="mdc-select__ripple"/>}
                        } else {
                            html! {""}
                        }
                    }

                    <span class="mdc-floating-label">
                        { &self.props.hint }
                    </span>
                </>
            }
        };
        let inner = if self.props.outlined {
            let notch = if self.props.nolabel {
                html! {}
            } else {
                html! {
                    <div class="mdc-notched-outline__notch">
                        { label }
                    </div>
                }
            };
            html! {
                <div class="mdc-notched-outline">
                    <div class="mdc-notched-outline__leading"></div>
                    { notch }
                    <div class="mdc-notched-outline__trailing"></div>
                </div>
            }
        } else {
            html! { <>
                <div class="mdc-line-ripple"></div>
                { label }
            </> }
        };
        html! {
            <div class=classes id=&self.props.id ref=self.node_ref.clone() style="width: 250px;">
                <input type="hidden" value=self.props.value/>
                <div
                    class="mdc-select__anchor"
                    role="button"
                    aria-haspopup="listbox"
                    aria-expanded=false>
                    <span class="mdc-select__ripple"></span>
                    <span class="mdc-select__selected-text-container">
                        <span class="mdc-select__selected-text"></span>
                    </span>
                    <span class="mdc-select__dropdown-icon">
                        <svg class="mdc-select__dropdown-icon-graphic" viewBox="7 10 10 5" focusable=false>
                            <polygon
                                class="mdc-select__dropdown-icon-inactive"
                                stroke="none"
                                fill-rule="evenodd"
                                points="7 10 12 15 17 10">
                            </polygon>
                            <polygon
                                class="mdc-select__dropdown-icon-active"
                                stroke="none"
                                fill-rule="evenodd"
                                points="7 15 12 10 17 15">
                            </polygon>
                        </svg>
                    </span>
                    { inner }
                </div>

                <div class="mdc-select__menu mdc-menu mdc-menu-surface mdc-menu-surface--fullwidth">
                    { self.props.children.clone() }
                </div>
            </div>
        }
    }

    fn destroy(&mut self) {
        if let Some(inner) = &self.inner {
            inner.unlisten("MDCSelect:change", &self.change_callback);
            inner.destroy();
        }
    }
}
