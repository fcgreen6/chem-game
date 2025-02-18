extern crate termion;
use termion::color::{Fg, Rgb};

pub struct ActionLog {

    actions: Vec<String>,
    is_error: Vec<bool>
}

// Initialized with two empty vectors.
impl Default for ActionLog {
    fn default() -> Self {
        ActionLog {

            actions: Vec::new(),
            is_error: Vec::new()
        }
    }
}

impl ActionLog {
    
    // PushAction Function:
    // Parameters:
    // - action: String description of the action.
    // - error: flag if the action is an error action.
    // Description: Puts an action on the bottom of the action log.
    pub fn PushAction(&mut self, action: String, error: bool) {

        self.actions.push(action.clone());
        self.is_error.push(error);

        if self.actions.len() > 5 {

            self.actions.remove(0);
            self.is_error.remove(0);
        }
    }

    // PrintIndex Function:
    // Parameters:
    // - index: The index to print from on the action log.
    // Description: Prints the element at the given index. Red if its an error. Green otherwise.
    pub fn PrintIndex(&self, index: usize) {

        if index >= self.actions.len() {

            return;
        }

        // Determine color of message.
        let mut col = Fg(Rgb(65, 221, 68));;
        if self.is_error[index] {

            col = Fg(Rgb(228, 8, 10));
        }

        print!("{}{}{}", col, self.actions[index], Fg(Rgb(255, 255, 255)));
    }
}

