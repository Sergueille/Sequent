use proc_macro::TokenStream;

/// For each field f, create a function f() to get a reference to the fields, and a function set_f() to set the field and save settings 
#[proc_macro_derive(SettingsMacro)]
pub fn derive_custom_model(item: TokenStream) -> TokenStream {
    
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    match input.data {
        syn::Data::Struct(s) => {

            let struct_name = input.ident;

            let field_names: Vec<syn::Ident> = s.fields.iter().map(|f| f.ident.clone().unwrap()).collect();
            let field_types: Vec<syn::Type> = s.fields.iter().map(|f| f.ty.clone()).collect();
            let set_field_names: Vec<syn::Ident> = field_names.iter().map(|i| syn::Ident::new(&format!("set_{}", i), i.span())).collect();

            quote::quote!{
                impl #struct_name {
                    #(
                        pub fn #field_names(&self) -> &#field_types {
                            return &self.#field_names;
                        }

                        pub fn #set_field_names(&mut self, new_value: #field_types) {
                            self.#field_names = new_value;

                            match save_settings(&self) {
                                Ok(()) => { }
                                Err(e) => { 
                                    println!("{}", e.message);
                                }
                            }
                        }
                    )*
                }
            }
        }
        _ => unimplemented!()
    }.into()

}
