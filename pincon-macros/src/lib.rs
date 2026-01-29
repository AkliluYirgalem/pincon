use pinocchio::error::ProgramError;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(InstructionAccounts, attributes(pincon))]
pub fn instruction_accounts(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut validations = Vec::new();

    if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            println!("{:#?}", fields.named);
            for field in fields.named {
                let field_name = &field.ident;

                // Check attributes for #[pincon(signer)]
                for attr in &field.attrs {
                    if attr.path().is_ident("pincon") {
                        let _ = attr.parse_nested_meta(|meta| {
                            if meta.path.is_ident("signer") {
                                validations.push(quote! {
                                    if !self.#field_name.is_signer() {
                                        return Err(ProgramError::MissingRequiredSignature);
                                    }
                                });
                            } else if meta.path.is_ident("mut") {
                                validations.push(quote! {
                                    if !self.#field_name.is_writable() {
                                        return Err(ProgramError::Immutable);
                                    }
                                });
                            }

                            Ok(())
                        });
                    }
                }
            }
        }
    }

    let expanded = quote! {
        impl #name {
            pub fn check_constraints(&self) -> ProgramResult {
                #(#validations)*
                Ok(())
            }
        }
    };

    expanded.into()
}
