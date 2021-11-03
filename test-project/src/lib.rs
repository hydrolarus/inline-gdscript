use gdnative::prelude::*;
use inline_gdscript::{gdscript, Context};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct TestProject;

#[methods]
impl TestProject {
    fn new(_owner: &Node) -> Self {
        TestProject
    }

    #[export]
    fn _ready(&self, _owner: TRef<Node>) {
        gdscript! {
            print("hello world from", " GDScript")

            for c in "test":
                print(c)
        }

        let x: String = gdscript! {
            print("inline GDScript blocks can also return values")
            return "it can " + "indeed"
        };
        godot_print!("{}", x);

        let x = "can also go from Rust to GDScript";
        gdscript! {
            print('x)
            var x: Node = '_owner
            x.get_parent().print_tree()
        }

        let mut ctx: Context = gdscript! {
            func my_function() -> float:
                print("in my_function()")
                return sin(12.0)
        };
        godot_dbg!(ctx.call("my_function", &[]));
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<TestProject>();
}

godot_init!(init);
