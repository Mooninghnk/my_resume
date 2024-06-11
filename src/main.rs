#![allow(non_camel_case_types)]

use sycamore::prelude::*;

fn main() {
    sycamore::render(|| {
        view! {
            nav{
                ul{
                    li{
                        a(style="background-color: red;"){"home"}
                        a{"home"}
                        a{"home"}
                        a{"home"}
                    }
                }
            }
        }
    })
}
