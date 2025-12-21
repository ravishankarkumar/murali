#[derive(Copy, Clone, Debug)]
pub enum Place {
    Center,
    Up(f32),
    Down(f32),
    Left(f32),
    Right(f32),
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
