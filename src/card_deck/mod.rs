extern crate rand;
use rand::Rng;

#[derive(Clone, PartialEq, Eq)]
pub struct Card {

    // Default atom data members.
    pub name: String,
    pub symbol: String,
    pub number_of_bonds: u16,
    pub is_metal: bool,
    pub atomic_number: u16,

    // Evolution data members.
    evo_name: String,
    evo_symbol: String,
    evo_atomic_number: u16
}

impl Card {

    // When evolved, an atom's evo fields become the main feilds.
    pub fn Evolve(&mut self) {

        self.name = self.evo_name.clone();
        self.symbol = self.evo_symbol.clone();
        self.atomic_number = self.evo_atomic_number.clone();
    }
}

pub struct CardDeck {

    deck: Vec<Card>,
    hand: Vec<Card>
}

impl Default for CardDeck {

    fn default() -> Self {

        // Create card structs which will be used to initialize the deck.
        let hydrogen = Card{name: String::from("H"), symbol: String::from("H "), number_of_bonds: 1, 
        is_metal: false, atomic_number: 1, evo_name: String::from("Li"), evo_symbol: String::from("Li"), evo_atomic_number: 3};

        let lithium = Card{name: String::from("Li"), symbol: String::from("Li"), number_of_bonds: 1, 
        is_metal: true, atomic_number: 3, evo_name: String::from("Na"), evo_symbol: String::from("Na"), evo_atomic_number: 11};

        let beryllium = Card{name: String::from("Be"), symbol: String::from("Be"), number_of_bonds: 2, 
        is_metal: true, atomic_number: 4, evo_name: String::from("Mg"), evo_symbol: String::from("Mg"), evo_atomic_number: 12};

        let carbon = Card{name: String::from("C"), symbol: String::from("C "), number_of_bonds: 4, 
        is_metal: false, atomic_number: 6, evo_name: String::from("Si"), evo_symbol: String::from("Si"), evo_atomic_number: 14};

        let nitrogen = Card{name: String::from("N"), symbol: String::from("N "), number_of_bonds: 3, 
        is_metal: false, atomic_number: 7, evo_name: String::from("P"), evo_symbol: String::from("P "), evo_atomic_number: 15};

        let oxygen = Card{name: String::from("O"), symbol: String::from("O "), number_of_bonds:2, 
        is_metal: false, atomic_number: 8, evo_name: String::from("S"), evo_symbol: String::from("S "), evo_atomic_number: 16};

        let flourine = Card{name: String::from("F"), symbol: String::from("F "), number_of_bonds: 1, is_metal: false,atomic_number: 9, evo_name: String::from("Cl"), evo_symbol: String::from("Cl"), evo_atomic_number: 17};

        let mut card_vector: Vec<Card> = Vec::new();

        // Add four hydrogens and four oxygens.
        for i in 0..4 {

            card_vector.push(hydrogen.clone());
            card_vector.push(oxygen.clone());
        }

        // Two lithium, beryllium, and carbon.
        for i in 0..2 {

            card_vector.push(lithium.clone());
            card_vector.push(beryllium.clone());
            card_vector.push(carbon.clone());
        }

        // One nitrigen and flourine.
        card_vector.push(nitrogen.clone());
        card_vector.push(flourine.clone());

        CardDeck {
            
            deck: card_vector,
            hand: Vec::new()
        }
    }
}

impl CardDeck {
    
    // AddToHand Function:
    // Description: Moves a card from the deck vector into the hand vector.
    pub fn AddToHand(&mut self) {

        match self.DrawCard() {
            
            Some(card) => {
                
                self.hand.push(card);
            },
            None => (),
        }
    }

    // DrawCard Function:
    // Description: Returns a card if there are cards remaining in the deck. Removes the returned value from the deck.
    // Return: Card representing an atom within and option.
    fn DrawCard(&mut self) -> Option<Card> {

        // In case of an empty deck.
        if self.deck.len() == 0 {

            return None;
        }

        // Initialize random number generator.
        let mut random = rand::thread_rng();

        // Generate a random index and use it to choose a card.
        let index: usize = random.gen_range(0..self.deck.len());
        let ret_val = self.deck[index].clone();

        // Remove the chosen card from the struct.
        self.deck.remove(index);

        return Some(ret_val);
    }

    // GetCard Function:
    // Parameters:
    // - card_name: Name of the atom displayed on the card.
    // Description: Returns a card based on the name of the card proveided. Returns None if the name is invalid.
    pub fn GetCard(&mut self, card_name: String) -> Option<Card> {

        for i in 0..self.hand.len() {

            if self.hand[i].name == card_name {

                return Some(self.hand[i].clone());
            }
        }

        return None;
    }

    // GetFromIndex Function:
    // Parameters:
    // - card_index: Index of the atom displayed on the card.
    // Description: Returns a card based on the index of the card proveided.
    pub fn GetFromIndex(&mut self, card_index: usize) -> Card {

        return self.hand[card_index].clone();
    }

    // EvolveCard Function:
    // Parameters:
    // - card_name: Name of the card to evolve in hand.
    // Description: Evolves a card in hand based on its name.
    pub fn EvolveCard(&mut self, card_name: String) -> bool {

        for i in 0..self.hand.len() {

            if self.hand[i].name == card_name {

                self.hand[i].Evolve();
                return true
            }
        }

        return false;
    }

    // RemoveCard Function:
    // Parameters:
    // - card_name: Name of the card to remove from hand.
    // Description: Removes a card from hand based on its name.
    pub fn RemoveCard(&mut self, card_name: String) {

        for i in 0..self.hand.len() {

            if self.hand[i].name == card_name {

                self.hand.remove(i);
                return;
            }
        }
    }

    // PrintUpperHand and PrintLowerHand Functions:
    // Description: Cards are displayed across two lines. Each function prints one line of the display.
    //---------------------------------------------------------------------------------------------------
    pub fn PrintUpperHand(&mut self) {

        for i in 0..self.hand.len() {

            print!("|{}", self.hand[i].symbol);
        }

        print!("|");
    }

    pub fn PrintLowerHand(&mut self) {

        for i in 0..self.hand.len() {

            if self.hand[i].atomic_number < 10 {
                
                print!("|{} ", self.hand[i].atomic_number);
            }
            else {

                print!("|{}", self.hand[i].atomic_number);
            }
        }

        print!("|");
    }
    //---------------------------------------------------------------------------------------------------
}