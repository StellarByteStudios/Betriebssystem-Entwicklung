use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::{
    fmt,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
};

use hashbrown::HashSet;
use spin::{Lazy, Mutex};

use crate::kernel::threads::thread::Thread;

#[derive(Debug, Clone)]
pub struct Variable {
    name: String,
    content: String,
}

static ENVIRONMENT: Lazy<Mutex<HashSet<Variable>>> = Lazy::new(|| Mutex::new(HashSet::new()));

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Variable {}

impl Hash for Variable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state); // nur nach name hashen
    }
}

impl Variable {
    pub fn new(name: &str, content: &str) -> Self {
        return Variable {
            name: name.to_string(),
            content: content.to_string(),
        };
    }

    pub fn get_content(&self) -> String {
        return self.content.to_string();
    }
}

pub fn env_contains(name: &str) -> bool {
    let variables = ENVIRONMENT.lock();
    let contains = variables.contains(&Variable::new(name, ""));
    return contains;
}

pub fn env_insert(name: &str, content: &str) {
    let mut variables = ENVIRONMENT.lock();
    variables.replace(Variable::new(name, content));
}

pub fn env_remove(name: &str) {
    let mut variables = ENVIRONMENT.lock();
    variables.remove(&Variable::new(name, ""));
}

pub fn env_get(name: &str) -> Option<String> {
    let variables = ENVIRONMENT.lock();
    let env_var_option = variables
        .get(&Variable::new(name, ""))
        .map(|var| var.content.clone());
    return env_var_option;
}
