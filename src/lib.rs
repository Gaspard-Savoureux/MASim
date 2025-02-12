/// Define the given actions as const with an array combining all of them.
///
/// For example:
/// The following:
/// ```rust
/// define_actions!(EAT, SING, DANCE)
/// ```
/// Is equivalent to:
/// ```rust
/// pub const EAT: u32 = 0;
/// pub const SING: u32 = 1;
/// pub const DANCE: u32 = 2;
/// pub static ACTIONS: &[32] = &[EAT, SING, DANCE];
/// ```
///
/// NOTE: If interested this was a pretty good read for incremental TT munchers: https://danielkeep.github.io/tlborm/book/pat-incremental-tt-munchers.html
#[macro_export]
macro_rules! define_actions {
    // end of recursion
    ($i:expr ; ) => {};

    // action name remaining
    ($i:expr ; $action_name:ident $(, $tail:ident)*) => {
        pub const $action_name: u32 = $i;
        define_actions!($i + 1; $($tail),*);
    };

    // entry point
    ( $($action_name:ident),+ ) => {
        define_actions!(0; $($action_name),*);
        pub static ACTIONS: &[u32] = &[$($action_name),+];
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_define_actions() {
        define_actions!(EAT, SING, DANCE);

        assert_eq!(EAT, 0);
        assert_eq!(SING, 1);
        assert_eq!(DANCE, 2);
        assert_eq!(ACTIONS, &[0, 1, 2])
    }
}
