use proc_macro::{Span, TokenStream};
use quote;
use syn::{parse_macro_input, FnArg, Ident, ItemFn, Pat};

#[proc_macro_attribute]
pub fn middleware(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = attr.to_string();

    let attrs: Vec<&str> = attr.split(",").collect();
    if attrs.len() < 2 {
        panic!("No pass method or uri example: #[middleware(get,/index.html)]")
    }

    let input = parse_macro_input!(item as ItemFn);

    let fn_name = input.sig.ident.clone();
    let p = get_params_name(&input, 0);

    let block = input.block;
    let method = get_attrs(&attrs, 0);
    let endpoint = get_attrs(&attrs, 1);

    quote::quote! {
         #[derive(Clone)]
         pub struct #fn_name;
        impl Middleware for #fn_name {
            fn run(&self,#p:Request)->middleware::State{
                self.callback(#p)
            }fn callback(&self,#p:Request)-> middleware::State{
                #block
            }fn endpoint(&self)->(Method,String){
                (Method::from_string(#method.to_string()),#endpoint.to_string())
            }
        }
    }
    .into()
}

fn get_attrs(a: &Vec<&str>, index: usize) -> String {
    let result: Vec<_> = a[index].split_whitespace().collect();
    result.join("")
}

fn get_params_name(inpust: &ItemFn, index: usize) -> Ident {
    let default = Ident::new("_ignored_", Span::call_site().into());
    return match inpust.sig.inputs.get(index) {
        | Some(fnarg) => {
            // println!("Fnarg {:#?}",fnarg );
            match fnarg {
                | FnArg::Typed(t) => match *t.pat.clone() {
                    | Pat::Ident(i) => {
                        //           println!("indent pat {:#?}", &t);
                        // let prueba = t.ty.clone();
                        // match *prueba.clone() {
                        //     syn::Type::Path(ty) => println!("{:#?}", ty.path.segments.first()),
                        //     _ => (),
                        // }

                        Ident::new(&i.ident.to_string().as_str(), Span::call_site().into())
                    },

                    | _ => default,
                },
                | _ => default,
            }
        },
        | None => default,
    };
}
