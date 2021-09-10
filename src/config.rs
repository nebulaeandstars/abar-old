use crate::statusblock::StatusBlock;

pub const fn delimiter() -> &'static str {
    " | "
}

pub fn blocks() -> Vec<StatusBlock> {
    vec![
        StatusBlock::new("test", &|| "this is a test block".to_string()),
        StatusBlock::new("block", &|| "this is another test block".to_string()),
    ]
}
