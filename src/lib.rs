pub use inline_gdscript_macros::gdscript;

use gdnative_core::{
    core_types::{FromVariant, OwnedToVariant, ToVariant, Variant},
    libc::c_char,
};

/// Execution context for a GDScript instance
pub struct Context(Variant);

impl Context {
    fn new_with_source(src: &str) -> Self {
        // In order to not pull in the all the bindings and possibly risk version
        // conflicts the sys API for getting ClassDB is used.
        // ClassDB allows to intance classes by name, also `GDScript`
        //
        // Everything goes through `Variant` for simplicity. If performance
        // becomes a concern then maybe cached methodbinds and ptrcalls should be used.
        let api = gdnative_core::private::get_api();
        let mut class_db = unsafe {
            let class_db = (api.godot_global_get_singleton)(b"ClassDB\0".as_ptr() as *mut c_char);
            Variant::from_object_ptr(class_db)
        };
        let mut gdscript = class_db
            .call("instance", &["GDScript".to_variant()])
            .unwrap();

        gdscript
            .call("set", &["source_code".to_variant(), src.to_variant()])
            .unwrap();

        gdscript.call("reload", &[]).unwrap();

        let instance = gdscript.call("new", &[]).unwrap();
        Self(instance)
    }

    /// Call the method `name` with arguments `args`.
    ///
    /// # Panics
    ///
    /// This method panics when the function does not exist or there was
    /// an execution error.
    pub fn call(&mut self, name: &str, args: &[Variant]) -> Variant {
        self.0.call(name, args).unwrap()
    }

    /// Set the value of a variable/property.
    ///
    /// # Panics
    ///
    /// This method panics when the variables does not exist or there was
    /// an execution error (for example in the setter).
    pub fn set(&mut self, var_name: &str, val: impl OwnedToVariant) {
        self.0
            .call("set", &[var_name.to_variant(), val.owned_to_variant()])
            .unwrap();
    }

    /// Get the value of a variable/property.
    ///
    /// # Panics
    ///
    /// This method panics when the variables does not exist, there was
    /// an execution error (for example in the getter) or the conversion from
    /// `Variant` to `T` failed.
    pub fn get<T: FromVariant>(&mut self, var_name: &str) -> T {
        let val = self.0.call("get", &[var_name.to_variant()]).unwrap();
        T::from_variant(&val).unwrap()
    }
}

#[doc(hidden)]
pub trait FromInlineGdscript<F: FnOnce(&mut Context)> {
    fn from_gdscript_macro(
        source: &'static str,
        extra_source: &'static str,
        indent: usize,
        set_variables: F,
    ) -> Self;
}

impl<T: FromVariant, F: FnOnce(&mut Context)> FromInlineGdscript<F> for T {
    fn from_gdscript_macro(
        source: &'static str,
        extra_source: &'static str,
        indent: usize,
        set_variables: F,
    ) -> Self {
        let mut new_src = String::new();

        let indent: String = " ".repeat(indent);

        for line in source.lines() {
            new_src.push_str(&indent);
            new_src.push_str(line);
            new_src.push('\n');
        }

        let mut ctx = Context::new_with_source(&format!(
            "extends Reference; func run():{}\n{}",
            new_src, extra_source
        ));
        set_variables(&mut ctx);
        T::from_variant(&ctx.call("run", &[])).unwrap()
    }
}

impl<F: FnOnce(&mut Context)> FromInlineGdscript<F> for Context {
    fn from_gdscript_macro(
        source: &'static str,
        extra_source: &'static str,
        _indent: usize,
        set_variables: F,
    ) -> Self {
        let mut ctx =
            Context::new_with_source(&format!("extends Reference;{}\n{}", source, extra_source));
        set_variables(&mut ctx);
        ctx
    }
}
