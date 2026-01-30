// use pinocchio::error::ProgramError;
use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{parse_macro_input, Data, DeriveInput, Fields},
};

#[proc_macro_derive(InstructionAccounts, attributes(pincon))]
pub fn instruction_accounts(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut validations = Vec::new();
    let mut field_idents = Vec::new();

    if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            for field in fields.named {
                if let Some(field_name) = &field.ident {
                    field_idents.push(field_name.clone());

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
    }

    let expanded = quote! {
        impl <'view> core::convert::TryFrom<&'view [AccountView]> for #name <'view>  {
            type Error = ProgramError;

            fn try_from(accounts: &'view [AccountView]) -> Result<Self, Self::Error> {
                // The let-else pattern for destructuring
                let [#(#field_idents,)* ..] = accounts else {
                    return Err(ProgramError::NotEnoughAccountKeys);
                };

                let inst = Self {
                    #(#field_idents,)*
                };

                inst.check_constraints()?;

                Ok(inst)
            }
        }

        impl <'view> #name <'view> {
            pub fn check_constraints(&self) -> Result<(), ProgramError> {
                #(#validations)*
                Ok(())
            }
        }
    };

    expanded.into()
}
