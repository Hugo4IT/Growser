use gdnative::prelude::*;
#[macro_use] extern crate html5ever;

use html::{Html, node::HtmlNode};

mod html;

fn init(handle: InitHandle) {
    handle.add_class::<Html>();
    handle.add_class::<HtmlNode>();
}

godot_init!(init);