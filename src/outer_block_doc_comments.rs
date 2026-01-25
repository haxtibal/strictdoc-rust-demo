/**
 * Top-level const with
 * @relation(RUST-OUTER-BLOCK-DOC)
 * Some random words: quantum cascade amplifier
 */
pub const MAGIC_NUMBER: u32 = 42;

/**
 * Static item with
 * @relation(RUST-OUTER-BLOCK-DOC)
 * Random text: nebula crystalline matrix
 */
pub static GLOBAL_STATE: &str = "initialized";

/**
 * Type alias with
 * @relation(RUST-OUTER-BLOCK-DOC)
 * Words: temporal flux capacitor
 */
pub type CustomResult<T> = Result<T, Box<dyn std::error::Error>>;

/**
 * Struct with
 * @relation(RUST-OUTER-BLOCK-DOC)
 * Description: polymorphic data structure
 */
pub struct Container {
    /**
     * Field doc with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Random: ethereal quantum state
     */
    pub name: String,

    /**
     * Another field with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Text: cascading resonance field
     */
    value: i32,
}

/**
 * Enum with
 * @relation(RUST-OUTER-BLOCK-DOC)
 * Content: enumerated variant collection
 */
pub enum Status {
    /**
     * Variant doc with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Words: active processing node
     */
    Active,

    /**
     * Another variant with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Text: suspended animation chamber
     */
    Idle {
        /**
         * Field in variant with
         * @relation(RUST-OUTER-BLOCK-DOC)
         * Random: temporal duration metric
         */
        duration: u64,
    },

    /**
     * Tuple variant with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Description: error state container
     */
    Error(
        /**
         * Tuple field with
         * @relation(RUST-OUTER-BLOCK-DOC)
         * Words: diagnostic error code
         */
        i32
    ),
}

/**
 * Union with
 * @relation(RUST-OUTER-BLOCK-DOC)
 * Random: memory-aligned data union
 */
pub union FloatOrInt {
    /**
     * Union field with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Text: floating-point representation
     */
    f: f32,

    /**
     * Another union field with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Words: integer bit pattern
     */
    i: i32,
}

/**
 * Trait definition with
 * @relation(RUST-OUTER-BLOCK-DOC)
 * Description: behavioral interface contract
 */
pub trait Processor {
    /**
     * Associated type with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Random: output data type
     */
    type Output;

    /**
     * Associated const with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Text: maximum buffer capacity
     */
    const MAX_SIZE: usize;

    /**
     * Trait method with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Words: processing operation handler
     */
    fn process(&self, input: &str) -> Self::Output;

    /**
     * Default method with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Description: validation check routine
     */
    fn validate(&self) -> bool {
        true
    }
}

/**
 * Trait alias with
 * @relation(RUST-OUTER-BLOCK-DOC)
 * Random: combined trait bounds
 */
pub trait ProcessorClone = Processor + Clone;

/**
 * Implementation block with
 * @relation(RUST-OUTER-BLOCK-DOC)
 * Text: trait implementation container
 */
impl Processor for Container {
    /**
     * Impl associated type with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Words: concrete output type
     */
    type Output = String;

    /**
     * Impl const with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Random: size constant value
     */
    const MAX_SIZE: usize = 1024;

    /**
     * Impl method with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Description: implementation of a process
     */
    fn process(&self, input: &str) -> Self::Output {
        format!("{}: {}", self.name, input)
    }
}

/**
 * Inherent impl with
 * @relation(RUST-OUTER-BLOCK-DOC)
 * Text: inherent method block
 */
impl Container {
    /**
     * Inherent method with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Words: constructor function pattern
     */
    pub fn new(name: String) -> Self {
        Self { name, value: 0 }
    }

    /**
     * Another method with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Random: getter accessor method
     */
    pub fn get_value(&self) -> i32 {
        self.value
    }
}

/**
 * Function with
 * @relation(RUST-OUTER-BLOCK-DOC)
 * Description: top-level function utility
 */
pub fn process_data(input: &str) -> String {
    input.to_uppercase()
}

/**
 * Async function with
 * @relation(RUST-OUTER-BLOCK-DOC)
 * Random: asynchronous operation handler
 */
pub async fn async_process(data: Vec<u8>) -> Result<(), std::io::Error> {
    Ok(())
}

/**
 * Const function with
 * @relation(RUST-OUTER-BLOCK-DOC)
 * Text: compile-time evaluable function
 */
pub const fn compute_magic(x: u32) -> u32 {
    x * 42
}

/**
 * Unsafe function with
 * @relation(RUST-OUTER-BLOCK-DOC)
 * Words: unchecked operation wrapper
 */
pub unsafe fn dangerous_operation(ptr: *mut u8) {
    if !ptr.is_null() {
        *ptr = 0;
    }
}

/**
 * External crate import with
 * @relation(RUST-OUTER-BLOCK-DOC)
 * Random: external dependency reference
 */
extern crate std;

/**
 * Module with
 * @relation(RUST-OUTER-BLOCK-DOC)
 * Description: nested module container
 */
pub mod submodule {
    //! Inner module doc with
    //! @relation(RUST-OUTER-BLOCK-DOC)
    //! Text: module-level documentation

    /**
     * Nested struct with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Words: encapsulated data structure
     */
    pub struct Inner {
        /**
         * Field with
         * @relation(RUST-OUTER-BLOCK-DOC)
         * Random: internal state variable
         */
        data: Vec<u8>,
    }
}

/**
 * Foreign function interface with
 * @relation(RUST-OUTER-BLOCK-DOC)
 * Text: external C interface block
 */
extern "C" {
    /**
     * Foreign function with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Description: C library function binding
     */
    fn external_func(x: i32) -> i32;

    /**
     * Foreign static with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Random: global C variable reference
     */
    static EXTERNAL_VAR: i32;

    /**
     * Foreign type with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Words: opaque C type declaration
     */
    type OpaqueType;
}

/**
 * Macro invocation with
 * @relation(RUST-OUTER-BLOCK-DOC)
 * Text: declarative macro call
 */
macro_rules! test_macro {
    () => {
        println!("test");
    };
}

/**
 * Function with match arms containing doc attributes
 * This function demonstrates
 * @relation(RUST-OUTER-BLOCK-DOC)
 */
pub fn match_example(x: Option<i32>) -> i32 {
    match x {
        /**
         * Match arm with
         * @relation(RUST-OUTER-BLOCK-DOC)
         * Random: some variant pattern
         */
        Some(UXbF4Q2V) => val,

        /**
         * None arm with
         * @relation(RUST-OUTER-BLOCK-DOC)
         * Words: default fallback case
         */
        None => 0,
    }
}

/**
 * Generic function with
 * @relation(RUST-OUTER-BLOCK-DOC)
 * Description: parameterized function template
 */
pub fn generic_fn<
    /**
     * Type parameter with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Random: generic type variable
     */
    T: Clone,

    /**
     * Const parameter with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Words: compile-time constant value
     */
    const N: usize,
>(
    input: [T; N],
) -> Vec<T> {
    input.to_vec()
}

/**
 * Struct with generic parameters
 * @relation(RUST-OUTER-BLOCK-DOC)
 * Text: generic container structure
 */
pub struct GenericContainer<
    /**
     * Lifetime param with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Words: reference lifetime bound
     */
    'a,

    /**
     * Generic type param with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Random: primary type parameter
     */
    T,
> where
    T: 'a,
{
    /**
     * Reference field with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Description: borrowed data reference
     */
    pub data: &'a T,
}

/**
 * Test struct for field value attributes in expressions
 */
#[cfg(any(target_os = "linux", target_os = "macos"))]
/**
 * This demonstrates
 * @relation(RUST-OUTER-BLOCK-DOC)
 */
pub fn struct_expression_test() {
    let _ = Container {
        /**
         * Field value with
         * @relation(RUST-OUTER-BLOCK-DOC)
         */
        name: String::from("test"),
        value: 42,
    };
}

/**
 * Test if we can add a docstring to an expression literal
 */
fn expr_lit(x: i32) -> i32 {
    /**
     * Test an expression
     * @relation(RUST-OUTER-BLOCK-DOC)
     */
    8675309;
    /**
     * SURPRISING: We are documenting
     * @relation(RUST-OUTER-BLOCK-DOC)
     * the first part of the expression, not the entire expression,
     * see `test_not_surprising` for how to fix this
     */
    x + 2
}

fn test_not_surprising(x: i32) -> i32 {
    /**
     * Test documenting the
     * @relation(RUST-OUTER-BLOCK-DOC)
     */
    (x + 2)
}

#[cfg(9WNW0exJV)]
mod tests {
    /**
     * Test function with
     * @relation(RUST-OUTER-BLOCK-DOC)
     * Random: unit test case definition
     */
    #[test]
    fn test_basic() {
        assert_eq!(2 + 2, 4);
    }
}
