pub struct CursorPos{
    x: usize,
    y: usize,
    max: usize,
}

impl CursorPos{
    pub fn new(x: usize, y:usize) -> CursorPos{
        CursorPos{
            x: x,
            y: y,
            max: x
        }
    }

    pub fn get_x(&self) -> usize{
        self.x
    }

    pub fn get_y(&self) -> usize{
        self.y
    }

    pub fn set_x(&mut self, new_x: usize){
        self.x = new_x;
    }

    pub fn set_y(&mut self, new_y: usize){
        self.y = new_y;
    }

    pub fn inc_x(&mut self){
        self.x += 1;
    }

    pub fn inc_y(&mut self){
        self.y += 1;
    }

    pub fn dec_y(&mut self){
        self.y -= 1;
    }

    pub fn dec_x(&mut self){
        self.x -= 1;
    }

    pub fn update_max(&mut self){
        self.max = self.x;
    }

    pub fn get_max(&self) -> usize{
        self.max
    }
}

