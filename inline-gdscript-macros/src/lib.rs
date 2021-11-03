#![feature(proc_macro_span)]

extern crate proc_macro;

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;

use crate::embed_gdscript::EmbedGdscript;
use quote::quote;

mod embed_gdscript;

fn gdscript_impl(input: TokenStream) -> Result<TokenStream, TokenStream> {
    let mut x = EmbedGdscript::new();

    x.add(input)?;
    let source = x.gdscript;
    let variables = x.variables;

    let mut extra_source = String::new();
    for (var, _) in &variables {
        extra_source.push_str(&format!("var {}\n", var));
    }

    let indent = x.first_indent.unwrap_or(0);

    let varname = variables.keys();
    let var = variables.values();

    Ok(quote! {
        ::inline_gdscript::FromInlineGdscript::from_gdscript_macro(
            #source,
            #extra_source,
            #indent,
            |ctx| {
                #(
                    ctx.set(#varname, #var); // todo better error handling
                )*
            },
        )
    })
}

/// Embed GDScript code inside a Rust source file
///
/// When assigned to a variable of a type that implements `FromVariant`
/// then the code in the block will run immediately when it gets encountered
/// during execution.
///
/// When the macro stands as a "statement" without assigning to a variable
/// it will yield `()`.
///
/// ```rust,no_run
/// gdscript! {
///     var x = "one" if true else "two"
///     print(x)
/// }
/// ```
///
/// When returning a value from the macro, the `return` statement in GDScript
/// must be used, as the code gets wrapped in a function.
///
/// ```rust,no_run
/// let s: String = gdscript! {
///     if true:
///         return "one"
///     else:
///         return "two"
/// };
/// ```
///
/// When the result of the macro is bound to a variable of type `Context` then
/// execution will be suspended and `Context::call()` can be used to execute
/// functions. In this mode the code does **not** get wrapped in a function, so
/// the macro body will have to include GDScript function definitions.
///
/// ```rust,no_run
/// let mut c: Context = gdscript! {
///     func factorial(n: int) -> int:
///         if n == 0:
///             return 1
///         return n * factorial(n - 1)
/// };
/// dbg!(c.call("factorial", &[12.to_variant()]));
/// ```
///
/// Variables from Rust can be used inside the macro by prefixing them with `'`.
/// The variables need to be of a type that implements `OwnedToVariant`.
///
/// ```rust,no_run
/// let s = "some string";
/// let x = Vector2::new(2.0, 4.5);
/// gdscript! {
///     var x_norm = 'x.normalized()
///     print('s)
/// }
/// ```
#[proc_macro]
pub fn gdscript(toks: TokenStream1) -> TokenStream1 {
    TokenStream1::from(match gdscript_impl(TokenStream::from(toks)) {
        Ok(tokens) => tokens,
        Err(tokens) => tokens,
    })
}
