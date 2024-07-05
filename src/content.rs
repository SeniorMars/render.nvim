use crate::{
    error::Error,
    render::{CodeId, Fold, FoldInner, FoldState, Metadata},
};
// use magick_rust::MagickWand;
use regex::Regex;
use std::{
    collections::BTreeMap,
    path::PathBuf,
    sync::{Arc, RwLock},
};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct NodeDim {
    pub(crate) height: usize,
    pub(crate) crop: Option<(usize, usize)>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum NodeView {
    Hidden,
    UpperBorder(usize, usize),
    LowerBorder(usize, usize),
    Visible(usize, usize),
}

impl NodeView {
    pub fn new(node: &Node, metadata: &Metadata, offset: isize) -> NodeView {
        let start;
        let mut height = node.range.1 - node.range.0 + 1;

        if offset <= -(height as isize) {
            // if we are above the upper line, just skip
            return NodeView::Hidden;
        } else if offset < 0 {
            // if we are in the upper cross-over region, calculate the visible height
            start = (-offset) as usize;
            height -= start;
            return NodeView::UpperBorder(start, height);
        }

        let distance_lower = metadata.viewport.0 as isize - offset;

        //dbg!(&offset, &height, &distance_lower);

        if distance_lower <= 0 {
            return NodeView::Hidden;
        } else if (distance_lower as usize) < height {
            // remove some height if we are in the command line region
            height -= (height as isize - distance_lower) as usize;
            start = offset as usize;
            return NodeView::LowerBorder(start, height);
        }

        NodeView::Visible(offset as usize, height)
    }

    pub fn is_visible(&self) -> bool {
        self != &NodeView::Hidden
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ContentType {
    Math,
    Gnuplot,
    Tex,
    File,
}

impl ContentType {
    pub fn from_fence(kind: &str) -> Result<Self, Error> {
        match kind {
            "math" => Ok(Self::Math),
            "gnuplot" => Ok(Self::Gnuplot),
            "latex" | "tex" => Ok(Self::Tex),
            _ => Err(Error::UnknownFence(kind.to_string())),
        }
    }
}

pub enum ContentState {
    Empty,
    Running,
    Ok,
    Err(Error),
}

impl ContentState {
    pub fn new() -> Shared<ContentState> {
        Arc::new(RwLock::new(ContentState::Empty))
    }
}

type Shared<T> = Arc<RwLock<T>>;

pub struct Node {
    pub id: CodeId,
    pub range: (usize, usize),
    content: (String, ContentType),
    state: Shared<ContentState>,
}

impl Node {
    pub fn new(id: CodeId, range: (usize, usize), content: &str, kind: ContentType) -> Node {
        let state = ContentState::new();
        let content = (content.to_string(), kind);

        Node {
            id,
            range,
            state,
            content,
        }
    }
}

pub struct Content {
    // matches fenced code blocks
    fences_regex: Regex,
    // image links in markdown
    file_regex: Regex,
    // matches headers
    header_regex: Regex,
    // matches newlines
    newlines: Regex,
}

type Proccessed = (
    BTreeMap<String, Node>,
    BTreeMap<usize, FoldInner>,
    Vec<usize>,
    bool,
);
