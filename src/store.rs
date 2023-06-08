use std::cell::RefCell;

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Store {}

thread_local! {
    pub static DATA: RefCell<Store>  = RefCell::new(Store::default());
}

impl Store {}
