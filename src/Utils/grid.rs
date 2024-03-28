use std::fmt;

pub struct Grid {
    pub map: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("Grid");
        debug_struct.field("width", &self.width);
        debug_struct.field("height", &self.height);
        let map_str: Vec<String> = self.map.iter().map(|row| format!("{:?}", row)).collect();
        debug_struct.field("map", &map_str);
        debug_struct.finish()
    }
}

impl Grid {
    pub fn new(input: String) -> Grid {
        let mut height = 0;
        let mut width = 0;

        let resultant_vector: Vec<Vec<char>> = input
            .lines()
            .map(|line| {
                let characters: Vec<char> = line.chars().collect();
                height += 1;
                width = characters.len();
                characters
            })
            .collect();

        Grid {
            map: resultant_vector,
            width,
            height,
        }
    }
}
