#![allow(non_snake_case)]

use std::collections::hash_map::Values;
use dioxus::{prelude::*};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
pub struct KeyedNotifications {
    pub inner: HashMap<String, String>,
}

// This is primarily delegated functionality on a hashmap
// press ctrl + . to see deletegations for HashMap
impl KeyedNotifications {
    pub fn set<K, V>(&mut self, key: K, value: V)
    where 
        K: Into<String>,
        V: Into<String>,
    {
        self.inner.insert(key.into(), value.into());
    }
    
    pub fn remove<K: AsRef<str>>(&mut self, key: K) 
    {   // ASref str because it requires borrowed str
        self.inner.remove(key.as_ref());
    }
    //std::collections::hash_map::
    pub fn messages(&self) -> Values<'_, String, String> {
        self.inner.values()
    }

    pub fn has_messages(&self) -> bool {
        !self.inner.is_empty()
    }

    
}

#[derive(PartialEq, Props)]
pub struct KeyedNotificationsProps<'a> {
    legend: Option<&'a str>,
    notifications: KeyedNotifications,
}

pub fn KeyedNotificationBox<'a>(cx: Scope<'a, KeyedNotificationsProps<'a>>) -> Element {
    let notifications = cx.props.notifications.messages().map(|msg| {
        rsx!{li { "{msg}"} }
    });

    let legend = cx.props.legend.unwrap_or("Errors");

    match cx.props.notifications.has_messages() {
        true => cx.render(rsx! {
            fieldset {
                class: "fieldset border-red-300 rounded",
                legend {
                    class: "bg-red-300 px-4",
                    "{legend}"
                }
                ul {
                    class: "list-disc ml-4",
                    notifications
                }
            }
        }),
        false => None
    }
}
