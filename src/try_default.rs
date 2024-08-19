use crate::DefaultNotFound;

///
/// A trait for all types!
///
/// If the type has a `Default` value,
///
/// ```
///     use ::try_default::TryDefault;
///
///     // Set to `Some(0)`.
///     let default_num = <u32>::try_default();
///
///     // Set to `None`, as `::std::fs::File` has no `Default`.
///     let default_file = <::std::fs::File>::try_default();
/// ```
///
pub trait TryDefault<V> {
    /// If the implementor implements Default, then this will return
    /// `Option::Some(Default::default())`.
    ///
    /// Otherwise it returns `Option::None`.
    fn try_default() -> Result<V, DefaultNotFound>;
}

impl<V> TryDefault<V> for V {
    default fn try_default() -> Result<V, DefaultNotFound> {
        Err(DefaultNotFound)
    }
}

impl<V: Default> TryDefault<V> for V {
    fn try_default() -> Result<V, DefaultNotFound> {
        Ok(V::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default, PartialEq, Debug)]
    struct IsDefault {
        a: u32,
    }

    #[derive(PartialEq, Debug)]
    struct NonDefault {
        a: u32,
    }

    #[test]
    fn it_should_default_when_used_on_core_type() {
        let n = <u32>::try_default();
        assert_eq!(n, Ok(u32::default()));
    }

    #[test]
    fn it_should_default_when_used_on_core_type_with_different_sytax() {
        let n = <u32 as TryDefault<u32>>::try_default();
        assert_eq!(n, Ok(u32::default()));
    }

    #[test]
    fn it_should_return_none_when_used_on_core_without_default() {
        let f = <::std::fs::File>::try_default();
        assert!(f.is_err());
    }

    #[test]
    fn it_returns_default_on_own_types() {
        let n = <IsDefault>::try_default();
        assert_eq!(n, Ok(IsDefault::default()));
    }

    #[test]
    fn it_returns_none_on_non_default_types() {
        let n = <NonDefault>::try_default();
        assert_eq!(n, Err(DefaultNotFound));
    }
}
