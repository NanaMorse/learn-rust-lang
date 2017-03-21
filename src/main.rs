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
        // todo what's the meaning for using panic?
        fn diverges() {
            panic!("This Function Never returns");
        }

        // function pointer
        let f: fn(i32) -> i32;
        f = plus_one;

        let seven = f(6);
    }

    // primitive types
    {
        // bool
        let x = true;
        let y: bool = false;

        // char
        let char_x = 'x';

        // several numbers

        // array
        let array_a = [1, 2, 3];
        let array_b = [12; 20];

        // get array length
        let array_a_length = array_a.len();

        // get array element
        let element_of_array = array_a[2];

        // todo slices

        // todo str

        // tuples
        let tuples_a = (1, "test");
        let tuples_b: (i32, char) = (2, 'c');

        let elem_a = tuples_a.0;
        let elem_b = tuples_a.1;

    }
}