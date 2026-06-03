pub struct LinesManager {
    lines_lenght: Vec<usize>,
}

impl LinesManager {
    pub fn init(string: &String) -> LinesManager {
        let mut vec: Vec<usize> = Vec::new();
        for line in string.split('\n') {
            vec.push(line.chars().count());
        }
        LinesManager { lines_lenght: vec }
    }

    //too slow
    pub fn recalculate_line_lenghts(&mut self, string: String) {
        let mut vec: Vec<usize> = Vec::new();
        for line in string.split('\n') {
            vec.push(line.chars().count());
        }
        self.lines_lenght = vec;
    }

    pub fn increment_lenght(&mut self, line: usize) {
        self.lines_lenght[line] += 1;
    }

    pub fn get_line_lenght(&self, line: usize) -> usize {
        self.lines_lenght[line]
    }

    pub fn get_num_lines(&self) -> usize {
        self.lines_lenght.len()
    }

    pub fn get_line_lenght_vec(&self) -> Vec<usize> {
        let mut vec: Vec<usize> = Vec::new();
        for i in 0..self.get_num_lines() {
            vec.push(self.get_line_lenght(i))
        }
        vec
    }
}
