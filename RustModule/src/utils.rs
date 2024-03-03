use std::any::{Any, TypeId};
use std::collections::HashMap;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}



pub(crate) struct ServiceRegistry
{
    services: HashMap<TypeId, Box<dyn Any>>, // Box is like a unique_ptr in C++
}

impl ServiceRegistry
{
    pub(crate) fn new() -> Self
    {
        ServiceRegistry {
            services: HashMap::new(),
        }
    }

    pub(crate) fn add_service<T: Any + 'static>(&mut self, service: T)
    {
        self.services.insert(TypeId::of::<T>(), Box::new(service));
    }

    pub(crate) fn get_service<T: Any + 'static>(&self) -> Option<&T>
    {
        self.services.get(&TypeId::of::<T>()).and_then(|boxed_any| boxed_any.downcast_ref::<T>())
    }
}