
#[derive(Debug)]
pub struct A {
    x: i32,
    y: i32,
}

impl A {

    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }
}
