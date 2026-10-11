use tokio::sync::{Mutex, OnceCell, RwLock};

use super::{impl_wrapper, Flattenable, TypeVisitor, TS};

impl_wrapper!(for Mutex<T>, generics: T: TS);
impl_wrapper!(for OnceCell<T>, generics: T: TS);
impl_wrapper!(for RwLock<T>, generics: T: TS);
