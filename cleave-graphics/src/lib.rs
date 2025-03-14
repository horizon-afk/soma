mod error;
mod graphics_bundle;
mod graphics_impl;
mod texture;
mod vertex;

pub mod prelude {
    pub use crate::error::CleaveGraphicsError;
    pub use crate::graphics_bundle::GraphicsBundle;
    pub use crate::graphics_impl::{Graphics, GraphicsOutput, GraphicsPass};
    pub use crate::texture::{RenderTexture, TextureBundle};
    pub use crate::vertex::Vertex;
}

pub type GraphicsResult<T> = Result<T, CleaveGraphicsError>;

use error::CleaveGraphicsError;
