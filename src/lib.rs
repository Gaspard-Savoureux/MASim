/// Define the given actions as const with an array combining all of them.
///
/// For example:
/// The following:
/// ```rust
/// use masim::define_const;
/// define_const!(ACTIONS => EAT, SING, DANCE);
/// ```
/// which is equivalent to:
/// ```rust
/// pub const EAT: u32 = 0;
/// pub const SING: u32 = 1;
/// pub const DANCE: u32 = 2;
/// pub static ACTIONS: &[u32] = &[EAT, SING, DANCE];
/// ```
///
/// ---
///
/// Can also be used like this:
/// ```rust
/// use masim::define_const;
/// define_const!(MAN_ACTIONS: &'static str => MOVE_UP: "moving up", MOVE_DOWN: "moving down");
/// ```
/// which is equivalent to:
/// ```rust
/// pub const MOVE_UP: &'static str = "moving up";
/// pub const MOVE_DOWN: &'static str = "moving down";
/// pub static ACTIONS: &[&'static str] = &[MOVE_UP, MOVE_DOWN];
/// ```
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

    // Define with specific value
    ($collection_name:ident: $type_of:ty => $($const_name:ident: $value:expr),+) => {
        $(pub const $const_name: $type_of = $value;)+
        pub static $collection_name: &[$type_of] = &[$($const_name),*];
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
        assert_eq!(ACTIONS, &[0, 1, 2]);

        define_const!(MAN_ACTIONS: &'static str => MOVE_UP: "moving up", MOVE_DOWN: "moving down");
        assert_eq!(MOVE_UP, "moving up");
        assert_eq!(MOVE_DOWN, "moving down");
        assert_eq!(MAN_ACTIONS, &["moving up", "moving down"]);
    }
}
