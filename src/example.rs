mod MyModule {
    fn my_fn() -> i32 {
        fn my_nested_fn() -> i32 {
            pub const MY_NESTED_CONST: u32 = 42;

            MY_NESTED_CONST
        }

        my_nested_fn()
    }
}

struct MyTupleStruct(i32);

union MyUnion {a: i32};

pub const MY_CONST: u32 = 42;

trait MyTrait {
    fn my_trait_fn() {};
}
