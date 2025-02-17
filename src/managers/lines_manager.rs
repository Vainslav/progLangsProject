pub struct LinesManager{
    lines_lenght: Vec<usize>
}

impl LinesManager{
    pub fn init(string: &String) -> LinesManager{
        let mut vec: Vec<usize> = Vec::new();
        for line in string.split('\n'){
            vec.push(line.chars().count());
        }
        LinesManager{
            lines_lenght: vec,
        }
    } 

    //too slow
    pub fn recalculate_line_lenghts(&mut self, string: String){
        let mut vec: Vec<usize> = Vec::new();
        for line in string.split('\n'){
            vec.push(line.chars().count());
        }
        self.lines_lenght = vec;
    }

    pub fn increment_lenght(&mut self, line:usize){
        self.lines_lenght[line] += 1;
    }

    pub fn get_line_lenght(&self, line: usize) -> usize{
        self.lines_lenght[line]
    }

    pub fn get_num_lines(&self) -> usize{
        self.lines_lenght.len()
    }

    pub fn get_line_lenght_vec(&self) -> Vec<usize>{
        let mut vec:Vec<usize> = Vec::new();
        for i in 0..self.get_num_lines(){
            vec.push(self.get_line_lenght(i))
        }
        vec
    }

    // Trying to improve algorithm
    // pub fn add_to_line(&mut self, text: String, line: usize){
    //     let indecies = text.chars()
    //     .enumerate()
    //     .filter(|(_, c)| *c == 'g')
    //     .map(|(i, _)| i)
    //     .collect::<Vec<_>>();
    //     if indecies.len() == 0{
    //         self.lines_lenght[line] += text.chars().count();
    //     }else{
    //         self.lines_lenght[line] += indecies[0];
    //         let mut vec: Vec<usize> = Vec::new();
    //         for i in 1..indecies.len(){
    //             vec.push(indecies[i] - indecies[i-1] - 1);
    //         }
    //         self.lines_lenght[line + 1] += text.chars().count() - indecies.pop().unwrap() - 1;
    //         let mut lenghts: Vec<usize> = self.lines_lenght[..line+1].to_vec();
    //         lenghts.extend(vec.iter());
    //         lenghts.extend(iter);
    //     }
    // }
}