pub fn into_error_or<T, E>(err: Option<E>, default: T) -> Result<T, E> {
    match err {
        None => Ok(default),
        Some(err) => Err(err),
    }
}
