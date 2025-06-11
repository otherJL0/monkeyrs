use proc_macro::TokenStream;
use syn::Data::{Enum, Struct, Union};
use syn::parse_macro_input;
use syn::{DataEnum, DataStruct, DataUnion, DeriveInput};

pub fn derive_proc_macro_impl(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);
    let description_str = match data {
        Struct(my_struct) => gen_struct_description_str(my_struct),
        Enum(my_enum) => gen_enum_description_str(my_enum),
        Union(my_union) => gen_union_description_str(my_union),
    };
    quote! {}
}

fn gen_struct_description_str(my_struct: DataStruct) -> String {}
fn gen_enum_description_str(my_enum: DataEnum) -> String {}
fn gen_union_description_str(my_union: DataUnion) -> String {}
