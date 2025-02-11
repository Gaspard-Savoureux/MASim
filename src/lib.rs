/// Define the given actions as const with an array combining all of them.
///
/// For example:
/// The following:
/// ```rust
/// define_actions!(EAT: 0, SING: 1, DANCE: 2)
/// ```
/// Is equivalent to:
/// ```rust
/// pub const EAT: u32 = 0;
/// pub const SING: u32 = 1;
/// pub const DANCE: u32 = 2;
/// pub static ACTIONS: &[32] = &[EAT, SING, DANCE];
/// ```
#[macro_export]
macro_rules! define_actions {
    ($($action_name:ident: $value:expr),+) => {
        $(pub const $action_name: u32 = $value;)+
        pub static ACTIONS: &[u32] = &[$($action_name),*];
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_define_actions() {
        define_actions!(EAT: 0, SING: 1, DANCE: 2);

        assert_eq!(EAT, 0);
        assert_eq!(SING, 1);
        assert_eq!(DANCE, 2);
        assert_eq!(ACTIONS, &[0, 1, 2])
    }
}
