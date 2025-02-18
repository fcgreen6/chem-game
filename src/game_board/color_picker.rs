extern crate rand;
use rand::Rng;

pub struct ColorPicker {

    colors: Vec<(u8, u8, u8)>,
    size: usize
}

// Initialize the struct with twenty unique colors.
impl Default for ColorPicker {

    fn default() -> Self {
        ColorPicker {
            colors: vec![
                (76, 156, 228), (125, 218, 88), (0, 128, 128), (0, 255, 255), 
                (0, 100, 0), (0, 0, 255), (0, 255, 127), (0, 0, 139), 
                (0, 206, 209), (0, 191, 255), (0, 250, 154), (0, 128, 0), 
                (0, 255, 0), (0, 139, 139), (0, 255, 255), (0, 0, 128), 
                (0, 255, 255), (0, 100, 0), (0, 0, 255), (0, 255, 127)
            ],
            size: 20
        }
    }
}

impl ColorPicker {

    // GetColor Function:
    // Description: Returns a unique color from ColorPicker's list of colors and removes taht color form the options.
    // Return: Tuple representing rgb value.
    pub fn GetColor(&mut self) -> (u8, u8, u8) {

        // Initialize random number generator.
        let mut random = rand::thread_rng();

        // Generate a random index and use it to choose a color.
        let index: usize = random.gen_range(0..self.size);
        let ret_val = self.colors[index];

        // Remove the chosen color from the struct.
        self.colors.remove(index);
        self.size = self.size - 1;

        return ret_val;
    }
}