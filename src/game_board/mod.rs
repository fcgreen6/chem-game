// External modules.
extern crate termion;
use termion::color::{Fg, Rgb};

// Std modules.
use core::array::from_fn;

// My modules.
mod board_tile;
use board_tile::{BoardTile, TileState};
mod color_picker;
use color_picker::ColorPicker;

// Struct to return values easily.
pub struct BoardStatus {
    
    pub error_message: Option<String>,
    pub points: Option<u16>
}

pub struct GameBoard {

    // Two dimentional of board tiles.
    tile_array: [[BoardTile; 6]; 6],
    color_picker: ColorPicker
}

impl Default for GameBoard {
    fn default() -> Self {
        GameBoard {

            // Put all empty spaces into the array.
            tile_array: from_fn(|_| from_fn(|_| BoardTile::default())),
            color_picker: ColorPicker::default()
        }
    }
}

impl GameBoard {

    // Bond Function:
    // Parameters:
    // - tile: Board tile that the atom is being played on.
    // - symbol: String which is displayed inside of a board tile.
    // - bond_number: Number of bonds that the atom can form.
    // - presedence: Value which determines who gets presedence over a freed bond zone.
    // - atomic_number: Atomic number of the atom from the periodic table.
    // Description: Turns empty zones into parent zones. Turns bond zones into atom zones.
    // Return: BoardStatus struct.
    pub fn Bond(&mut self, tile: (usize, usize), symbol: String, bond_number: u16, is_metal: bool, presedence: u8, atomic_number: u16) -> BoardStatus {

        // Get the state of the selected tile.
        let tile_state: TileState;
        {
            let selected_tile = self.GetTile(tile);
            tile_state = selected_tile.GetState();
        }

        if tile_state == TileState::Empty {

            // If bond happens on an empty space, the played atom becomes a parent atom with a unique color.
            let compound_color: (u8, u8, u8);
            {
                compound_color = self.color_picker.GetColor();
            }

            let selected_tile = self.GetTile(tile);

            if is_metal {

                // Make the parent zone with metal trait.
                selected_tile.MakeParentZone(tile, symbol.clone(), bond_number, is_metal, presedence, Some(symbol.clone()), atomic_number);
            }
            else {

                // Make the parent zone without metal trait.
                selected_tile.MakeParentZone(tile, symbol.clone(), bond_number, is_metal, presedence, None, atomic_number);
            }

            selected_tile.SetColor(compound_color);

            // Return restrict tile error message.
            return BoardStatus {

                error_message: None,
                points: None
            }
        }
        else if tile_state == TileState::Bond {

            // Get the coordinates of the atom being bonded to.
            let bond_tile_coords: (usize, usize);
            let bond_tile_bonds: u16;
            {
                // Get coordinates of the bond atom.
                {
                    let selected_tile = self.GetTile(tile);
                    bond_tile_coords = selected_tile.GetBondTile();
                }
                
                // Get number of bonds the bond atom has.
                {
                    let bond_tile = self.GetTile(bond_tile_coords);
                    bond_tile_bonds = bond_tile.GetBondNumber();
                }
            }

            // Get the compound parent from the atom being bonded to.
            let parent_tile_coords: (usize, usize);
            {
                let bond_tile = self.GetTile(bond_tile_coords);
                parent_tile_coords = bond_tile.GetParentTile();
            }

            // Get color and metalic property from compound parent.
            let compound_metal: Option<String>;
            let compound_color: (u8, u8, u8);
            let mut add_metal: Option<String> = None;
            {
                let parent_tile = self.GetTile(parent_tile_coords);
                compound_metal = parent_tile.GetCompoundMetal();
                compound_color = parent_tile.GetColor();
            }

            // If the compound is a metal, extra conditions must be met.
            if is_metal {

                match compound_metal {

                    Some(metal) => {

                        // Metals within the same compound must have the same type.
                        if metal == symbol {

                            let bond_tile = self.GetTile(bond_tile_coords);

                            // Cannot bond directly to another metal.
                            if bond_tile.IsMetal() {

                                // Direct bond error.
                                return BoardStatus {

                                    error_message: Some(String::from("Error: Metals cannot bond with each other.")),
                                    points: None
                                };
                            }
                            else {

                                add_metal = None;
                            }
                        }
                        else {

                            // Different type error.
                            return BoardStatus {

                                error_message: Some(String::from("Error: Compounds can only contain one type of metal.")),
                                points: None
                            };
                        }
                    },
                    None => add_metal = Some(symbol.clone()), // The metal becomes the compound metal.
                };
            }

            // Officially bond the atom to the compound.
            {
                let selected_tile = self.GetTile(tile);
                selected_tile.MakeAtomZone(parent_tile_coords, symbol, bond_number, is_metal, presedence);
                selected_tile.SetColor(compound_color);
            }

            let mut neutral_atoms: u8 = 0;
            let mut bonds_created: u16 = bond_number;

            // Atoms are bonded to each other with the BondAtom function.
            // This function must be called on both atoms.
            {
                let selected_tile = self.GetTile(tile);
                
                if selected_tile.BondAtom(bond_tile_bonds) {

                    neutral_atoms = neutral_atoms + 1;
                }

                bonds_created = bonds_created - selected_tile.GetBondNumber();
            }
            
            if self.GetTile(bond_tile_coords).BondAtom(bond_number) {

                neutral_atoms = neutral_atoms + 1;

                // Bond zones are removed in case the atom becomes neutral.
                // If the atom is not neutral, they will be reset by the UpdateBondZones function.
                self.RemoveBondZones(bond_tile_coords);
            }

            {
                let parent_tile = self.GetTile(parent_tile_coords);

                // Update the overall compound. UpdateCompound returns true if the coumpound is neutralized.
                if parent_tile.UpdateCompound(neutral_atoms, atomic_number, bonds_created, add_metal) {

                    return BoardStatus {

                        error_message: None,
                        points: Some(parent_tile.GetCompoundScore())
                    }
                }
            }

            return BoardStatus {

                error_message: None,
                points: None
            }
        }
        else if tile_state == TileState::Restricted {

            // Return restrict tile error message.
            return BoardStatus {

                error_message: Some(String::from("Error: Selected space is Restricted.")),
                points: None
            }
        }
        else {

            // Return occupied tile error message.
            return BoardStatus {

                error_message: Some(String::from("Error: Selected space is occupied.")),
                points: None
            }
        }
    }

    // RemoveBondZones Function:
    // Parameters:
    // - coords: Coordinates of the atom to remove bond zones from.
    // Description: Removes all bond zones around an atom.
    fn RemoveBondZones(&mut self, coords: (usize, usize)) {

        self.RemoveBondZone(GameBoard::GetUpTile(coords));
        self.RemoveBondZone(GameBoard::GetDownTile(coords));
        self.RemoveBondZone(GameBoard::GetLeftTile(coords));
        self.RemoveBondZone(GameBoard::GetRightTile(coords));
    }

    // RemoveBondZone Function:
    // Parameters:
    // - option: A set of coordinates that may not exist.
    // Description: Removes the bond state from an atom.
    fn RemoveBondZone(&mut self, option: Option<(usize, usize)>) {

        // There is not always a tile around the given coords. For example, if the tile is at the top.
        // If there is not a tile, nothing happens.
        match option {

            Some(tile) => {
                    
                let remove_tile = self.GetTile(tile);

                if remove_tile.GetState() == TileState::Bond {

                    remove_tile.ClearTile();
                }
                else if remove_tile.GetState() == TileState::Restricted {

                    // Check if the substate is a bond zone.
                    if remove_tile.IsBondTile() {

                        // Convert the substate of the tile to empty.
                        remove_tile.EmptySubState();
                    }
                }
            },
            None => (),
        }
    }

    // Restrict Function:
    // Parameters: 
    // - coords_1 and coords_2: Coordinates of the tiles to resrict.
    // Description: Changes the state of two tiles to restriced.
    pub fn Restrict(&mut self, coords_1: (usize, usize), coords_2: (usize, usize)) -> BoardStatus {

        // Test if coords are valid to restrict.
        match self.TestRestrict(coords_1) {

            Some(status) => return status,
            None => (),
        }
    
        // Test if coords are valid to restrict.
        match self.TestRestrict(coords_2) {

            Some(status) => return status,
            None => (),
        }

        // Restrict the given coordinates.
        self.ApplyRestrict(coords_1);
        self.ApplyRestrict(coords_2);

        return BoardStatus {

            error_message: None,
            points: None
        }
    }

    // TestRestrict Function:
    // Parameters:
    // - coords: The coordinates of the tile to prepare for the restrict function.
    // Description: Checks if a tile is an atom, parent or restricted state. 
    fn TestRestrict(&mut self, coords: (usize, usize)) -> Option<BoardStatus> {

        let restrict_tile = self.GetTile(coords);

        // Check for an atom or parent at given coords.
        if (restrict_tile.GetState() == TileState::Atom) || (restrict_tile.GetState() == TileState::Parent) {

            return Some(BoardStatus {

                error_message: Some(String::from("Error: Cannot restrict on top of atoms.")),
                points: None
            })
        }
        else if restrict_tile.GetState() == TileState::Restricted {

            return Some(BoardStatus {

                error_message: Some(String::from("Error: Zone is already restricted.")),
                points: None
            })
        }

        return None;
    }

    // ApplyRestrict Function:
    // Parameters:
    // - coords: The coords of the atom to chage into a restricred syay
    fn ApplyRestrict(&mut self, coords: (usize, usize)) {

        let restrict_tile = self.GetTile(coords);
        restrict_tile.MakeRestrictZone();
    }

    // Destroy Function:
    // Parameters:
    // - coords: The coordinates of the atim to destroy.
    // Description: Finds the parent of the provided atom. Destroys all elements with that parent on the game board.
    pub fn Destroy(&mut self, coords: (usize, usize)) -> BoardStatus {
        
        // Get coordinates of the parent tile.
        let parent_tile: (usize, usize);
        {
            let specified_atom = self.GetTile(coords);

            if (specified_atom.GetState() != TileState::Parent) && (specified_atom.GetState() != TileState::Atom) {

                return BoardStatus {

                    error_message: Some(String::from("Error: No compound specified.")),
                    points: None
                }
            }

            parent_tile = specified_atom.GetParentTile();
        }

        // Go to every tile. If it has the specified parent tile, remove all bond zones and clear the tile.
        for i in 0..6 {

            for j in 0..6 {

                if (self.GetTile((i, j)).GetState() == TileState::Atom) || (self.GetTile((i, j)).GetState() == TileState::Parent) {

                    if self.GetTile((i, j)).GetParentTile() == parent_tile {

                        self.RemoveBondZones((i, j));
                        self.GetTile((i, j)).ClearTile();
                    }
                }
            }
        }

        self.UpdateBondZones();

        return BoardStatus {

            error_message: None,
            points: None
        }
    }

    // EndTurn Function:
    // Description: Resets bondzones that changed earlier in the turn.
    pub fn EndTurn(&mut self) {

        for i in 0..6 {

            for j in 0..6 {

                if self.GetTile((i, j)).GetState() == TileState::Restricted {

                    self.GetTile((i, j)).DecrementRestrict();
                }
            }
        }

        self.UpdateBondZones();
    }

    // UpdateBondZone Functions:
    // Description: Goes to each tile. If the tile is an non-neutral atom determine how 
    // bond zones are organized based on presedence.
    fn UpdateBondZones(&mut self) {

        for i in 0..6 {

            for j in 0..6 {

                // Get state of tile.
                let curr_state: TileState;
                {
                    let curr_tile = self.GetTile((i, j));
                    curr_state = curr_tile.GetState();
                }

                // Check for atom or parent state.
                if (curr_state == TileState::Atom) || (curr_state == TileState::Parent) {

                    let available_bonds: u16;
                    {
                        let curr_tile = self.GetTile((i, j));
                        available_bonds = curr_tile.GetBondNumber();
                    }

                    // If the atom has bonds, check the bond zones.
                    if available_bonds != 0 {

                        self.UpdateBondZone(GameBoard::GetUpTile((i, j)), (i, j), String::from("vv"));
                        self.UpdateBondZone(GameBoard::GetDownTile((i, j)), (i, j), String::from("^^"));
                        self.UpdateBondZone(GameBoard::GetLeftTile((i, j)), (i, j), String::from(">>"));
                        self.UpdateBondZone(GameBoard::GetRightTile((i, j)), (i, j), String::from("<<"));
                    }
                }
            }
        }
    }

    // UpdateBondZone Function:
    // Parameters:
    // - option: Option variable which holds neighbor coordinates if the neighbor exists
    // - coords: Coords for the atom we are testing for presedence.
    // - symbol: Symbol to be used by the bond zone.
    // Description: Adjusts the bond zones of an adjacent atom.
    fn UpdateBondZone(&mut self, option: Option<(usize, usize)>, coords: (usize, usize), symbol: String) {

        // There is not always a tile around the given coords. For example, if the tile is at the top.
        // If there is not a tile, nothing happens.
        match option {

            Some(neighbor_coords) => {
                    
                // Get state of neigboring atom.
                let neighbor_state: TileState;
                {
                    let neighbor = self.GetTile(neighbor_coords);
                    neighbor_state = neighbor.GetState();
                }

                // If in bond state, check that the neighbor atom points to the test atom. If it does not, test for presedence.
                if neighbor_state == TileState::Bond {

                    let bond_atom_coords: (usize, usize);
                    {
                        let neighbor = self.GetTile(neighbor_coords);
                        bond_atom_coords = neighbor.GetBondTile();
                    }

                    if coords != bond_atom_coords {

                        let test_atom_presedence: u8;
                        let bonded_atom_presedence: u8;

                        {
                            let test_atom = self.GetTile(coords);
                            test_atom_presedence = test_atom.GetPresedence();
                        }

                        {
                            let bonded_atom = self.GetTile(bond_atom_coords);
                            bonded_atom_presedence = bonded_atom.GetPresedence();
                        }

                        // Presedence of neighbor atom is greater than the atom already bonded.
                        if test_atom_presedence < bonded_atom_presedence {

                            let neighbor_atom = self.GetTile(neighbor_coords);
                            neighbor_atom.MakeBondZone(symbol, coords);
                        }
                    }
                }
                else if neighbor_state == TileState::Empty {

                    let neighbor = self.GetTile(neighbor_coords);
                    neighbor.MakeBondZone(symbol, coords);
                }
                else if neighbor_state == TileState::Restricted {

                    // Check that the restricted tile has bond tile substate.
                    if self.GetTile(neighbor_coords).IsBondTile() {
                        
                        let bond_atom_coords: (usize, usize);
                        {
                            let neighbor = self.GetTile(neighbor_coords);
                            bond_atom_coords = neighbor.GetBondTile();
                        }

                        // Use presedence to determine the structure of bond zones within restricted tiles.
                        if coords != bond_atom_coords {

                            let test_atom_presedence: u8;
                            let bonded_atom_presedence: u8;

                            {
                                let test_atom = self.GetTile(coords);
                                test_atom_presedence = test_atom.GetPresedence();
                            }

                            {
                                let bonded_atom = self.GetTile(bond_atom_coords);
                                bonded_atom_presedence = bonded_atom.GetPresedence();
                            }

                            if test_atom_presedence < bonded_atom_presedence {

                                let neighbor_atom = self.GetTile(neighbor_coords);
                                neighbor_atom.BondSubState(symbol, coords);
                            }
                        }
                    }
                    else {

                        // Presedence of neighbor is greater than the atom already bonded.
                        let neighbor = self.GetTile(neighbor_coords);
                        neighbor.BondSubState(symbol, coords);
                    }
                }
            },
            None => (),
        }
    }

    // PrintTile Function:
    // Parameters:
    // - coords: Coordinates to the tile to print.
    // Description: Prints an individual tile on the gameboard.
    pub fn PrintTile(&mut self, coords: (usize, usize)) {

        // Get the tile and its color.
        let tile = self.GetTile(coords);
        let col = tile.GetColor();

        // Set up TUI colors.
        let fmt_col = Fg(Rgb(col.0, col.1, col.2));
        let white = Fg(Rgb(255, 255, 255));

        print!("{}[{}{}{}]{}", fmt_col, white, tile.GetSymbol(), fmt_col, white);
    }

    // GetTile Function:
    // Parameters:
    // - coords: The coordinates of a tile.
    // Description: Gets a tile object from the board.
    // Return: Mutable refference to a BoardTile object.
    fn GetTile(&mut self, coords: (usize, usize)) -> &mut BoardTile {

        return &mut self.tile_array[coords.0][coords.1];
    }

    // FindBondZone Function:
    // Description: Finds and returns the first bond zone starting from (0, 0).
    pub fn FindBondZone(&mut self) -> Option<(usize, usize)> {

        for i in 0..6 {

            for j in 0..6 {

                let curr_state: TileState;
                {
                    let curr_tile = self.GetTile((i, j));
                    curr_state = curr_tile.GetState();
                }

                if curr_state == TileState::Bond {

                    // Return the coordinates of tile found.
                    return Some((i, j));
                }
            }
        }

        // No tile found.
        return None;
    }

    // FindEmptyZone Function:
    // Description: Finds and returns the first empty zone starting from (0, 0).
    pub fn FindEmptyZone(&mut self) -> (usize, usize) {

        for i in 0..6 {

            for j in 0..6 {

                let curr_state: TileState;
                {
                    let curr_tile = self.GetTile((i, j));
                    curr_state = curr_tile.GetState();
                }

                if curr_state == TileState::Empty {

                    // Return the first empty space found.
                    return (i, j);
                }
            }
        }

        // This case should never hapen.
        return (0, 0);
    }

    // GetUpTile, GetDownTile, GetLeftTile, and GetRightTile:
    // Description: Given a pair of coordinates, return an adjacent tile if it exists.
    // -----------------------------------------------------------------------------------------------------------------
    fn GetUpTile(coords: (usize, usize)) -> Option<(usize, usize)> {

        if coords.1 == 0 {

            return None;
        }

        return Some((coords.0, coords.1 - 1));
    }

    fn GetDownTile(coords: (usize, usize)) -> Option<(usize, usize)> {

        if coords.1 == 5 {

            return None;
        }

        return Some((coords.0, coords.1 + 1));
    }

    fn GetLeftTile(coords: (usize, usize)) -> Option<(usize, usize)> {

        if coords.0 == 0 {

            return None;
        }

        return Some((coords.0 - 1, coords.1));
    }

    fn GetRightTile(coords: (usize, usize)) -> Option<(usize, usize)> {

        if coords.0 == 5 {

            return None;
        }

        return Some((coords.0 + 1, coords.1));
    }
    // -----------------------------------------------------------------------------------------------------------------
}