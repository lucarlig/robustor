use crate::tools::diagnose;
use dioxus::{html::input_data::keyboard_types::Key, prelude::*};
use im_rc::Vector;

#[derive(Clone)]
pub struct Message {
    pub content: String,
}

pub type Messages = Vector<Message>;

pub fn get_app(cx: Scope) -> Element {
    let messages: &UseRef<Messages> = use_ref(cx, Messages::new);
    let main_rs_state = use_state(cx, || "".to_string());
    let cargo_toml_state = use_state(cx, || "".to_string());

    let main_rs_curr = main_rs_state.current().to_string();
    let cargo_toml_curr = cargo_toml_state.current().to_string();

    cx.render(rsx!(
        div {
            messages.read().iter().map(|message| rsx! {
            p { message.content.clone()}
        }),
                input {
                    value: "{main_rs_curr}",
                    placeholder: "main.rs",
                    oninput: move |evt| main_rs_state.set(evt.value.clone())
                }
                input {
                    value: "{cargo_toml_curr}",
                    placeholder: "cargo.toml",
                    oninput: move |evt| cargo_toml_state.set(evt.value.clone())
                }
                button {
                    onclick: move |_| {
                        messages
                            .write()
                            .push_back(Message {
                                content: diagnose(&main_rs_curr.clone(), &cargo_toml_curr.clone()),
                            });
                    },
                    "Diagnose"
                }

        }
    ))
}
