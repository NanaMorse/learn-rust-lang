fn main() {
    // variable-bindings
    {
        let my_number = 0;
        let my_string = "hello string";

        // Patterns
        let (first_var, second_var) = ("first_var", 2);

        // Type annotations
        let integer_32_bit: i32 = 15;

        // Mutability
        let mut mutable_var = 1;
        mutable_var = 2;

        print!("{}", integer_32_bit);
    }

    // functions
    {
        // print number
        fn print_number(x: i32) {
            print!("x is: {}", x);
        }

        // get number
        fn plus_one(x: i32) -> i32 {
            x + 1
        }

        // bad practice
        fn use_return() -> i32 {
            return 12;
        }

        // Diverging functions
        fn diverges() {
            panic!("This Function Never returns");
        }

        // function pointer
        let f: fn(i32) -> i32;
        f = plus_one;

        let seven = f(6);
    }
}