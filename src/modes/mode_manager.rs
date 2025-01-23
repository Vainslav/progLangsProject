use crate::modes::mode_trait::Mode;

use super::insert_mode::InsertMode;
struct ModeManager{
    modes: Vec<Box<dyn Mode>>
}

impl ModeManager{
    pub fn init() -> Self{
        ModeManager{
            modes: vec![Box::new(InsertMode::init())]
        }
    }

    pub fn run(){

    }
}