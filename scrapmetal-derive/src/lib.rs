#![recursion_limit = "1000"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::Tokens;

#[proc_macro_derive(Term)]
pub fn derive_into_heap(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_derive_input(&source).unwrap();
    let expanded = impl_term(&ast);

    // Uncomment to debug the generated code...
    // println!("\n\n{:?}", expanded);

    expanded.parse().unwrap()
}

fn impl_term(ast: &syn::DeriveInput) -> Tokens {
    match ast.body {
        syn::Body::Struct(ref data) => impl_term_for_struct(ast, data),
        syn::Body::Enum(ref variants) => impl_term_for_enum(ast, variants),
    }
}

fn impl_term_for_struct(ast: &syn::DeriveInput, data: &syn::VariantData) -> Tokens {
    match *data {
        syn::VariantData::Struct(ref fields) => impl_term_for_struct_struct(ast, fields),
        syn::VariantData::Tuple(ref fields) => impl_term_for_tuple_struct(ast, fields),
        syn::VariantData::Unit => impl_term_for_unit_struct(ast),
    }
}

fn impl_term_for_struct_struct(ast: &syn::DeriveInput, fields: &[syn::Field]) -> Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let transforms: Vec<_> = fields.iter()
        .map(|f| {
            quote! {
                #f.ident : f.transform(self.#f.ident) ,
            }
        })
        .collect();

    let queries: Vec<_> = fields.iter()
        .map(|f| {
            quote! {
                let r = q.query(&self.#f.ident);
                each(q, r);
            }
        })
        .collect();

    let mutations: Vec<_> = fields.iter()
        .map(|f| {
            quote! {
                let r = m.mutate(&mut self.#f.indent);
                each(m, r);
            }
        })
        .collect();

    quote! {
        impl #impl_generics ::scrapmetal::Term for #name #ty_generics
            #where_clause
        {
            #[inline]
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            fn map_one_transform<F>(self, f: &mut F) -> Self
            where
                F: ::scrapmetal::GenericTransform,
            {
                Self {
                    #( #transforms )*
                }
            }

            #[inline]
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            fn map_one_query<Q, R, F>(&self, q: &mut Q, mut each: F)
            where
                Q: ::scrapmetal::GenericQuery<R>,
                F: FnMut(&mut Q, R),
            {
                #( #queries )*
            }

            #[inline]
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            fn map_one_mutation<M, R, F>(&mut self, mutation: &mut M, mut each: F)
            where
                M: ::scrapmetal::GenericMutate<R>,
                F: FnMut(&mut M, R),
            {
                #( #mutations )*
            }
        }
    }
}

fn impl_term_for_tuple_struct(ast: &syn::DeriveInput, fields: &[syn::Field]) -> Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let fields: Vec<_> = (0..fields.len()).map(syn::Ident::new).collect();

    let transforms: Vec<_> = fields.iter()
        .map(|i| {
            quote! {
                f.transform(self.#i) ,
            }
        })
        .collect();

    let queries: Vec<_> = fields.iter()
        .map(|i| {
            quote! {
                let r = q.query(&self.#i);
                each(q, r);
            }
        })
        .collect();

    let mutations: Vec<_> = fields.iter()
        .map(|i| {
            quote! {
                let r = m.mutate(&mut self.#i);
                each(m, r);
            }
        })
        .collect();

    quote! {
        impl #impl_generics ::scrapmetal::Term for #name #ty_generics
            #where_clause
        {
            #[inline]
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            fn map_one_transform<F>(self, f: &mut F) -> Self
            where
                F: ::scrapmetal::GenericTransform,
            {
                #name ( #( #transforms )* )
            }

            #[inline]
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            fn map_one_query<Q, R, F>(&self, q: &mut Q, mut each: F)
            where
                Q: ::scrapmetal::GenericQuery<R>,
                F: FnMut(&mut Q, R),
            {
                #( #queries )*
            }

            #[inline]
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            fn map_one_mutation<M, R, F>(&mut self, m: &mut M, mut each: F)
            where
                M: ::scrapmetal::GenericMutate<R>,
                F: FnMut(&mut M, R),
            {
                #( #mutations )*
            }
        }
    }
}

fn impl_term_for_unit_struct(ast: &syn::DeriveInput) -> Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    quote! {
        impl #impl_generics ::scrapmetal::Term for #name #ty_generics
            #where_clause
        {
            #[inline(always)]
            fn map_one_transform<F>(self, _: &mut F) -> Self
            where
                F: ::scrapmetal::GenericTransform,
            {
                self
            }

            #[inline(always)]
            fn map_one_query<Q, R, F>(&self, _: &mut Q, _: F)
            where
                Q: ::scrapmetal::GenericQuery<R>,
                F: FnMut(&mut Q, R),
            {}

            #[inline(always)]
            fn map_one_mutation<M, R, F>(&mut self, _: &mut M, _: F)
            where
                M: ::scrapmetal::GenericMutate<R>,
                F: FnMut(&mut M, R),
            {}
        }
    }
}

fn impl_term_for_enum(ast: &syn::DeriveInput, variants: &[syn::Variant]) -> Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let transforms: Vec<_> = variants.iter()
        .map(|v| {
            let variant_ident = &v.ident;
            match v.data {
                syn::VariantData::Struct(ref fields) => {
                    let field_names: Vec<_> = fields.iter()
                        .map(|f| {
                            let ident = &f.ident;
                            quote! {
                                #ident ,
                            }
                        })
                        .collect();

                    let field_transforms: Vec<_> = fields.iter()
                        .map(|f| {
                            let ident = &f.ident;
                            quote! {
                                #ident : f.transform( #ident ) ,
                            }
                        })
                        .collect();

                    quote! {
                        #name :: #variant_ident { #( #field_names )* } => {
                            #name :: #variant_ident { #( #field_transforms )* }
                        }
                    }
                }
                syn::VariantData::Tuple(ref fields) => {
                    let tuple_names: Vec<_> = (0..fields.len())
                        .map(|i| {
                            let c = ('a' as u8 + i as u8) as char;
                            let mut s = String::with_capacity(1);
                            s.push(c);
                            syn::Ident::new(s)
                        })
                        .collect();

                    let tuple_patterns: Vec<_> = tuple_names.iter()
                        .map(|p| {
                            quote! {
                                #p ,
                            }
                        })
                        .collect();

                    let tuple_transforms: Vec<_> = tuple_names.iter()
                        .map(|p| {
                            quote! {
                                f.transform( #p ) ,
                            }
                        })
                        .collect();

                    quote! {
                        #name :: #variant_ident ( #( #tuple_patterns )* ) => {
                            #name :: #variant_ident ( #( #tuple_transforms )* )
                        }
                    }
                }
                syn::VariantData::Unit => {
                    quote! {
                        // Nothing to do here.
                    }
                }
            }
        })
        .collect();

    let queries: Vec<_> = variants.iter()
        .map(|v| {
            let variant_ident = &v.ident;
            match v.data {
                syn::VariantData::Struct(ref fields) => {
                    let field_names: Vec<_> = fields.iter()
                        .map(|f| {
                            let ident = &f.ident;
                            quote! {
                                ref #ident ,
                            }
                        })
                        .collect();

                    let field_queries: Vec<_> = fields.iter()
                        .map(|f| {
                            let ident = &f.ident;
                            quote! {
                                let r = q.query( #ident );
                                each(q, r);
                            }
                        })
                        .collect();

                    quote! {
                        #name :: #variant_ident { #( #field_names )* } => {
                            #( #field_queries )*
                        }
                    }
                }
                syn::VariantData::Tuple(ref fields) => {
                    let tuple_names: Vec<_> = (0..fields.len())
                        .map(|i| {
                            let c = ('a' as u8 + i as u8) as char;
                            let mut s = String::with_capacity(1);
                            s.push(c);
                            syn::Ident::new(s)
                        })
                        .collect();

                    let tuple_patterns: Vec<_> = tuple_names.iter()
                        .map(|p| {
                            quote! {
                                ref #p ,
                            }
                        })
                        .collect();

                    let tuple_queries: Vec<_> = tuple_names.iter()
                        .map(|p| {
                            quote! {
                                let r = q.query( #p );
                                each(q, r);
                            }
                        })
                        .collect();

                    quote! {
                        #name :: #variant_ident ( #( #tuple_patterns )* ) => {
                            #( #tuple_queries )*
                        }
                    }
                }
                syn::VariantData::Unit => {
                    quote! {
                        // Nothing to do here.
                    }
                }
            }
        })
        .collect();

    let mutations: Vec<_> = variants.iter()
        .map(|v| {
            let variant_ident = &v.ident;
            match v.data {
                syn::VariantData::Struct(ref fields) => {
                    let field_names: Vec<_> = fields.iter()
                        .map(|f| {
                            let ident = &f.ident;
                            quote! {
                                ref mut #ident ,
                            }
                        })
                        .collect();

                    let field_mutations: Vec<_> = fields.iter()
                        .map(|f| {
                            let ident = &f.ident;
                            quote! {
                                let r = m.mutate( #ident );
                                each(m, r);
                            }
                        })
                        .collect();

                    quote! {
                        #name :: #variant_ident { #( #field_names )* } => {
                            #( #field_mutations )*
                        }
                    }
                }
                syn::VariantData::Tuple(ref fields) => {
                    let tuple_names: Vec<_> = (0..fields.len())
                        .map(|i| {
                            let c = ('a' as u8 + i as u8) as char;
                            let mut s = String::with_capacity(1);
                            s.push(c);
                            syn::Ident::new(s)
                        })
                        .collect();

                    let tuple_patterns: Vec<_> = tuple_names.iter()
                        .map(|p| {
                            quote! {
                                ref mut #p ,
                            }
                        })
                        .collect();

                    let tuple_mutations: Vec<_> = tuple_names.iter()
                        .map(|p| {
                            quote! {
                                let r = m.mutate( #p );
                                each(m, r);
                            }
                        })
                        .collect();

                    quote! {
                        #name :: #variant_ident ( #( #tuple_patterns )* ) => {
                            #( #tuple_mutations )*
                        }
                    }
                }
                syn::VariantData::Unit => {
                    quote! {
                        // Nothing to do here.
                    }
                }
            }
        })
        .collect();

    quote! {
        impl #impl_generics ::scrapmetal::Term for #name #ty_generics
            #where_clause
        {
            #[inline]
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            fn map_one_transform<F>(self, f: &mut F) -> Self
            where
                F: ::scrapmetal::GenericTransform,
            {
                match self {
                    #( #transforms )*
                }
            }

            #[inline]
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            fn map_one_query<Q, R, F>(&self, q: &mut Q, mut each: F)
            where
                Q: ::scrapmetal::GenericQuery<R>,
                F: FnMut(&mut Q, R),
            {
                match *self {
                    #( #queries )*
                }
            }

            #[inline]
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            fn map_one_mutation<M, R, F>(&mut self, m: &mut M, mut each: F)
            where
                M: ::scrapmetal::GenericMutate<R>,
                F: FnMut(&mut M, R),
            {
                match *self {
                    #( #mutations )*
                }
            }
        }
    }
}
