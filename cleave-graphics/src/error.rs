#[derive(thiserror::Error, Debug)]
pub enum CleaveGraphicsError {
    #[error("Missing adapter")]
    MissingAdapter,
    #[error("Request adapter error: {0}")]
    RequestDevice(#[from] wgpu::RequestDeviceError),
    #[error("Create surface error: {0}")]
    CreateSurface(#[from] wgpu::CreateSurfaceError),
    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),
}
