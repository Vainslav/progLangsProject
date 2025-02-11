use termion::cursor;

#[derive(Clone)]
pub struct CursorPos{
    x_display: u16,
    x_actual: usize,
    y_display: u16,
    y_actual: usize,
    max: usize,
}

impl CursorPos{
    pub fn new(x: u16, y: u16) -> CursorPos{
        CursorPos{
            x_display: x,
            x_actual: x as usize,
            y_display: y,
            y_actual: y as usize,
            max: x as usize
        }
    }

    pub fn get_x_display(&self) -> u16{
        self.x_display
    }

    pub fn get_y_display(&self) -> u16{
        self.y_display
    }

    pub fn set_x_actual(&mut self, new_x: usize){
        self.x_actual = new_x;
    }

    pub fn set_y_actual(&mut self, new_y: usize){
        self.y_actual = new_y;
    }

    pub fn inc_x(&mut self){
        self.x_actual += 1;
        if self.x_display != termion::terminal_size().unwrap().0{
            self.x_display += 1;
        }
    }

    pub fn inc_y(&mut self){
        self.y_actual += 1;
        if self.y_display != termion::terminal_size().unwrap().1 - 1{
            self.y_display += 1;
        }
    }

    pub fn dec_y(&mut self){
        self.y_actual -= 1;
        if self.y_display != 1{
            self.y_display -= 1;
        }
    }

    pub fn dec_x(&mut self){
        self.x_actual -= 1;
        if self.x_display != 1{
            self.x_display -= 1;
        }
    }

    pub fn update_max(&mut self){
        self.max = self.x_actual;
    }

    pub fn get_max(&self) -> usize{
        self.max
    }

    pub fn set_max_newline(&mut self){
        self.max = 1;
    }

    pub fn get_x_actual(&self) -> usize{
        self.x_actual
    }

    pub fn get_y_actual(&self) -> usize{
        self.y_actual
    }

    pub fn set_x_display(&mut self, new_x: u16){
        self.x_display = {
            if new_x > termion::terminal_size().unwrap().0{
                termion::terminal_size().unwrap().0
            }else{
                new_x
            }
        }
    }

    pub fn set_y_display(&mut self, new_y: u16){
        self.y_display = {
            if new_y > termion::terminal_size().unwrap().1 - 1{
                termion::terminal_size().unwrap().1 - 1
            }else{
                new_y
            }
        }
    }

    pub unsafe fn set_x_display_unsafe(&mut self, new_x: u16){
        self.x_display = new_x
    }

    pub unsafe fn set_y_display_unsafe(&mut self, new_y: u16){
        self.y_display = new_y
    }
}
