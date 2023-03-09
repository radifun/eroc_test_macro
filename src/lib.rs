// =================================================================================================
// Copyright (c) 2023 Viet-Hoa Do <doviethoa@doviethoa.com>
//
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// =================================================================================================

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn test_case(_attributes: TokenStream, content: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(content);

    if let syn::Item::Fn(function) = item {
        let syn::ItemFn {
            attrs,
            block,
            vis,
            sig: syn::Signature { ident, unsafety, constness, inputs, .. },
            ..
        } = function;

        let init_items = quote::quote!();

        let code = quote::quote!(
            #[test]
            #(#attrs)*
            #vis #unsafety #constness fn #ident() {
                fn run_test(#inputs) #block

                #init_items
                run_test();
            }
        );

        return code.into();
    } else {
        panic!("Cannot use test_case attribute.");
    }
}
