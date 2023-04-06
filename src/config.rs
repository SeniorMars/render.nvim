use std::ffi;

use nvim_oxi as oxi;
use oxi::{
    conversion::{self, FromObject, ToObject},
    lua,
    serde::{Deserializer, Serializer},
    Object,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Opts {
    auto_display: Option<bool>,
    protocol: Option<String>,
    #[serde(default)]
    latex: LatexOpts,
    #[serde(default)]
    typst: TypstOpts,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct PDFRendererOpts {
    background_color: Option<String>,
    font_color: Option<String>,
    font: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct LatexOpts {
    #[serde(default)]
    pdf_options: PDFRendererOpts,
    #[serde(default)]
    enable: bool,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct TypstOpts {
    #[serde(default)]
    fill: PDFRendererOpts,
    background: Option<String>,
    foreground: Option<String>,
    #[serde(default)]
    enable: bool,
}

impl FromObject for Opts {
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}

impl ToObject for Opts {
    fn to_object(self) -> Result<Object, conversion::Error> {
        self.serialize(Serializer::new()).map_err(Into::into)
    }
}

impl lua::Poppable for Opts {
    unsafe fn pop(lstate: *mut lua::ffi::lua_State) -> Result<Self, lua::Error> {
        let obj = Object::pop(lstate)?;
        Self::from_object(obj).map_err(lua::Error::pop_error_from_err::<Self, _>)
    }
}

impl lua::Pushable for Opts {
    unsafe fn push(self, lstate: *mut lua::ffi::lua_State) -> Result<ffi::c_int, lua::Error> {
        self.to_object()
            .map_err(lua::Error::push_error_from_err::<Self, _>)?
            .push(lstate)
    }
}
