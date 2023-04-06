use serde::Deserialize;

pub type CodeId = String;
pub type Folds = Vec<(usize, isize)>;

#[derive(Debug, Deserialize)]
pub struct Metadata {
    file_range: (u64, u64),
    viewport: (u64, u64),
    cursor: u64,
    winpos: (usize, usize),
    char_height: usize,
}

impl Metadata {
    pub fn new() -> Self {
        Self {
            file_range: (1, 1),
            viewport: (1, 1),
            cursor: 1,
            winpos: (1, 1),
            char_height: 0,
        }
    }
}


#[derive(PartialEq, Eq, Clone, Debug)]
pub enum FoldState {
    Folded(usize),
    Open,
}

#[derive(Debug)]
pub struct Fold {
    pub line: usize,
    pub state: FoldState,
}

// #[derive(Debug)]
// pub enum FoldInner {
//     Fold(Fold),
//     Node((CodeId, NodeView)),
// }
