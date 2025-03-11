extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};
use convert_case::{Casing, Case};

fn get_camel_ident(input: Ident) -> Ident {
    let camel_name = input.to_string().to_case(Case::UpperCamel);
    syn::Ident::new(&camel_name, input.span())
}

#[proc_macro]
pub fn make_camel(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Ident);

    let camel_ident = get_camel_ident(input);

    TokenStream::from(quote! {
        #camel_ident
    })
}

#[proc_macro]
pub fn create_crc_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Ident);
    let const_name = input.to_string();

    // Extract the CRC number from the constant name
    // Assuming the name format is 'CRC_XX_NAME', where XX is the CRC number
    let mut parts = const_name.split('_');
    let crc_part = parts.nth(1).expect("Invalid constant name format");
    let crc_num = crc_part.parse::<u32>().expect("Invalid CRC number");

    // Determine the type based on the CRC number
    let type_size = if crc_num <= 8 {
        "u8"
    } else if crc_num <= 16 {
        "u16"
    } else if crc_num <= 32 {
        "u32"
    } else if crc_num <= 64 {
        "u64"
    } else if crc_num <= 128 {
        "u128"
    } else {
        panic!("CRC number is too large")
    };

    // Create identifiers for the struct and type
    let struct_name = &input.clone();
    let span = input.span();
    let camel_name = get_camel_ident(input);
    let type_ident = syn::Ident::new(type_size, span);

    // Generate the code
    let expanded = quote! {
        struct #camel_name;

        impl Hasher for #camel_name {
            fn hash_name(&self) -> &'static str {
                stringify!(#struct_name)
            }

            fn active_bits(&self) -> &'static u32 {
                &#crc_num
            }

            fn hash(&self, data: &[u8]) -> Vec<u8> {
                let crc = Crc::<#type_ident>::new(&crc::#struct_name);
                crc.checksum(data).to_be_bytes().to_vec()
            }
        }
    };

    TokenStream::from(expanded)
}
