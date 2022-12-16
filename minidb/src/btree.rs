mod branch;
mod leaf;
mod meta;
mod node;

pub struct Pair<'a> {
    pub key: &'a [u8],
    pub value: &'a [u8],
}
