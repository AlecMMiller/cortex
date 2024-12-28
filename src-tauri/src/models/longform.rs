use crate::macros::macros::create_id;

create_id!(LongformTextId);

create_id!(TextBlockId);

pub struct LongformContent {
    pub id: LongformTextId,
    pub blocks: Vec<TextBlock>,
}

pub struct TextBlock {
    pub id: TextBlockId,
    pub content: String,
}
