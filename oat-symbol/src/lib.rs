use std::cell::RefCell;
use std::num::NonZeroU32;

use indexmap::IndexSet;

use scoped_tls;

mod arena;
use arena::DroplessArena;

pub struct GlobalSession {
    interner: Interner,
}

scoped_tls::scoped_thread_local!(static SESSION: GlobalSession);

#[inline]
pub fn with_global_interner<R>(f: impl FnOnce(&GlobalSession) -> R) -> R {
    SESSION.with(f)
}

/// Creates the session globals and then runs the closure `f`.
#[inline]
pub fn create_session_if_not_set_then<R>(f: impl FnOnce(&GlobalSession) -> R) -> R {
    if !SESSION.is_set() {
        let gs = GlobalSession {
            interner: Interner::new(),
        };
        SESSION.set(&gs, || SESSION.with(f))
    } else {
        SESSION.with(f)
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
pub struct Symbol(NonZeroU32);

impl Symbol {
    pub fn new(index: u32) -> Self {
        Self(unsafe { NonZeroU32::new_unchecked(index.saturating_add(1)) })
    }

    pub fn index(self) -> usize {
        (self.0.get() - 1) as usize
    }

    pub fn intern(name: &str) -> Self {
        with_global_interner(|session| session.interner.intern(name))
    }

    pub fn name(self) -> &'static str {
        with_global_interner(|session| session.interner.name(self.index()))
    }
}

impl std::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "symbol #{}: {}", self.index(), self.name())
    }
}

impl From<&str> for Symbol {
    fn from(s: &str) -> Symbol {
        Symbol::intern(s)
    }
}

#[derive(Default)]
struct InnerInterner {
    arena: DroplessArena,
    symbol_set: IndexSet<&'static str>,
}

#[derive(Default)]
struct Interner {
    inner: RefCell<InnerInterner>,
}

impl Interner {
    fn new() -> Self {
        Interner {
            inner: RefCell::new(Default::default()),
        }
    }

    fn intern(&self, name: &str) -> Symbol {
        let InnerInterner { arena, symbol_set } = &mut *self.inner.borrow_mut();

        if let Some(sym) = symbol_set.get_index_of(name) {
            return Symbol::new(sym as u32);
        }

        let bytes = arena.alloc_slice(name.as_bytes());
        let name: &'static str =
            unsafe { std::intrinsics::transmute(std::str::from_utf8_unchecked(bytes)) };
        // let name = Box::leak(name.to_string().into_boxed_str());

        let index = symbol_set.insert_full(name).0;
        Symbol::new(index as u32)
    }

    #[inline]
    fn name(&self, index: usize) -> &'static str {
        self.inner.borrow().symbol_set.get_index(index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::create_session_if_not_set_then;

    use super::Symbol;
    #[test]
    fn test_intern() {
        create_session_if_not_set_then(|_| {
            let name = "name";
            dbg!(Symbol::intern(name));
            dbg!(Symbol::intern("ed"));
            dbg!(Symbol::intern("was"));
            dbg!(Symbol::intern("hello"));
            dbg!(Symbol::intern("name"));
            dbg!(Symbol::intern("ed"));
        });
    }
}
