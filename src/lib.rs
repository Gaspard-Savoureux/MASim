/// Define the given actions as const with an array combining all of them.
///
/// For example:
/// The following:
/// ```rust
/// define_const!(ACTIONS => EAT, SING, DANCE)
/// ```
/// is equivalent to:
/// ```rust
/// pub const EAT: u32 = 0;
/// pub const SING: u32 = 1;
/// pub const DANCE: u32 = 2;
/// pub static ACTIONS: &[32] = &[EAT, SING, DANCE];
/// ```
///
/// NOTE: This was a pretty good read for incremental TT munchers: https://danielkeep.github.io/tlborm/book/pat-incremental-tt-munchers.html
#[macro_export]
macro_rules! define_const {
    // end of recursion
    ($i:expr ; ) => {};

    // action name remaining
    ($i:expr ; $const_name:ident $(, $tail:ident)*) => {
        pub const $const_name: u32 = $i;
        define_const!($i + 1; $($tail),*);
    };

    // entry point
    ( $collection_name:ident => $($const_name:ident),+ ) => {
        define_const!(0; $($const_name),*);
        pub static $collection_name: &[u32] = &[$($const_name),+];
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_define_const() {
        define_const!(ACTIONS => EAT, SING, DANCE);

        assert_eq!(EAT, 0);
        assert_eq!(SING, 1);
        assert_eq!(DANCE, 2);
        assert_eq!(ACTIONS, &[0, 1, 2])
    }
}
