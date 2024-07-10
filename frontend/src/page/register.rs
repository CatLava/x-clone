#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::sync_handler;

use crate::prelude::*;


pub struct PageState {
    username: UseState<String>,
    password: UseState<String>
}

impl PageState {
    pub fn new(cx: Scope) -> Self {
        Self {
            username: use_state(cx, String::new).clone(),
            password: use_state(cx, String::new).clone()
        }
    }
}

// element have optional props
#[inline_props]
pub fn UsernameInput<'a>(
    cx: Scope<'a>,
    // It will look at the structure below and assocaite with structure
    state: UseState<String>,
    // Event handler requires lifetimes
    oninput: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "flex flex-col",
            label {
                r#for: "username",
                   "Username",

            }
            input {
                id: "username",
                name: "username",
                class: "input-field",
                placeholder: "User Name",
                value: "{state.current()}",
                // closure fith form event
                oninput: move |ev| oninput.call(ev),
            }
        }
    })
}

#[inline_props]
pub fn PasswordInput<'a>(
    cx: Scope<'a>,
    // It will look at the structure below and assocaite with structure
    state: UseState<String>,
    // Event handler requires lifetimes
    oninput: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "flex flex-col",
            label {
                r#for: "password",
                   "Password",

            }
            input {
                id: "password",
                r#type: "password",
                name: "password",
                class: "input-field",
                placeholder: "Password",
                value: "{state.current()}",
                // closure fith form event
                oninput: move |ev| oninput.call(ev),
            }
        }
    })
}
pub fn Register(cx: Scope) -> Element {
    // cx gets saved into the scope using the state
    // state keeps info across multiple function calls 
    let page_state = PageState::new(cx);
    let page_state = use_ref(cx, || page_state);

    //sync handler does it in the webpage
    //async handler needs to call out elsewhwer
    let username_oninput = sync_handler!([page_state], move |ev: FormEvent| {
        page_state.with_mut(|state| state.username.set(ev.value.clone()))
    });

    let password_oninput = sync_handler!([page_state], move |ev: FormEvent| {
        page_state.with_mut(|state| state.password.set(ev.value.clone()))
    });

    cx.render(rsx!{
        form {
            class: "flex flex-col gap-5",
            prevent_default: "onsubmit",
            onsubmit: move |_| {},
            UsernameInput {
                // .with is essentially read only
                state: page_state.with(|state| state.username.clone()),
                oninput: username_oninput
            },

            PasswordInput {
                // .with is essentially read only
                state: page_state.with(|state| state.password.clone()),
                oninput: password_oninput
            },

            button {
                class: "btn",
                // r# is raw identifier to reuse key words
                r#type: "submit",
                "Signup"
            }
        }
    })
}