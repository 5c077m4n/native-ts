struct Location {
    pub line: u32,
    pub column: u32,
}
pub struct Node {
    pub node_type: &'_ str,
    pub start: u32,
    pub end: u32,
    pub location: Location,
}
