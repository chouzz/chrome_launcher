
pub fn get_default<T, F>(option: Option<T>, default_fn: F) -> T
where
    F: FnOnce() -> T,
{
    option.unwrap_or_else(default_fn)
}
