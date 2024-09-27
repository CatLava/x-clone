#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::{Router, Route};
use fermi::{use_init_atom_root, AtomRef};

use crate::{elements::{navbar::Navbar, toaster::{ToastRoot, Toaster}}, prelude::*};

//creates a default toaster when the application runs
pub static TOASTER: AtomRef<Toaster> = |_| Toaster::default();

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);

    let toaster = use_toaster(cx);
    cx.render(rsx! {
        Router {
            Route { to : page::ACCOUNT_REGISTER, page::Register {}}
            Route { to : page::ACCOUNT_LOGIN, page::Login {}}
            Route { to : page::HOME, page::Home {}},
            Route { to : page::POST_NEW_CHAT, page::NewChat {}},
            ToastRoot {
                toaster: toaster
            },

            // will always exist when application loads
            Navbar{}


        }

    })
}
