# `inline-gdscript`

Write GDScript code directly in your Rust code. Based on the amazing [`inline-python`](https://github.com/fusion-engineering/inline-python).

## Example

```rust
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
```

## Using Rust variables

To use Rust variables within a `gdscript!` macro, use the "lifetime"/"label" syntax `'name`.

Any variable that gets passed between Rust and `gdscript!` must implement `OwnedToVariant`.

# License

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be licensed under the [MIT license](LICENSE.md), without any additional terms or conditions.