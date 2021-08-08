/** Get the type name of a struct */
pub(crate) fn nameof<T>(_: &T) -> &'static str {
    return std::any::type_name::<T>();
}
