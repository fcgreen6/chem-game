// Enum used to check the state of a tile.
#[derive(Clone, PartialEq, Eq)]
pub enum TileState {
    Empty,
    Restricted,
    Bond,
    Atom,
    Parent,
}

// Board tile data members. Members within an option do not exist within all tile states.
pub struct BoardTile {

    // Universal data members. A tile contains these regardless of state.
    state: TileState,
    color: (u8, u8, u8),
    symbol: String,

    // Restricted zone data member. Unique to restricted tiles.
    restrict_counter: Option<u8>,

    // Bond zone data member. Unique to bond zones.
    bond_tile: Option<(usize, usize)>,

    // Atom zone data members. Unique to atom zones and parent zones.
    parent: Option<(usize, usize)>,
    bond_number: Option<u16>,
    is_metal: Option<bool>,
    presedence: Option<u8>,

    // Parent atom data members. Unique to parent zones.
    number_of_atoms: Option<u8>,
    number_of_neutral: Option<u8>,
    largest_bond: Option<u16>,
    atomic_sum: Option<u16>,
    metal_component: Option<String>
}

// default Function:
// - Description: Default case for BoardTile struct. Creates an empty tile.
impl Default for BoardTile {
    fn default() -> Self {
        BoardTile {

            // Default case is an empty tile.
            state: TileState::Empty,
            color: (255, 255, 255),
            symbol: String::from("  "), // Symbol displayed in this case is an empty space.

            // Unused data members.
            restrict_counter: None,
            bond_tile: None,
            parent: None,
            bond_number: None,
            is_metal: None,
            presedence: None,
            number_of_atoms: None,
            number_of_neutral: None,
            largest_bond: None,
            atomic_sum: None,
            metal_component: None
        }
    }
}

impl BoardTile {

    //-----------------------------------------------------------------------------------------------------------------------------
    // State Management Functions
    //-----------------------------------------------------------------------------------------------------------------------------

    // ClearTile Function:
    // - Description: Creates an empty tile or resets an existing tile to empty state.
    pub fn ClearTile(&mut self) {

        // Tile becomes an empty tile.
        self.state = TileState::Empty;
        self.color = (255, 255, 255);
        self.symbol = String::from("  ");

        // Unused data members.
        self.restrict_counter = None;
        self.bond_tile = None;
        self.parent = None;
        self.bond_number = None;
        self.is_metal = None;
        self.presedence = None;
        self.number_of_atoms = None;
        self.number_of_neutral = None;
        self.largest_bond = None;
        self.atomic_sum = None;
        self.metal_component = None;
    }

    // MakeRestrictZone Function:
    // - Description: Makes a tile into a restricted tile. This function should only be used on empty tiles or bond zone
    //   tiles. Restrict zone tiles keep the data members associated with the tile that they are restricting so that
    //   the tile can be reverted when the timer expires.
    pub fn MakeRestrictZone(&mut self) {
        
        // Create an empty tile. When a tile is restricted, it cannot be used for two turns.
        self.state = TileState::Restricted;
        self.color = (228, 8, 10);

        self.restrict_counter = Some(2);
    }

    // MakeBondZone Function:
    // - Parameters:
    //   - symbol: The symbol to display within the bond zone.
    //   - bond_tile: The tile that the bond zone points to.
    // - Description: Applies the bond state to a tile. This function should only be used on empty tiles and restricted tiles.
    pub fn MakeBondZone(&mut self, symbol: String, bond_tile: (usize, usize)) {

        // Create an empty tile with bond state.
        self.ClearTile();
        self.symbol = symbol;
        self.state = TileState::Bond;

        // Tile that the bond zone belongs to.
        self.bond_tile = Some(bond_tile);
    }

    // MakeAtomZone Function:
    // - Parameters:
    //   - parent: Parent atom within the overall compound.
    //   - symbol: The symbol to display within the atom tile.
    //   - bond_number: The number of remaining bonds that the atom has.
    //   - is_metal: Boolean value flagged as true if the atom is a metal.
    //   - presedence: Turn number the atom was played on.
    // - Description: Applies the atom state to a tile. This function should be used only on bond zones.
    pub fn MakeAtomZone(&mut self, parent: (usize, usize), symbol: String, bond_number: u16, is_metal: bool, presedence: u8) {

        // Create an empty tile. Fill in atom related fields.
        self.ClearTile();
        self.state = TileState::Atom;
        self.symbol = symbol;

        // Information related to the atom in the space.
        self.parent = Some(parent);
        self.bond_number = Some(bond_number);
        self.is_metal = Some(is_metal);
        self.presedence = Some(presedence);
    }

    // MakeParentZone Function:
    // - Parameters:
    //   - tile: The tile that the atom itself is on.
    //   - symbol: The symbol to display within the atom tile.
    //   - bond_number: The number of remaining bonds that the atom has.
    //   - is_metal: Boolean value flagged as true if the atom is a metal.
    //   - metal_component: Option containing the name of the compound's metal component or nothing at all.
    //   - Sum: The atomic number of the atom used to initialize the atomic_sum field.
    //   - Presedence: Turn number the atom was played on.
    // - Description: Applies the parent state to a tile. This function should only be used when an atom is played on an
    //   empty tile. In this case, the atom becomes the parent of any atoms that bond to it.
    pub fn MakeParentZone(&mut self, tile: (usize, usize), symbol: String, bond_number: u16, is_metal: bool, presedence: u8, metal_component: Option<String> , sum: u16) {

        // Make the atom component of the parent atom.
        self.MakeAtomZone(tile, symbol, bond_number, is_metal, presedence);
        self.state = TileState::Parent;

        // Parent data members provided as parameters.
        match metal_component {
            
            Some(component) => self.metal_component = Some(component),
            None => self.metal_component = None,
        }
        self.atomic_sum = Some(sum);

        // Parent data members initialized when a parent is created.
        self.number_of_atoms = Some(1);
        self.number_of_neutral = Some(0);
        self.largest_bond = Some(0);
    }

    //-----------------------------------------------------------------------------------------------------------------------------
    // Getters and Setters
    //-----------------------------------------------------------------------------------------------------------------------------

    // GetState Function:
    // - Use within: All states.
    // - Description: Returns the state of the tile.
    pub fn GetState(&self) -> TileState {

        return self.state.clone();
    }

    // GetColor Function:
    // - Use within: All states.
    // - Description: Returns the color of the tile.
    pub fn GetColor(&self) -> (u8, u8, u8) {

        return self.color;
    }

    // SetColor Function:
    // - Use within: All states.
    // - Parameters:
    //   - r, g, & b: Numbers representing rgb color code.
    // - Description: Sets the color of a tile. 
    pub fn SetColor(&mut self, rgb: (u8, u8, u8)) {

        self.color = rgb;
    }

    // GetSymbol Function:
    // - Use within: All states.
    // - Description: Returns the symbol within the tile.
    pub fn GetSymbol(&self) -> String {

        return self.symbol.clone();
    }

    // IsBondTile Function:
    // - Use within: All states.
    // - Description: Returns true if a zone is a bond zone or restricted bond zone. Useful for updating an empty restricted zone
    //   to a restricted bond zone.
    pub fn IsBondTile(&self) -> bool {
        
        match &self.bond_tile {

            Some(value) => return true,
            None => return false,
        }
    }

    // BondSubState Function:
    // - Use within: Restricted state.
    // - Parameters:
    //   - Symbol: The symbol displayed within a tile.
    //   - bond_tile: The coordinates of the tile to bond to.
    // - Description: Changes the state that a restricted tile will change to when it resolves. (empty -> bond zone).
    pub fn BondSubState(&mut self, symbol: String, bond_tile: (usize, usize)) {

        // Change the symbol.
        self.symbol = symbol;

        // Add an atom to bond to.
        self.bond_tile = Some(bond_tile);
    }

    // EmptySubState Function:
    // - Use within: Restricted state.
    // - Parameters:
    //   - Symbol: The symbol displayed within a tile.
    //   - bond_tile: The coordinates of the tile to bond to.
    // - Description: Changes the state that a restricted tile will change to when it resolves. (bond zone -> empty).
    pub fn EmptySubState(&mut self) {

        // Store counter and reset the rest of the tile.
        let counter_temp = self.restrict_counter.unwrap();
        self.ClearTile();

        // Change state and add back the counter.
        self.state = TileState::Restricted;
        self.restrict_counter = Some(counter_temp);
        self.color = (228, 8, 10);
    }

    // DecrementRestrict Function:
    // - Use within: Restricted state.
    // - Description: Removes a counter from restrict_counter. If the restrict_counter expires, determines the previous state
    //   of the tile and reverts it.
    pub fn DecrementRestrict(&mut self) {

        // Remove one from restrict counter.
        self.restrict_counter = Some(self.restrict_counter.unwrap() - 1);

        // If the counter has expired, revert the state of the tile.
        if self.restrict_counter.unwrap() == 0 {

            self.restrict_counter = None;
            self.color = (255, 255, 255);

            match &self.bond_tile {

                Some(value) => self.state = TileState::Bond,
                None => self.state = TileState::Empty,
            }
        }
    }

    // GetBondTile Function:
    // - Use within: Bond state.
    // - Description: Returns the tile that a bond zone is attached to.
    pub fn GetBondTile(&self) -> (usize, usize) {

        return self.bond_tile.unwrap();
    }

    // IsMetal Function:
    // - Use Within: Atom or parent state.
    // - Description: Returns true if the atom at the given space is a metal. False otherwise.
    pub fn IsMetal(&self) -> bool {

        return self.is_metal.unwrap();
    }

    // BondAtom Function:
    // - Use within: Atom or parent state.
    // - Parameters: 
    //   - partner_bonds: Number of available bonds that the partner atom has.
    // - Description: Calculates the remaining number of bonds when two atoms are bonded together.
    // - Return: True if the atom becomes neutral. Fals if the atom has remaining bonds.
    pub fn BondAtom(&mut self, partner_bonds: u16) -> bool {

        if self.bond_number.unwrap() <= partner_bonds {

            // All bonds are removed from the atom.
            self.bond_number = Some(0);
            return true;
        } else {

            // There must be some number of remaining bonds.
            self.bond_number = Some(self.bond_number.unwrap() - partner_bonds);
            return false;
        }
    }

    // GetBondNumber Function:
    // - Use within: Atom or parent state.
    // - Description: Returns the number of remaining bonds that an atom has.
    pub fn GetBondNumber(&self) -> u16 {

        return self.bond_number.unwrap();
    }

    // GetParentTile Function:
    // - Use within: Atom or parent state.
    // - Description: Returns the tile that the compound parent atom is located on.
    pub fn GetParentTile(&self) -> (usize, usize) {

        return self.parent.unwrap();
    }

    // GetPresedence Function:
    // - Use within: Atom or parent state.
    // - Description: Returns the presedence of an atom. A lower number has higher presedence.
    pub fn GetPresedence(&self) -> u8 {

        return self.presedence.unwrap();
    }

    // GetCompoundMetal Function:
    // - Use within: Parent state.
    // - Description: Returns information related to the metal component of the compound.
    // - Return:
    //   - Exists: Boolean value flagged as true if the compound has a metal component. False otherwise.
    //   - Component: The symbol of the component. None if no component exists.
    pub fn GetCompoundMetal(&self) -> Option<String> {

        match &self.metal_component {

            Some(value) => return Some(value.clone()),
            None => return None,
        }
    }

    // UpdateCompound Function:
    // - Use within: Parent state.
    // - Parameters:
    //   - neutral_created: Neutral atoms created as a result of bonding.
    //   - atomic_number: Atomic number of the atom that was added to the compound.
    //   - bond_number: Size of the bond that occured.
    //   - metal: Symbol for metal is attached if the new atom is a metal.
    // - Description: Update parent data members related to the overall compound whenever a new atom is added.
    // - Return: True if the atom is neutral. False if the atom still is not neutral.
    pub fn UpdateCompound(&mut self, neutral_created: u8, atomic_number: u16, bonds_created: u16, metal: Option<String>) -> bool {

        // Increment values.
        self.number_of_atoms = Some(self.number_of_atoms.unwrap() + 1);
        self.number_of_neutral = Some(self.number_of_neutral.unwrap() + neutral_created);
        self.atomic_sum = Some(self.atomic_sum.unwrap() + atomic_number);
        
        // Add metal component if new atom was a metal.
        match metal {

            Some(value) => self.metal_component = Some(value),
            None => (), // Nothing happens in this case but Rust requires all cases to be covered.
        }

        // If bond_number is less than the current largest bond, replace the current largest bond.
        if bonds_created > self.largest_bond.unwrap() {

            self.largest_bond = Some(bonds_created);
        }

        // Trur is returned if all atoms within the compound are neutral.
        if self.number_of_atoms == self.number_of_neutral {

            return true;
        }

        return false;
    }

    // GetCompoundScore Function:
    // - Use within: Parent state.
    // - Description: Returns the score of a compound. Since only neutral atoms score, it is recomended to use this
    //   function only on neutral atoms.
    pub fn GetCompoundScore(&self) -> u16 {

        return self.atomic_sum.unwrap() * self.largest_bond.unwrap();
    }
}