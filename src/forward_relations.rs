mod FooModule {
    fn foo() {
        fn bar() -> i32 {
            pub const BAR_CONST: u32 = 42;
            BAR_CONST
        }

        println!("{bar()}");
    }
}

struct FooTuple(i32);

union FooUnion {a: i32};

pub const FOO_CONST: u32 = 42;

trait FooTrait {
    fn foo() {};
}

pub enum FooEnum {
    Foo,
    Bar {
        baz: u64,
    },
}
