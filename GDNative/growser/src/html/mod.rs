pub mod node;

use std::collections::HashMap;

use gdnative::api::{PanelContainer};
use gdnative::api::vbox_container::VBoxContainer;
use gdnative::prelude::*;
use html5ever::{
    expanded_name, parse_document,
    tendril::TendrilSink,
    tree_builder::{
        ElementFlags,
        NodeOrText::{AppendNode, AppendText},
        TreeSink,
    },
    Attribute, LocalName, Namespace, Prefix, QualName,
};

use crate::html::node::{HtmlNode, HtmlNodeType};

enum HtmlResource {
    Source(String),
    Linked(String),
}

struct HtmlElement {
    element_type: QualName,
    attributes: Vec<Attribute>,
    children: Vec<usize>,
    parent: usize,
}

struct Sink {
    next_id: usize,
    elements: HashMap<usize, HtmlElement>,
    styles: HashMap<usize, HtmlResource>,
    scripts: HashMap<usize, HtmlResource>,
}

impl Sink {
    fn get_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 2;
        id
    }
}

impl TreeSink for Sink {
    type Handle = usize;
    type Output = Self;

    fn finish(self) -> Self::Output {
        self
    }

    fn parse_error(&mut self, msg: std::borrow::Cow<'static, str>) {
        godot_error!("Parse error: {}", msg);
    }

    fn get_document(&mut self) -> Self::Handle {
        0
    }

    fn elem_name<'a>(&'a self, target: &'a Self::Handle) -> html5ever::ExpandedName<'a> {
        self.elements
            .get(target)
            .expect("Not an element")
            .element_type
            .expanded()
    }

    fn create_element(
        &mut self,
        name: QualName,
        attrs: Vec<html5ever::Attribute>,
        flags: html5ever::tree_builder::ElementFlags,
    ) -> Self::Handle {
        let id = self.get_id();
        godot_print!("Created {:?} as {}", name, id);
        match name.local.as_ref() {
            "style" => {
                for attr in (attrs.as_ref() as &Vec<Attribute>).into_iter() {
                    if attr.name.expanded().local.as_ref() == "src" {
                        let _ = self.styles.insert(
                            id,
                            HtmlResource::Linked(attr.value.escape_default().to_string()),
                        );
                    }
                }
            }
            "script" => {
                for attr in (attrs.as_ref() as &Vec<Attribute>).into_iter() {
                    if attr.name.expanded().local.as_ref() == "src" {
                        let _ = self.scripts.insert(
                            id,
                            HtmlResource::Linked(attr.value.escape_default().to_string()),
                        );
                    }
                }
            }
            "link" => {
                let mut is_stylesheet = false;
                let mut linked_value = None;
                for attr in (attrs.as_ref() as &Vec<Attribute>).into_iter() {
                    let name = attr.name.expanded().local.as_ref();
                    if name == "src" {
                        linked_value = Some(attr.value.escape_default().to_string());
                        if is_stylesheet {
                            break;
                        }
                    } else if name == "rel" {
                        is_stylesheet = attr.value.escape_default().to_string() == "stylesheet";
                        if linked_value.is_some() {
                            break;
                        }
                    }
                }
                if is_stylesheet && linked_value.is_some() {
                    self.styles
                        .insert(id, HtmlResource::Linked(linked_value.unwrap()));
                }
            }
            _ => (),
        }
        self.elements.insert(
            id,
            HtmlElement {
                element_type: name,
                attributes: attrs,
                children: Vec::new(),
                parent: 0,
            },
        );
        id
    }

    fn create_comment(&mut self, text: html5ever::tendril::StrTendril) -> Self::Handle {
        let id = self.get_id();
        godot_print!("Created comment \"{}\" as {}", text.escape_default(), id);
        id
    }

    fn create_pi(
        &mut self,
        target: html5ever::tendril::StrTendril,
        data: html5ever::tendril::StrTendril,
    ) -> Self::Handle {
        unimplemented!()
    }

    fn append(
        &mut self,
        parent: &Self::Handle,
        child: html5ever::tree_builder::NodeOrText<Self::Handle>,
    ) {
        match child {
            AppendNode(n) => {
                if *parent != 0 {
                    self.elements.get_mut(&n).unwrap().parent = *parent;
                    self.elements.get_mut(parent).unwrap().children.push(n)
                }
            }
            AppendText(t) => match self
                .elements
                .get(parent)
                .expect("Not an element")
                .element_type
                .expanded()
                .local
                .as_ref()
            {
                "style" => {
                    if self.styles.contains_key(parent) {
                        let old = self.styles.get_mut(parent).unwrap();
                        if let HtmlResource::Source(source) = old {
                            source.push_str(&t.escape_default().to_string());
                        }
                    } else {
                        let _ = self.styles.insert(
                            *parent,
                            HtmlResource::Source(t.escape_default().to_string()),
                        );
                    }
                }
                "script" => {
                    if self.scripts.contains_key(parent) {
                        let old = self.scripts.get_mut(parent).unwrap();
                        if let HtmlResource::Source(source) = old {
                            source.push_str(&t.escape_default().to_string());
                        }
                    } else {
                        let _ = self.scripts.insert(
                            *parent,
                            HtmlResource::Source(t.escape_default().to_string()),
                        );
                    }
                }
                _ => {
                    if t.chars()
                        .filter(|s| *s != ' ' && *s != '\n' && *s != '\t')
                        .collect::<String>()
                        .len()
                        > 0
                    {
                        let node = self.create_element(
                            QualName::new(
                                Some(Prefix::from("growser")),
                                Namespace::from("https://hugo4it.com/growser/2022"),
                                LocalName::from("text"),
                            ),
                            vec![Attribute {
                                name: QualName::new(
                                    Some(Prefix::from("growser")),
                                    Namespace::from("https://hugo4it.com/growser/2022"),
                                    LocalName::from("content"),
                                ),
                                value: t,
                            }],
                            ElementFlags::default(),
                        );
                        self.append(parent, AppendNode(node));
                    }
                }
            },
        };
    }

    fn append_based_on_parent_node(
        &mut self,
        element: &Self::Handle,
        prev_element: &Self::Handle,
        child: html5ever::tree_builder::NodeOrText<Self::Handle>,
    ) {
        self.append_before_sibling(element, child);
    }

    fn append_doctype_to_document(
        &mut self,
        name: html5ever::tendril::StrTendril,
        public_id: html5ever::tendril::StrTendril,
        system_id: html5ever::tendril::StrTendril,
    ) {
        godot_print!("Append doctype: {} {} {}", name, public_id, system_id);
    }

    fn get_template_contents(&mut self, target: &Self::Handle) -> Self::Handle {
        if let Some(expanded_name!(html "template")) =
            self.elements.get(target).map(|n| n.element_type.expanded())
        {
            target + 1
        } else {
            godot_error!("Not a template element.");
            panic!("Not a template element.")
        }
    }

    fn same_node(&self, x: &Self::Handle, y: &Self::Handle) -> bool {
        x == y
    }

    fn set_quirks_mode(&mut self, mode: html5ever::tree_builder::QuirksMode) {
        godot_print!("Set quirks mode to {:?}", mode);
    }

    fn append_before_sibling(
        &mut self,
        sibling: &Self::Handle,
        new_node: html5ever::tree_builder::NodeOrText<Self::Handle>,
    ) {
        match new_node {
            AppendNode(n) => godot_print!("Append node {} before {}", n, sibling),
            AppendText(t) => {
                godot_print!("Append text before {}: \"{}\"", sibling, t.escape_default())
            }
        }
    }

    fn add_attrs_if_missing(&mut self, target: &Self::Handle, attrs: Vec<html5ever::Attribute>) {
        assert!(self.elements.contains_key(target), "Not an element");
        godot_print!("Add missing attributes to {}:", target);
        for attr in attrs.into_iter() {
            godot_print!("  {:?} = {}", attr.name, attr.value);
        }
    }

    fn remove_from_parent(&mut self, target: &Self::Handle) {
        godot_print!("Remove {} from parent", target)
    }

    fn reparent_children(&mut self, node: &Self::Handle, new_parent: &Self::Handle) {
        godot_print!("Move children from {} to {}", node, new_parent);
    }
}

#[derive(NativeClass)]
#[inherit(VBoxContainer)]
pub struct Html {}

#[methods]
impl Html {
    fn new(_owner: TRef<VBoxContainer>) -> Self {
        Self {}
    }

    #[export]
    fn load(&mut self, owner: TRef<VBoxContainer>, input: String) {
        let mut buffer = input.as_bytes();
        let sink = Sink {
            next_id: 1,
            elements: HashMap::new(),
            styles: HashMap::new(),
            scripts: HashMap::new(),
        };
        let sink = parse_document(sink, Default::default())
            .from_utf8()
            .read_from(&mut buffer)
            .unwrap();

        for i in  0..owner.get_child_count() {
            unsafe { owner.get_child(i).unwrap().assume_safe().queue_free() };
        }

        let mut nodes: HashMap<usize, Ref<PanelContainer>> = HashMap::new();
        for (id, element) in sink.elements.iter() {
            let node = HtmlNode::new_instance();
            if let Some(prefix) = element.element_type.prefix.as_ref() {
                if prefix.to_string() == "growser" {
                    match element.element_type.local.to_string().as_str() {
                        "text" => unsafe {
                            node.base().call(
                                "set_node_type",
                                &[HtmlNodeType::Text(
                                    element
                                        .attributes
                                        .iter()
                                        .filter(|a| {
                                            a.name
                                                == QualName::new(
                                                    Some(Prefix::from("growser")),
                                                    Namespace::from(
                                                        "https://hugo4it.com/growser/2022",
                                                    ),
                                                    LocalName::from("content"),
                                                )
                                        })
                                        .collect::<Vec<&Attribute>>()
                                        .get(0)
                                        .as_ref()
                                        .unwrap()
                                        .value
                                        .to_string(),
                                )
                                .to_variant()],
                            );
                        },
                        other => godot_print!("Unrecognized growser element: {}", other),
                    }
                }
            } else {
                unsafe { node.base().call("set_node_type", &[HtmlNodeType::Generic.to_variant()]); }
                match element.element_type.local.to_string().as_str() {
                    "p" => godot_print!("Found <p> element!"),
                    other => godot_print!("Unrecognized element type: {}", other),
                }
            }
            nodes.insert(*id, unsafe { node.base().assume_shared() });
        }

        for (id, node) in nodes.clone().iter_mut() {
            let element = sink.elements.get(id).unwrap();
            if element.parent != 0 {
                unsafe {
                    nodes
                        .get_mut(&element.parent)
                        .unwrap()
                        .assume_safe()
                        .call("add_html_child", &[node.assume_safe().to_variant()])
                };
            } else {
                unsafe { owner.add_child(node.assume_safe(), false) };
            }
        }

        godot_print!(
            "Styles: {}\nScripts: {}",
            sink.styles.len(),
            sink.scripts.len()
        );
    }
}
