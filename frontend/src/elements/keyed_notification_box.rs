#![allow(non_snake_case)]

use dioxus::{prelude::*, rsx::Element};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
pub struct KeyedNotifications {
    pub inner: HashMap<String, String>,
}

impl KeyedNotifications {
    pub fn set<K, V>(&mut self, key: K, value: V)
    where 
        K: Into<String>,
        V: Into<String>,
    {
        self.inner.insert(key.into(), value.into());
    }
    
    pub fn remove<K: AsRef<str>>(&mut self, k: &Q) 
    {
        self.inner.remove(k.as_ref())
    }
    
    pub fn messages(&self) -> std::collections::hash_map::Values<'_, String, String> {
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
    
}