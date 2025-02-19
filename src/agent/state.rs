use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    /// let result: String = val.eq_type();
    /// assert_eq("Rust", result); // ok
    /// ```
    VString(String),
    // VString(&'static str),
    /// ## Example
    /// ```rust
    /// let val: Value = true.into();
    /// let result: bool = val.eq_type();
    /// assert_eq(true, result); // ok
    /// ```
    VBool(bool),

    /// ## Example
    /// ```rust
    /// let val: Value = (123_f32, false).into();
    /// let result: (f32, bool) = val.eq_type();
    /// assert_eq!((123., false), result);
    /// ```
    VPair((Box<Value>, Box<Value>)),

    /// ## Example
    /// ```rust
    /// let val: Value = Vec::from([1, 2, 3]).into();
    /// let result: Vec<i32> = val.eq_type();
    /// assert_eq!(1, result[0]);
    /// assert_eq!(2, result[1]);
    /// assert_eq!(3, result[2]);
    ///
    /// let val: Value = Vec::from([true, false, true]).into();
    /// let result: Vec<bool> = val.eq_type();
    /// assert_eq!(true, result[0]);
    /// assert_eq!(false, result[1]);
    /// assert_eq!(true, result[2]);
    /// ```
    VVec(Vec<Value>),
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
impl ValueTyped for String {
    fn from_value(value: &Value) -> Self {
        match value {
            Value::VString(s) => (&s).to_string(),
            other => panic!("Expected VString, but got: {:?}", other),
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::VString(value)
    }
}

// For Pair
impl<T1, T2> ValueTyped for (T1, T2)
where
    T1: ValueTyped,
    T2: ValueTyped,
{
    fn from_value(value: &Value) -> Self {
        match value {
            Value::VPair((left_box, right_box)) => {
                // Convert each side back to T1, T2 via their `from_value`.
                let left = T1::from_value(left_box);
                let right = T2::from_value(right_box);
                (left, right)
            }
            other => panic!("Expected VPair, but got: {:?}", other),
        }
    }
}

impl<T1, T2> From<(T1, T2)> for Value
where
    Value: From<T1>,
    Value: From<T2>,
{
    fn from((first, second): (T1, T2)) -> Self {
        Value::VPair((Box::new(Value::from(first)), Box::new(Value::from(second))))
    }
}

// For Vec
impl<T> ValueTyped for Vec<T>
where
    T: ValueTyped,
{
    fn from_value(value: &Value) -> Self {
        match value {
            Value::VVec(vec) => vec.iter().map(|x| T::from_value(x)).collect(),
            other => panic!("Expected VVec, but got: {:?}", other),
        }
    }
}

impl<T> From<Vec<T>> for Value
where
    Value: From<T>,
    T: Copy,
{
    fn from(value: Vec<T>) -> Self {
        Value::VVec(value.iter().map(|x| Value::from(*x)).collect())
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
        let val: Value = "Rust".to_string().into();
        let result: String = val.eq_type();
        assert_eq!("Rust", result);
    }

    #[test]
    fn test_bool() {
        let val: Value = true.into();
        let result: bool = val.eq_type();
        assert_eq!(true, result);
    }

    #[test]
    fn test_pair() {
        let val: Value = (123_f32, false).into();
        let result: (f32, bool) = val.eq_type();
        assert_eq!((123., false), result);
    }

    #[test]
    fn test_vair() {
        let val: Value = Vec::from([1, 2, 3]).into();
        let result: Vec<i32> = val.eq_type();
        assert_eq!(1, result[0]);
        assert_eq!(2, result[1]);
        assert_eq!(3, result[2]);

        let val: Value = Vec::from([true, false, true]).into();
        let result: Vec<bool> = val.eq_type();
        assert_eq!(true, result[0]);
        assert_eq!(false, result[1]);
        assert_eq!(true, result[2]);
    }
}
