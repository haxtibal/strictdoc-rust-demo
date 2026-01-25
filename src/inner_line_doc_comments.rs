//! Top-level module
//! @relation(RUST-INNER-LINE-DOC, RUST-INNER-DOC-POS-FN)

#![feature(extern_types, trait_alias)]

pub trait Processor {
    type Output;
    const MAX_SIZE: usize;

    fn process(&self, input: &str) -> Self::Output;

    fn validate(&self) -> bool {
        //! Default method with
        //! @relation(RUST-INNER-LINE-DOC, RUST-INNER-DOC-POS-FN)
        //! Description: validation check routine

        true
    }
}

pub trait ProcessorClone = Processor + Clone;

pub struct Container {
    pub name: String,
    value: i32,
}

impl Processor for Container {
    type Output = String;
    const MAX_SIZE: usize = 1024;

    fn process(&self, input: &str) -> Self::Output {
        //! Impl method with
        //! @relation(RUST-INNER-LINE-DOC, RUST-INNER-DOC-POS-FN)
        //! Description: implementation of a process

        format!("{}: {}", self.name, input)
    }
}

impl Container {
    //! Inherent impl with
    //! @relation(RUST-INNER-LINE-DOC, RUST-INNER-DOC-POS-FN)
    //! Text: inherent method block

    pub fn new(name: String) -> Self {
        //! Inherent method with
        //! @relation(RUST-INNER-LINE-DOC)
        //! Words: constructor function pattern

        Self { name, value: 0 }
    }

    pub fn get_value(&self) -> i32 {
        //! Another method with
        //! @relation(RUST-INNER-LINE-DOC, RUST-INNER-DOC-POS-FN)
        //! Random: getter accessor method

        self.value
    }
}

pub fn process_data(input: &str) -> String {
    //! Function with
    //! @relation(RUST-INNER-LINE-DOC, RUST-INNER-DOC-POS-FN)
    //! Description: top-level function utility

    input.to_uppercase()
}

pub async fn async_process(data: Vec<u8>) -> Result<(), std::io::Error> {
    //! Async function with
    //! @relation(RUST-INNER-LINE-DOC, RUST-INNER-DOC-POS-FN)
    //! Random: asynchronous operation handler

    Ok(())
}

pub const fn compute_magic(x: u32) -> u32 {
    //! Const function with
    //! @relation(RUST-INNER-LINE-DOC, RUST-INNER-DOC-POS-FN)
    //! Text: compile-time evaluable function

    x * 42
}

pub unsafe fn dangerous_operation(ptr: *mut u8) {
    //! Unsafe function with
    //! @relation(RUST-INNER-LINE-DOC, RUST-INNER-DOC-POS-FN)
    //! Words: unchecked operation wrapper

    if !ptr.is_null() {
        *ptr = 0;
    }
}

pub mod submodule {
    //! Module with
    //! @relation(RUST-INNER-LINE-DOC, RUST-INNER-DOC-POS-MOD)
    //! Description: nested module container

    pub struct Inner {
        data: Vec<u8>,
    }
}

extern "C" {
    //! Foreign function interface with
    //! @relation(RUST-INNER-LINE-DOC, RUST-INNER-DOC-POS-EXT)
    //! Text: external C interface block

    fn external_func(x: i32) -> i32;

    static EXTERNAL_VAR: i32;

    type OpaqueType;
}

mod tests {
    #[test]
    fn test_basic() {
        //! Test function with
        //! @relation(RUST-INNER-LINE-DOC, RUST-INNER-DOC-POS-FN)
        //! Random: unit test case definition

        assert_eq!(2 + 2, 4);
    }
}
