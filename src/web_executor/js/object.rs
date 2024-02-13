use crate::web_executor::js::{JSContext, JSFunction, JSFunctionVariadic, JSRuntime, JSValue};
use core::fmt::Display;

pub trait JSObject {
    type RT: JSRuntime;

    fn set_property(
        &self,
        name: &str,
        value: &<Self::RT as JSRuntime>::Value,
    ) -> crate::types::Result<()>;

    fn get_property(&self, name: &str) -> crate::types::Result<<Self::RT as JSRuntime>::Value>;

    fn call_method(
        &self,
        name: &str,
        args: &[&<Self::RT as JSRuntime>::Value],
    ) -> crate::types::Result<<Self::RT as JSRuntime>::Value>;

    fn set_method(
        &self,
        name: &str,
        func: &<Self::RT as JSRuntime>::Function,
    ) -> crate::types::Result<()>;

    fn set_method_variadic(
        &self,
        name: &str,
        func: &<Self::RT as JSRuntime>::FunctionVariadic,
    ) -> crate::types::Result<()>;

    #[allow(clippy::type_complexity)]
    fn set_property_accessor(
        &self,
        name: &str,
        getter: Box<dyn Fn(&mut <Self::RT as JSRuntime>::GetterCB)>,
        setter: Box<dyn Fn(&mut <Self::RT as JSRuntime>::SetterCB)>,
    ) -> crate::types::Result<()>;
}

pub trait JSGetterCallback {
    type RT: JSRuntime;

    fn context(&mut self) -> &mut <Self::RT as JSRuntime>::Context;

    fn error(&mut self, error: impl Display);

    fn ret(&mut self, value: <Self::RT as JSRuntime>::Value);
}

pub trait JSSetterCallback {
    type RT: JSRuntime;

    fn context(&mut self) -> &mut <Self::RT as JSRuntime>::Context;

    fn error(&mut self, error: impl Display);

    fn value(&mut self) -> &<Self::RT as JSRuntime>::Value;
}
