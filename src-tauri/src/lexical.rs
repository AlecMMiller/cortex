use serde::Deserialize;
use serde_json::Error;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum Direction {
    Ltr,
    Rtl
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum Mode {
    Normal,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Text {
    pub text: String,
    pub mode: Mode,
    pub format: u8,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Paragraph {
    pub direction: Option<Direction>,
    pub indent: u8,
    pub text_format: u8,
    pub version: u8,
    pub children: Vec<LexicalNode>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum ListType {
    Bullet,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct List {
    pub direction: Option<Direction>,
    pub indent: u8,
    pub list_type: ListType,
    pub start: u8,
    pub tag: String,
    pub children: Vec<ListItem>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ListItem {
    pub direction: Option<Direction>,
    pub children: Vec<LexicalNode>,
    pub indent: u8,
    pub value: u8
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AutoLink {
    children: Vec<LexicalNode>,
    direction: Direction,
    indent: u8,
    url: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
enum LexicalNode {
    Paragraph(Paragraph),
    Heading(Heading),
    Text(Text),
    List(List),
    Autolink(AutoLink),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Heading {
    pub children: Vec<LexicalNode>,
    pub direction: Option<Direction>,
    pub indent: u8,
    pub tag: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ElementNode {
    pub direction: Option<Direction>,
    pub indent: u8,
    pub children: Vec<LexicalNode>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EditorState {
    pub root: ElementNode,
}

impl EditorState {
    pub fn from_str(value: &str) -> Result<Self, Error> {
        serde_json::from_str(value)
    }
}