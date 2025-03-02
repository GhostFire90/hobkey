#![feature(proc_macro_quote)]
extern crate proc_macro;
use proc_macro::*;
use quote::quote;
use syn::{Expr, Stmt};

/// # preserve_temp_map
/// This macro is used as an attribute to preserve the temp map so it is identical to when it is called
/// ## Limitations 12/16/24
/// - Must use return statement if it has a return type for detection
/// - Must not use variable name tmp_m 
#[proc_macro_attribute]
pub fn preserve_temp_map(_ : TokenStream, item : TokenStream) -> TokenStream{
    let mut body : syn::Item = syn::parse(item).unwrap();
   
    let fn_body = match &mut body{
        syn::Item::Fn(fn_item) => fn_item,
        _=> panic!("expected function")
    };
    fn_body.block.stmts.insert(0, syn::parse(quote!(let tmp_mapped : u64 = crate::memory::paging::PageTableManager::get_temp();).into()).unwrap());
    let mut idx = 0;
    let mut indexes : Vec<usize> = Vec::new();
    for s in &fn_body.block.stmts{
        match s{
            Stmt::Expr(Expr::Return(_), _) => indexes.push(idx),
            _ => {}
        }
        idx+=1;
    }
    //println!("{:?}",indexes);
    if indexes.len() != 0{
        for i in indexes{
            fn_body.block.stmts.insert(i, syn::parse(quote!(crate::memory::paging::PageTableManager::map_temp(tmp_mapped).unwrap();).into()).unwrap());
            
        }
    }
    else{
        fn_body.block.stmts.insert(fn_body.block.stmts.len(), syn::parse(quote!(crate::memory::paging::PageTableManager::map_temp(tmp_mapped).unwrap();).into()).unwrap());
    }
    

    use quote::ToTokens;
    
    let ret = body.into_token_stream().into();
    println!("{}", ret);
    ret
}