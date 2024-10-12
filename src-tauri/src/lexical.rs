use serde::Deserialize;

pub trait GetRawText {
    fn get_raw_text(&self) -> Option<String>;
}

#[derive(Deserialize)]
pub struct NodeChildren(Vec<LexicalNode>);

impl GetRawText for NodeChildren {
    fn get_raw_text(&self) -> Option<String> {
        let children = self.0.iter();
        let children: Vec<String> = children
            .filter_map(|child| match child {
                LexicalNode::Autolink(link) => link.get_raw_text(),
                LexicalNode::List(list) => list.get_raw_text(),
                LexicalNode::Text(text) => text.get_raw_text(),
                LexicalNode::Paragraph(para) => para.get_raw_text(),
                LexicalNode::Heading(heading) => heading.get_raw_text(),
                LexicalNode::Listitem(item) => item.get_raw_text(),
                LexicalNode::Quote(quote) => quote.get_raw_text(),
                LexicalNode::Code(code) => code.get_raw_text(),
                LexicalNode::Unknown => None,
            })
            .collect();

        if children.len() == 0 {
            return None;
        }

        Some(children.join(" "))
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
enum Direction {
    Ltr,
    Rtl,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
enum Mode {
    Normal,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub text: String,
    //pub mode: Mode,
    //pub format: u8,
}

impl GetRawText for Text {
    fn get_raw_text(&self) -> Option<String> {
        let text = self.text.trim();
        if text == "" || text == " " {
            return None;
        }

        return Some(text.to_string());
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paragraph {
    //pub direction: Option<Direction>,
    //pub indent: u8,
    //pub text_format: u8,
    //pub version: u8,
    pub children: NodeChildren,
}

impl GetRawText for Paragraph {
    fn get_raw_text(&self) -> Option<String> {
        self.children.get_raw_text()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ListType {
    Bullet,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct List {
    //pub direction: Option<Direction>,
    //pub indent: u8,
    //pub list_type: ListType,
    //pub start: u8,
    //pub tag: String,
    pub children: NodeChildren,
}

impl GetRawText for List {
    fn get_raw_text(&self) -> Option<String> {
        self.children.get_raw_text()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListItem {
    //pub direction: Option<Direction>,
    pub children: NodeChildren,
    //pub indent: u8,
    //pub value: u8,
}

impl GetRawText for ListItem {
    fn get_raw_text(&self) -> Option<String> {
        self.children.get_raw_text()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Autolink {
    children: NodeChildren,
    //direction: Direction,
    //indent: u8,
    //url: String,
}

impl GetRawText for Autolink {
    fn get_raw_text(&self) -> Option<String> {
        self.children.get_raw_text()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum LexicalNode {
    Paragraph(Paragraph),
    Heading(Heading),
    Text(Text),
    List(List),
    Autolink(Autolink),
    Listitem(ListItem),
    Quote(Quote),
    Code(Code),
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Code {
    pub children: NodeChildren,
}

impl GetRawText for Code {
    fn get_raw_text(&self) -> Option<String> {
        self.children.get_raw_text()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Heading {
    pub children: NodeChildren,
    //pub direction: Option<Direction>,
    //pub indent: u8,
    //pub tag: String,
}

impl GetRawText for Heading {
    fn get_raw_text(&self) -> Option<String> {
        self.children.get_raw_text()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub children: NodeChildren,
}

impl GetRawText for Quote {
    fn get_raw_text(&self) -> Option<String> {
        self.children.get_raw_text()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementNode {
    //pub direction: Option<Direction>,
    //pub indent: u8,
    pub children: NodeChildren,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorState {
    pub root: ElementNode,
}

impl GetRawText for EditorState {
    fn get_raw_text(&self) -> Option<String> {
        let root = &self.root;
        root.children.get_raw_text()
    }
}
