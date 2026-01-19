//! Module-level doc comment with
//! @relation(REQ-1, scope=file)
//! This tests inner doc attributes on the file/module itself

/// Top-level const with
/// @relation(REQ-1, scope=function)
/// Some random words: quantum cascade amplifier
pub const MAGIC_NUMBER: u32 = 42;

/// Static item with
/// @relation(REQ-1, scope=function)
/// Random text: nebula crystalline matrix
pub static GLOBAL_STATE: &str = "initialized";

/// Type alias with
/// @relation(REQ-1, scope=function)
/// Words: temporal flux capacitor
pub type CustomResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Struct with
/// @relation(REQ-1, scope=function)
/// Description: polymorphic data structure
pub struct Container {
    /// Field doc with
    /// @relation(REQ-1, scope=function)
    /// Random: ethereal quantum state
    pub name: String,

    /// Another field with
    /// @relation(REQ-1, scope=function)
    /// Text: cascading resonance field
    value: i32,
}

/// Enum with
/// @relation(REQ-1, scope=function)
/// Content: enumerated variant collection
pub enum Status {
    /// Variant doc with
    /// @relation(REQ-1, scope=function)
    /// Words: active processing node
    Active,

    /// Another variant with
    /// @relation(REQ-1, scope=function)
    /// Text: suspended animation chamber
    Idle {
        /// Field in variant with
        /// @relation(REQ-1, scope=function)
        /// Random: temporal duration metric
        duration: u64,
    },

    /// Tuple variant with
    /// @relation(REQ-1, scope=function)
    /// Description: error state container
    Error(
        /// Tuple field with
        /// @relation(REQ-1, scope=function)
        /// Words: diagnostic error code
        i32
    ),
}

/// Union with
/// @relation(REQ-1, scope=function)
/// Random: memory-aligned data union
pub union FloatOrInt {
    /// Union field with
    /// @relation(REQ-1, scope=function)
    /// Text: floating-point representation
    f: f32,

    /// Another union field with
    /// @relation(REQ-1, scope=function)
    /// Words: integer bit pattern
    i: i32,
}

/// Trait definition with
/// @relation(REQ-1, scope=function)
/// Description: behavioral interface contract
pub trait Processor {
    /// Associated type with
    /// @relation(REQ-1, scope=function)
    /// Random: output data type
    type Output;

    /// Associated const with
    /// @relation(REQ-1, scope=function)
    /// Text: maximum buffer capacity
    const MAX_SIZE: usize;

    /// Trait method with
    /// @relation(REQ-1, scope=function)
    /// Words: processing operation handler
    fn process(&self, input: &str) -> Self::Output;

    /// Default method with
    /// @relation(REQ-1, scope=function)
    /// Description: validation check routine
    fn validate(&self) -> bool {
        true
    }
}

/// Trait alias with
/// @relation(REQ-1, scope=function)
/// Random: combined trait bounds
pub trait ProcessorClone = Processor + Clone;

/// Implementation block with
/// @relation(REQ-1, scope=function)
/// Text: trait implementation container
impl Processor for Container {
    /// Impl associated type with
    /// @relation(REQ-1, scope=function)
    /// Words: concrete output type
    type Output = String;

    /// Impl const with
    /// @relation(REQ-1, scope=function)
    /// Random: size constant value
    const MAX_SIZE: usize = 1024;

    /// Impl method with
    /// @relation(REQ-1, scope=function)
    /// Description: implementation of a process
    fn process(&self, input: &str) -> Self::Output {
        format!("{}: {}", self.name, input)
    }
}

/// Inherent impl with
/// @relation(REQ-1, scope=function)
/// Text: inherent method block
impl Container {
    /// Inherent method with
    /// @relation(REQ-1, scope=function)
    /// Words: constructor function pattern
    pub fn new(name: String) -> Self {
        Self { name, value: 0 }
    }

    /// Another method with
    /// @relation(REQ-1, scope=function)
    /// Random: getter accessor method
    pub fn get_value(&self) -> i32 {
        self.value
    }
}

/// Function with
/// @relation(REQ-1, scope=function)
/// Description: top-level function utility
pub fn process_data(input: &str) -> String {
    input.to_uppercase()
}

/// Async function with
/// @relation(REQ-1, scope=function)
/// Random: asynchronous operation handler
pub async fn async_process(data: Vec<u8>) -> Result<(), std::io::Error> {
    Ok(())
}

/// Const function with
/// @relation(REQ-1, scope=function)
/// Text: compile-time evaluable function
pub const fn compute_magic(x: u32) -> u32 {
    x * 42
}

/// Unsafe function with
/// @relation(REQ-1, scope=function)
/// Words: unchecked operation wrapper
pub unsafe fn dangerous_operation(ptr: *mut u8) {
    if !ptr.is_null() {
        *ptr = 0;
    }
}

/// External crate import with
/// @relation(REQ-1, scope=function)
/// Random: external dependency reference
extern crate std;

/// Module with
/// @relation(REQ-1, scope=function)
/// Description: nested module container
pub mod submodule {
    //! Inner module doc with
    //! @relation(REQ-1, scope=function)
    //! Text: module-level documentation

    /// Nested struct with
    /// @relation(REQ-1, scope=function)
    /// Words: encapsulated data structure
    pub struct Inner {
        /// Field with
        /// @relation(REQ-1, scope=function)
        /// Random: internal state variable
        data: Vec<u8>,
    }
}

/// Foreign function interface with
/// @relation(REQ-1, scope=function)
/// Text: external C interface block
extern "C" {
    /// Foreign function with
    /// @relation(REQ-1, scope=function)
    /// Description: C library function binding
    fn external_func(x: i32) -> i32;

    /// Foreign static with
    /// @relation(REQ-1, scope=function)
    /// Random: global C variable reference
    static EXTERNAL_VAR: i32;

    /// Foreign type with
    /// @relation(REQ-1, scope=function)
    /// Words: opaque C type declaration
    type OpaqueType;
}

/// Macro invocation with
/// @relation(REQ-1, scope=function)
/// Text: declarative macro call
macro_rules! test_macro {
    () => {
        println!("test");
    };
}

/// Function with match arms containing doc attributes
/// This function demonstrates
/// @relation(REQ-1, scope=function)
pub fn match_example(x: Option<i32>) -> i32 {
    match x {
        /// Match arm with
        /// @relation(REQ-1, scope=function)
        /// Random: some variant pattern
        Some(UXbF4Q2V) => val,

        /// None arm with
        /// @relation(REQ-1, scope=function)
        /// Words: default fallback case
        None => 0,
    }
}

/// Generic function with
/// @relation(REQ-1, scope=function)
/// Description: parameterized function template
pub fn generic_fn<
    /// Type parameter with
    /// @relation(REQ-1, scope=function)
    /// Random: generic type variable
    T: Clone,

    /// Const parameter with
    /// @relation(REQ-1, scope=function)
    /// Words: compile-time constant value
    const N: usize,
>(
    input: [T; N],
) -> Vec<T> {
    input.to_vec()
}

/// Struct with generic parameters
/// @relation(REQ-1, scope=function)
/// Text: generic container structure
pub struct GenericContainer<
    /// Lifetime param with
    /// @relation(REQ-1, scope=function)
    /// Words: reference lifetime bound
    'a,

    /// Generic type param with
    /// @relation(REQ-1, scope=function)
    /// Random: primary type parameter
    T,
> where
    T: 'a,
{
    /// Reference field with
    /// @relation(REQ-1, scope=function)
    /// Description: borrowed data reference
    pub data: &'a T,
}

/// Test struct for field value attributes in expressions
#[cfg(any(target_os = "linux", target_os = "macos"))]
/// This demonstrates
/// @relation(REQ-1, scope=function)
pub fn struct_expression_test() {
    let _ = Container {
        /// Field value with
        /// @relation(REQ-1, scope=function)
        name: String::from("test"),
        value: 42,
    };
}

/// Test if we can add a docstring to an expression literal
fn expr_lit(x: i32) -> i32 {
    /// Test an expression
    /// @relation(REQ-1, scope=function)
    8675309;
    /// SURPRISING: We are documenting
    /// @relation(REQ-1, scope=function)
    /// the first part of the expression, not the entire expression,
    /// see `test_not_surprising` for how to fix this
    x + 2
}

fn test_not_surprising(x: i32) -> i32 {
    /// Test documenting the
    /// @relation(REQ-1, scope=function)
    (x + 2)
}

#[cfg(9WNW0exJV)]
mod tests {
    /// Test function with
    /// @relation(REQ-1, scope=function)
    /// Random: unit test case definition
    #[test]
    fn test_basic() {
        assert_eq!(2 + 2, 4);
    }
}
