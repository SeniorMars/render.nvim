use std::collections::BTreeMap;

use serde::Deserialize;

use crate::content::NodeView;

// represents the id of a code block
pub type CodeId = String;

// where folds are located
pub type FoldRanges = Vec<(usize, isize)>;

#[derive(Debug, Deserialize)]
pub struct Metadata {
    /// the dimensions of the file
    file_range: (u64, u64),
    /// the dimensions of the viewport
    pub(crate) viewport: (u64, u64),
    /// the cursor position
    cursor: u64,
    /// the position of the window
    winpos: (usize, usize),
    /// the height of a character
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

#[derive(Debug)]
pub enum FoldInner {
    Fold(Fold),
    Node((CodeId, NodeView)),
}

// impl FoldInner {
//     pub fn is_in_view(&self, metadata: &Metadata, blocks: &BTreeMap<CodeId, Node>) -> bool {
//         match self {
//             FoldInner::Node((id, _)) => {
//                 let range = blocks.get(id).unwrap().range;
//
//                 range.1 as u64 >= metadata.file_range.0 && range.0 as u64 <= metadata.file_range.1
//             }
//             FoldInner::Fold(ref fold) => {
//                 fold.line as u64 >= metadata.file_range.0
//                     && fold.line as u64 <= metadata.file_range.1
//             }
//         }
//     }
// }

// impl FoldInner {
//     pub fn is_in_view(&self, metadata: &Metadata, blocks: &BTreeMap<CodeId, Node>) -> bool {
//         match self {
//             FoldInner::Node((id, _)) => blocks
//                 .get(id)
//                 .and_then(|node| {
//                     let (start, end) = node.range;
//                     Some(
//                         end as u64 >= metadata.file_range.0
//                             && start as u64 <= metadata.file_range.1,
//                     )
//                 })
//                 .unwrap_or(false),
//             FoldInner::Fold(Fold { line, .. }) => {
//                 *line as u64 >= metadata.file_range.0 && *line as u64 <= metadata.file_range.1
//             }
//         }
//     }
// }
