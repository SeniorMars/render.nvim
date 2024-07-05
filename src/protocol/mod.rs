use std::{collections::HashMap, io::Stdin};

use crate::error::Error;

/// This module contains the kitty remote protocol implementation
mod kitty;
// mod sixel;
// mod iterm2;

pub type Image = Vec<u8>; // for not, we represent images as raw bytes
pub type ImageId = u32;

pub struct Setup {
    state: State,
    child: Option<Child>,
    features: Features,
}

pub struct State {
    images: HashMap<ImageId, Image>,
}

struct Child {
    handle: std::process::Child,
    pid: u32,
    stdin: Stdin,
}

pub struct Features {
    crop: bool,
}

pub trait Protocol {
    fn setup(state: Setup) -> Result<Self, Error>
    where
        Self: Sized;

    fn render(&mut self, image: Image, x: u32, y: u32, width: u32, height: u32);
    fn clear(&mut self, image_id: Option<ImageId>, shallow: bool);
}
