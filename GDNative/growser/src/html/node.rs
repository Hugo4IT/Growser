use gdnative::{prelude::*, api::{VBoxContainer, PanelContainer, HBoxContainer}};

#[derive(FromVariant, ToVariant, Clone, Debug)]
pub(crate) enum HtmlNodeType {
    Generic,
    Text(String)
}

#[derive(NativeClass)]
#[inherit(PanelContainer)]
pub(crate) struct HtmlNode {
    node_type: HtmlNodeType,
    custom_node: Option<Ref<Control>>
}

#[methods]
impl HtmlNode{
    pub fn new(_owner: TRef<PanelContainer>) -> Self {
        Self {
            node_type: HtmlNodeType::Generic,
            custom_node: None
        }
    }

    #[export]
    pub fn add_html_child(&mut self, owner: TRef<PanelContainer>, new_child: Ref<Control>) {
        match self.node_type {
            HtmlNodeType::Generic => {
                let vbox = unsafe { self.custom_node.as_ref().unwrap().assume_safe().cast::<VBoxContainer>().unwrap() };
                vbox.add_child(new_child, false);
            },
            HtmlNodeType::Text(_) => {
                if let Some(hbox) = unsafe { self.custom_node.as_ref().unwrap().assume_safe().cast::<HBoxContainer>() } {
                    hbox.add_child(new_child, false);
                } else {
                    let label = self.custom_node.take().unwrap();
                    let hbox = HBoxContainer::new();
                    owner.remove_child(label);
                    hbox.add_child(label, false);
                    hbox.add_child(new_child, false);
                    // let vbox_size = vbox.size();
                    owner.add_child(hbox, false);
                    // owner.set_size(vbox_size, false);
                }
            }
        }
    }

    #[export]
    pub fn set_node_type(&mut self, owner: TRef<PanelContainer>, new_type: HtmlNodeType) {
        self.node_type = new_type.clone();

        match new_type {
            HtmlNodeType::Generic => {
                let vbox = VBoxContainer::new();
                owner.add_child(unsafe { vbox.assume_shared() }, false);
                self.custom_node = unsafe { Some(vbox.upcast::<Control>().assume_shared() )};
            }
            HtmlNodeType::Text(text) => {
                let label = Label::new();
                label.set_text(text);
                let label_size = label.size();
                owner.add_child(unsafe { label.assume_shared() }, false);
                self.custom_node = unsafe { Some(label.upcast::<Control>().assume_shared()) };
                owner.set_size(label_size, false);
            }
        }
    }
}