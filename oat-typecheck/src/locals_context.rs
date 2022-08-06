use std::rc::Rc;

use indexmap::IndexMap;

use oat_ast::Id;

#[derive(Debug, Clone)]
pub(crate) struct LocalsContext<T>
where
    T: Clone,
{
    parent: Option<Rc<LocalsContext<T>>>,
    locals: IndexMap<Id, T>,
}

impl<T: Clone> LocalsContext<T> {
    pub fn new() -> LocalsContext<T> {
        LocalsContext {
            parent: None,
            locals: Default::default(),
        }
    }

    pub fn new_child(self) -> LocalsContext<T> {
        LocalsContext {
            parent: Some(Rc::new(self)),
            locals: Default::default(),
        }
    }

    pub fn lookup(&self, name: Id) -> Option<T> {
        if let Some(v) = self.locals.get(&name) {
            return Some(v.clone());
        }
        self.parent.as_ref().and_then(|parent| parent.lookup(name))
    }

    pub fn set(&mut self, name: Id, value: T) {
        self.locals.insert(name, value);
    }
}

impl<T: Clone> Default for LocalsContext<T> {
    fn default() -> Self {
        Self::new()
    }
}
