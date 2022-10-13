mod inline_module {
    fn _something_private() {} // prefixing with underscore supresses warning
    pub fn something_public() {}

    mod inline_submodule { // submodules are private by default
        fn _something_private() {}
        pub fn something_public() {}
    }

    /* Because submodule_publicity is defined in the same scope as
     * "inline_submodule", it has access to inline_submodule despite
     * "inline_submodule" not being public
     */
    pub fn submodule_publicity() {
        inline_submodule::something_public();
    }

    /* Fields of a struct are private by default */
    pub struct MyStruct {
        pub public: i32,
        private: i32,
    }

    pub fn get_struct(x: i32) -> MyStruct { MyStruct{ public: x, private: x } }

    /* function defined in the same scope as access to private field in a struct */
    pub fn print_struct(s: &MyStruct) {
        println!("<MyStruct public={} private={}>", s.public, s.private)
    }

    /* Variants of a public enum are all public */
    pub enum Enum { Var1, Var2 }
}

/* the module is declared here, but its content is stored in file_module.rs */
mod file_module;

fn print_enum(e: &inline_module::Enum) {
    match e {
        inline_module::Enum::Var1 => println!("variant 1"),
        inline_module::Enum::Var2 => println!("variant 2"),
    }
}
                                              

fn main() {
    /* Because the main function is defined in the same scope as
     * "inline_module", it has access to the module (but not necessarily its
     * children) despite the module being private
     */
    inline_module::something_public();
    inline_module::submodule_publicity();
    // inline_module::something_private(); // does not work
    // inline_module::inline_submodule::something_public();  // does not work

    file_module::something_public();

    let my_struct: inline_module::MyStruct = inline_module::get_struct(69);
    println!("{}", my_struct.public);
    // println!("{}", my_struct.private); // does not work
    inline_module::print_struct(&my_struct);

    /* By convention, functions are brought into scope using the parent module
     * structs and enums are broguth into scope explicitly
     */
    use inline_module::Enum;
    let e1: Enum = Enum::Var1;
    let e2: Enum = Enum::Var2;
    print_enum(&e1); print_enum(&e2);

    /* When there are name collisions with importing, use the "as" keyword */
    use inline_module::Enum as OtherEnum;
    let other: Enum = OtherEnum::Var1;
    print_enum(&other);
}

