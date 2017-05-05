extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate regex;

use proc_macro::TokenStream;
use regex::Regex;

struct RestrictedStringError(());

#[proc_macro_derive(RestrictedString, attributes(RestrictedStringRegex))]
pub fn restricted_string(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_restricted_string(&ast);
    
    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_restricted_string(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    let input_regex = &ast.attrs[0];
    quote! {
        type Err = RestrictedStringError
        impl FromStr for #name {
            fn from_str(s: &str) -> Result<#name, Self::Err> {
                lazy_static! {
                    static ref re: Regex = Regex::new(#input_regex).unwrap();
                }
            }
            if regex.is_match(s) {
                Ok(#name::new(s.to_owned()))
            } 
            else {
                Err(Self::Err)
            }
        }
    }
}


#[cfg(test)]
mod tests {

    #[derive(RestrictedString)]
    #[RestrictedStringRegex = ".*"]
    struct AnyString {
        s: String
    }

    impl AnyString {
        pub fn new(string: String) -> AnyString {
            AnyString {
                s: string
            }
        }
    }
    
    #[test]
    fn it_works() {
    }
}
