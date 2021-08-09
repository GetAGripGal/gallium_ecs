use std::any::TypeId;

/** Get the type name of a type */
pub(crate) fn nameof<T>(_: &T) -> &'static str {
    return std::any::type_name::<T>();
}

/** Get the type-id of a type */
pub(crate) fn typeid<T: 'static>(_: &T) -> std::any::TypeId {
    return std::any::TypeId::of::<T>();
}