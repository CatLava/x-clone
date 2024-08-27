#![allow(non_snake_case)]

use dioxus::prelude::*;
use uchat_domain::UserFacingError;

use crate::elements::keyed_notification_box::KeyedNotifications;
use crate::elements::KeyedNotificationBox;
use crate::fetch_json;
use crate::maybe_class;
use crate::sync_handler;

use crate::prelude::*;
use crate::util::api_client;
use crate::util::ApiClient;


pub struct PageState {
    username: UseState<String>,
    password: UseState<String>,
    form_errors: KeyedNotifications
}

impl PageState {
    pub fn new(cx: Scope) -> Self {
        Self {
            username: use_state(cx, String::new).clone(),
            password: use_state(cx, String::new).clone(),
            form_errors: KeyedNotifications::default(),
        }
    }

    pub fn can_submit(&self) -> bool {
        !(self.form_errors.has_messages()
            || self.username.current().is_empty()
            || self.password.current().is_empty())
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
pub fn Login(cx: Scope) -> Element {
    let api_client = ApiClient::global();
    // cx gets saved into the scope using the state
    // state keeps info across multiple function calls 
    let page_state = PageState::new(cx);
    let page_state = use_ref(cx, || page_state);

    let form_onsubmit = 
        async_handler!(&cx, [api_client, page_state],
            move |_| async move {
            // import within the closure so they don't leak out in the form
            use uchat_endpoint::user::endpoint::{Login, LoginOk};

            
            let response = fetch_json!(<CreateUserOk>, api_client, request_data);
            match response {
                Ok(res) => (),
                Err(e) => (),
            }
        }
        );
    //sync handler does it in the webpage
    //async handler needs to call out elsewhwer
    let username_oninput = sync_handler!([page_state], move |ev: FormEvent| {
        if let Err(e) = uchat_domain::Username::new(&ev.value) {
            page_state.with_mut(|state| state.form_errors.set("bad-username", e.formatted_error()))
        } else {
            page_state.with_mut(|state| state.form_errors.remove("bad-username"))

        }
        page_state.with_mut(|state| state.username.set(ev.value.clone()))
    });

    let password_oninput = sync_handler!([page_state], move |ev: FormEvent| {
        if let Err(e) = uchat_domain::Password::new(&ev.value) {
            page_state.with_mut(|state| state.form_errors.set("bad-password", e.formatted_error()))
        } else {
            page_state.with_mut(|state| state.form_errors.remove("bad-password"))

        }
        page_state.with_mut(|state| state.password.set(ev.value.clone()))
    });

    let submit_btn_style = maybe_class!("btn-disabled", !page_state.with(|state| state.can_submit()));

    cx.render(rsx!{
        form {
            class: "flex flex-col gap-5",
            prevent_default: "onsubmit",
            onsubmit: form_onsubmit,
            // need restrictions on username and password
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

            KeyedNotificationBox {
                legend: "Form Errors",
                notifications: page_state.clone().with(|state| state.form_errors.clone()),

            }

            button {
                class: "btn {submit_btn_style}",
                // r# is raw identifier to reuse key words
                r#type: "submit",
                disabled: !page_state.with(|state| state.can_submit()),
                "Login"
            }
        }
    })
}