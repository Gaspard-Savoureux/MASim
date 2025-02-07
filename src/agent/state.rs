#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    /// ## Example
    /// ```rust
    /// let val: Value = 10_i32.into();
    /// let result: i32 = val.eq_type();
    /// assert_eq(10, result); // ok
    /// ```
    VI32(i32),
    /// ## Example
    /// ```rust
    /// let val: Value = 10_u32.into();
    /// let result: u32 = val.eq_type();
    /// assert_eq(10, result); // ok
    /// ```
    VU32(u32),
    /// ## Example
    /// ```rust
    /// let val: Value = 0.2_f32.into();
    /// let result: f32 = val.eq_type();
    /// assert_eq(0.2, result); // ok
    /// ```
    VFloat(u32),

    /// ## Example
    /// ```rust
    /// let val: Value = "Rust".into();
    /// let result: &'static str = val.eq_type();
    /// assert_eq("Rust", result); // ok
    /// ```
    VString(&'static str),

    /// ## Example
    /// ```rust
    /// let val: Value = true.into();
    /// let result: bool = val.eq_type();
    /// assert_eq(true, result); // ok
    /// ```
    VBool(bool),
}

pub trait ValueTyped: Sized {
    fn from_value(value: &Value) -> Self;
}

// For i32
impl ValueTyped for i32 {
    fn from_value(value: &Value) -> Self {
        match value {
            Value::VI32(x) => *x,
            other => panic!("Expected VI32, but got: {:?}", other),
        }
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::VI32(value)
    }
}

// For u32
impl ValueTyped for u32 {
    fn from_value(value: &Value) -> Self {
        match value {
            Value::VU32(x) => *x,
            other => panic!("Expected VU32, but got: {:?}", other),
        }
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::VU32(value)
    }
}

// For f32
impl ValueTyped for f32 {
    fn from_value(value: &Value) -> Self {
        match value {
            Value::VFloat(x) => f32::from_bits(*x),
            other => panic!("Expected VU32, but got: {:?}", other),
        }
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::VFloat(value.to_bits() as u32)
    }
}

// For bool
impl ValueTyped for bool {
    fn from_value(value: &Value) -> Self {
        match value {
            Value::VBool(b) => *b,
            other => panic!("Expected VBool, but got: {:?}", other),
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::VBool(value)
    }
}

// For String
impl ValueTyped for &'static str {
    fn from_value(value: &Value) -> Self {
        match value {
            Value::VString(s) => &s,
            other => panic!("Expected VString, but got: {:?}", other),
        }
    }
}

impl From<&'static str> for Value {
    fn from(value: &'static str) -> Self {
        Value::VString(value)
    }
}

impl Value {
    pub fn eq_type<T>(&self) -> T
    where
        T: ValueTyped,
    {
        T::from_value(self)
    }
}

pub fn to_value<T>(value: T) -> Value
where
    T: Into<Value>,
{
    value.into()
}

pub type State = Vec<Value>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i32() {
        let val: Value = 1_i32.into();
        let result: i32 = val.eq_type();
        assert_eq!(1, result);
    }

    #[test]
    fn test_u32() {
        let val: Value = 1_u32.into();
        let result: u32 = val.eq_type();
        assert_eq!(1, result);
    }

    #[test]
    fn test_f32() {
        let val: Value = 1.0_f32.into();
        let result: f32 = val.eq_type();
        assert_eq!(1., result);

        let val: Value = 0.2_f32.into();
        let result: f32 = val.eq_type();
        assert_eq!(0.2, result);
    }

    #[test]
    fn test_string() {
        let val: Value = "Rust".into();
        let result: &'static str = val.eq_type();
        assert_eq!("Rust", result);
    }

    #[test]
    fn test_bool() {
        let val: Value = true.into();
        let result: bool = val.eq_type();
        assert_eq!(true, result);
    }
}
