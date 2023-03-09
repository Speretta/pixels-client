#[derive(Debug)]
pub enum CanvasError {
    ClientError,
    Cooldown(f32),
}
