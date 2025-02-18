// Termion used as TUI and for colored tiles.
extern crate termion;
use termion::color::{Fg, Rgb};
use termion::clear;

// Rng crate.
extern crate rand;
use rand::Rng;

// Used for user input.
use std::io::stdin;

// Class representing the game board.
mod game_board;
use game_board::{GameBoard, BoardStatus};

// Class representing the decks of cards used by the game.
mod card_deck;
use card_deck::{CardDeck, Card};

// Class used to record events that happen in the game.
mod action_log;
use action_log::ActionLog;

fn main() {

    // User input variable.
    let mut user_input = String::new();

    // Starting screen.
    {
        let mut board: GameBoard = Default::default();
        let mut log: ActionLog = Default::default();

        let mut invalid: bool = true;
        while invalid {

            // Refresh the screen and get user input.
            //-----------------------------------------------------------------------
            println!("{}", clear::All);

            PrintScore(0, 0);
            PrintGameBoard(&mut board, &log);
            PrintEmptyHand();
 
            println!("Enter the command \"start\" to begin a new game.");

            user_input = String::new();
            stdin().read_line(&mut user_input);
            //-----------------------------------------------------------------------

            // Check that the start command was used properly.
            let input_fields = MatchCommand(&mut user_input);
            if (input_fields.0 == Some(String::from("start"))) && (input_fields.1.0 == None) {

                invalid = false;
            }
            else {

                // Log error if start command is not used.
                log.PushAction(String::from("Error: Command not recognised."), true);
            }
        }
    }

    // Game loop.
    let mut quit: bool = false;
    while !quit {
        
        // Initialize classes.
        let mut board: GameBoard = Default::default();
        let mut log: ActionLog = Default::default();

        // Initialize decks.
        let mut player_deck: CardDeck = Default::default();
        let mut computer_deck: CardDeck = Default::default();

        // Draw four cards to each hand.
        for i in 0..4 {
            
            player_deck.AddToHand();
            computer_deck.AddToHand();
        }

        // Score variables.
        let mut player_score: u16 = 0;
        let mut computer_score: u16 = 0;

        // Ability variables. For simplicity, the computer does not use abilities.
        let mut player_evolve: u8 = 1;
        let mut player_destroy: u8 = 1;
        let mut player_restrict: u8 = 2;

        // Choose who goes first.
        let mut turn_number = 1;
        let mut player_turn = false;
        {
            // Initialize random number generator.
            let mut random = rand::thread_rng();

            // Generate a random number to determine who moves first.
            let coin: u8 = random.gen_range(0..2);
            if coin == 0 {

                player_turn = true;
                log.PushAction(String::from("Player is going first."), false);
            }
            else {

                log.PushAction(String::from("Computer is going first."), false);
            }
        }

        // There are sixteen turns in each game. Eight for each player.
        for i in 0..16 {

            // Player turn conditional.
            if player_turn {

                player_deck.AddToHand();

                {
                    // While loop for ability phase.
                    let mut invalid: bool = true;
                    while invalid {
                        
                        // Refresh the screen and get user input.
                        //-----------------------------------------------------------------------
                        println!("{}", clear::All);

                        PrintScore(player_score, computer_score);
                        PrintGameBoard(&mut board, &log);
                        PrintHand(&mut player_deck, player_evolve, player_destroy, player_restrict);

                        println!("Ability Phase. Type \"pass\" to skip your ability phase.");

                        user_input = String::new();
                        stdin().read_line(&mut user_input);
                        //-----------------------------------------------------------------------

                        // Outer conditional checks what command has been used by the user.
                        let input_fields = MatchCommand(&mut user_input);
                        if (input_fields.0 == Some(String::from("restrict"))) && (player_restrict > 0) {

                            // Check that user used the correct number of imput fields.
                            if (input_fields.1.0 != None) && (input_fields.1.1 != None) && (input_fields.1.2 == None) {

                                // Get the two coordinates used by the restrict function.
                                let coords_1 = input_fields.1.0.unwrap();
                                let coords_2 = input_fields.1.1.unwrap();

                                // Check that coords can be created for the spaces provided by user.
                                if CreateCoords(coords_1.clone()).0 && CreateCoords(coords_2.clone()).0 {

                                    // Create restrict tiles on the board.
                                    let status = board.Restrict(CreateCoords(coords_1).1, CreateCoords(coords_2).1);

                                    if status.error_message != None {

                                        // Log any error that occurs within the restrict function.
                                        log.PushAction(status.error_message.unwrap(), true);
                                    }
                                    else {

                                        // A valid command was used, so invalid flag is set to false.
                                        invalid = false;
                                        log.PushAction(String::from("Player used restrict ability."), false);
                                        player_restrict -= 1;
                                    }
                                }
                                else {

                                    log.PushAction(String::from("Error: Invalid parameters."), true);
                                }
                            }
                            else {

                                log.PushAction(String::from("Error: Invalid parameters."), true);
                            }
                        } else if (input_fields.0 == Some(String::from("destroy"))) && (player_destroy > 0) {

                            // Check only one parameter exists for destroy function.
                            if (input_fields.1.0 != None) && (input_fields.1.1 == None) {

                                // Unwrap the coords from user input.
                                let coords = input_fields.1.0.unwrap();
                                if (CreateCoords(coords.clone()).0) {

                                    // Destroy with the given coords.
                                    let status = board.Destroy(CreateCoords(coords.clone()).1);

                                    if status.error_message != None {

                                        // Log any error associated with the destroy function.
                                        log.PushAction(status.error_message.unwrap(), true);
                                    }
                                    else {

                                        // A valid command was used, so invalid flag is set to false.
                                        invalid = false;
                                        log.PushAction(String::from("Player has destroyed a compound."), false);
                                        player_destroy -= 1;
                                    }
                                }
                                else {

                                    log.PushAction(String::from("Error: Invalid parameters."), true);
                                } 
                            }
                            else {

                                log.PushAction(String::from("Error: Invalid parameters."), true);
                            }
                        } else if (input_fields.0 == Some(String::from("evolve"))) && (player_evolve > 0) {

                            // Check that the fields for evolve are satisfied.
                            if (input_fields.1.0 != None) && (input_fields.1.1 == None) {

                                // If this is true, the evolution was successful.
                                if player_deck.EvolveCard(input_fields.1.0.unwrap()) {

                                    // A valid command was used, so invalid flag is set to false.
                                    invalid = false;
                                    log.PushAction(String::from("Player has evolved an atom."), false);
                                    player_evolve -= 1;
                                }
                                else {

                                    // Else the parameters are invalid.
                                    log.PushAction(String::from("Error: Invalid parameters."), true);
                                }
                            }
                            else {

                                log.PushAction(String::from("Error: Invalid parameters."), true);
                            }
                        } else if input_fields.0 == Some(String::from("pass")) {

                            // A valid command was used, so invalid flag is set to false.
                            invalid = false;
                            log.PushAction(String::from("Player has passed their ability phase."), false);
                        } else {

                            log.PushAction(String::from("Error: Invalid command."), true);
                        }
                    }
                }

                {
                    // While loop for main phase.
                    let mut invalid: bool = true;
                    while invalid {

                        // Refresh the screen and get user input.
                        //-----------------------------------------------------------------------
                        println!("{}", clear::All);

                        PrintScore(player_score, computer_score);
                        PrintGameBoard(&mut board, &log);
                        PrintHand(&mut player_deck, player_evolve, player_destroy, player_restrict);

                        println!("Main Phase. Use the play command to play an atom.");

                        user_input = String::new();
                        stdin().read_line(&mut user_input);
                        //-----------------------------------------------------------------------

                        // Atoms must be played with the play command.
                        let input_fields = MatchCommand(&mut user_input);
                        if input_fields.0 == Some(String::from("play")) {

                            // Verify fields used for play command.
                            if (input_fields.1.0 != None) && (input_fields.1.1 != None) && (input_fields.1.2 == None) {

                                // Get the parameters from user input.
                                let name = input_fields.1.0.clone().unwrap();
                                let coords = input_fields.1.1.clone().unwrap();

                                let card_option = player_deck.GetCard(name.clone());
                                let coords_return = CreateCoords(coords); // Numeric coordinates, not tile coordinates.

                                // card_option is an option representing a card for a given symbol or nothing.
                                // coords_return.0 gets the boolean flag for the CreateCoords function.
                                if (card_option != None) && coords_return.0 {

                                    // If the coords and card are valid, attempt to bond.
                                    let card: Card = card_option.unwrap();
                                    let status: BoardStatus = board.Bond(coords_return.1, card.symbol, card.number_of_bonds, card.is_metal, turn_number, card.atomic_number);

                                    // Check status of the bond operation.
                                    if status.error_message != None {

                                        log.PushAction(status.error_message.unwrap(), true);
                                    }
                                    else if status.points != None {

                                        // If the points field is specified, the bond was successful and a neutral compound was created.
                                        log.PushAction(String::from("Player completed a compound!"), false);
                                        player_deck.RemoveCard(name.clone());
                                        player_score += status.points.unwrap();
                                        invalid = false;
                                    }
                                    else {

                                        // Bond was successful. Compound is not yet neutral.
                                        player_deck.RemoveCard(name.clone());
                                        invalid = false;
                                    }
                                }
                                else {

                                    log.PushAction(String::from("Error: Invalid parameters."), true);
                                }
                            }
                            else {

                                log.PushAction(String::from("Error: Invalid parameters."), true);
                            }
                        }
                        else {

                            log.PushAction(String::from("Error: Invalid command."), true);
                        }
                    }
                }

                // Change turn.
                player_turn = false;
                board.EndTurn();
            }
            else {

                computer_deck.AddToHand();
                
                /*
                Computer Algorithm:
                - Description: Starting from (0, 0), find the first bond zone. Attempt to bond all cards in 
                  hand to this zone. Only metal atoms are unable to bond within certain bond zones. Since there are only
                  four metals in the deck and five cards in hand, one atom is gaurenteed to bond.
                  If there are no bond zones, play on the first empty tile starting from (0, 0).
                */
                let bond_zone_option = board.FindBondZone();
                if bond_zone_option != None {

                    let bond_zone_coords = bond_zone_option.unwrap();

                    // Iterate over the five cards in hand.
                    for i in 0..5 {

                        // Attempt to bond with card at given index.
                        let card = computer_deck.GetFromIndex(i);
                        let status: BoardStatus = board.Bond(bond_zone_coords, card.symbol, card.number_of_bonds, card.is_metal, turn_number, card.atomic_number);

                        // If there is no error with the bond operation, break out of the loop.
                        if status.error_message == None {

                            if status.points != None {

                                log.PushAction(String::from("Computer completed a compound!"), false);
                                computer_score += status.points.unwrap();
                            }

                            computer_deck.RemoveCard(card.name);
                            break;
                        }
                    } 
                }
                else {

                    // If there are no bond zones, play on an empty tile.
                    let empty_tile = board.FindEmptyZone();
                    let card = computer_deck.GetFromIndex(0);

                    // Since the tile is empty, bonding is gaurenteed.
                    board.Bond(empty_tile, card.symbol, card.number_of_bonds, card.is_metal, turn_number, card.atomic_number);
                    computer_deck.RemoveCard(card.name);
                }

                // Change turn.
                player_turn = true;
                board.EndTurn();
            }
        }

        log.PushAction(String::from("Game over!"), false);

        let mut invalid = true;
        while invalid {
            
            // Draw so that board is printed correctly.
            player_deck.AddToHand();
            computer_deck.AddToHand();

            // Refresh the screen and get user input.
            //-----------------------------------------------------------------------
            println!("{}", clear::All);

            PrintScore(player_score, computer_score);
            PrintGameBoard(&mut board, &log);
            PrintHand(&mut player_deck, player_evolve, player_destroy, player_restrict);

            if player_score > computer_score {

                println!("Player wins! Type \"start\" to play again. Type \"quit\" to exit the game.");
            } else if player_score > computer_score {

                println!("Computer wins! Type \"start\" to play again. Type \"quit\" to exit the game.");
            }
            else {

                println!("Its a tie! Type \"start\" to play again. Type \"quit\" to exit the game.");
            }

            user_input = String::new();
            stdin().read_line(&mut user_input);
            //-----------------------------------------------------------------------

            // Condional to sort through quit and start commands.
            let input_fields = MatchCommand(&mut user_input);
            if (input_fields.0 == Some(String::from("quit"))) && (input_fields.1.0 == None) {

                quit = true;
                invalid = false;
            } else if (input_fields.0 == Some(String::from("start"))) && (input_fields.1.0 == None) {

                invalid = false;
            } else {

                log.PushAction(String::from("Error: Invalid command."), true);
            }
        }
    }
}

// PrintScore Function:
// Parameters:
// - player: Player's score in the game.
// - player: Computer's score in the game.
// Description: Prints the portion of the game board that contains score.
fn PrintScore(player: u16, computer: u16) {

    // Need to add some spaces so that the text does not shift when there is a smaller number.
    let spaces: String;
    if player < 10 {

        spaces = String::from("  ");
    } else if player < 100 {

        spaces = String::from(" ");
    } else {

        spaces = String::from("");
    }

    println!("----                                               ----  | Player Score: {}{}       Computer Score: {}", player, spaces, computer);
    println!("|H |       A   B   C   D   E   F                   |He|  -------------------------------------------------------")
}

// PrintGameBoard Function:
// Parameters:
// - game_board: Class representing the game board.
// - action_log: Class holding logged events.
// Description: Prints the portion of the game board that board tiles and actions.
fn PrintGameBoard(game_board: &mut GameBoard, action_log: &ActionLog) {

    // Row 1.
    print!("|1 |     1");
    for i in 0..6 {
        game_board.PrintTile((i, 0));
    }
    print!("                 |2 |  | Action Log:\n");

    // Row 2.
    print!("-------  2");
    for i in 0..6 {
        game_board.PrintTile((i, 1));
    }
    print!("  -------------------  | ");
    action_log.PrintIndex(0);
    print!("\n");

    // Row 3.
    print!("|Li|Be|  3");
    for i in 0..6 {
        game_board.PrintTile((i, 2));
    }
    print!("  |B |C |N |O |F |Ne|  | ");
    action_log.PrintIndex(1);
    print!("\n");

    // Row 4.
    print!("|3 |4 |  4");
    for i in 0..6 {
        game_board.PrintTile((i, 3));
    }
    print!("  |5 |6 |7 |8 |9 |10|  | ");
    action_log.PrintIndex(2);
    print!("\n");

    // Row 5.
    print!("-------  5");
    for i in 0..6 {
        game_board.PrintTile((i, 4));
    }
    print!("  -------------------  | ");
    action_log.PrintIndex(3);
    print!("\n");

    // Row 6.
    print!("|Na|Mg|  6");
    for i in 0..6 {
        game_board.PrintTile((i, 5));
    }
    print!("  |Al|Si|P |S |Cl|Ar|  | ");
    action_log.PrintIndex(4);
    print!("\n");

    println!("|11|12|                             |13|14|15|16|17|18|  -------------------------------------------------------")
}

// PrintHand Function:
// Parameters:
// - hand: Class containing the player's hand.
// - evolve, destroy, and restrict: Variables holding the counters for abities.
// Description: Prints the part of the game board which contains player hand and ability counters.
fn PrintHand(hand: &mut CardDeck, evolve: u8, destroy: u8, restrict: u8) {

    println!("-------------------------------------------------------  | Player Hand:            Remaining Abilities:");

    print!("|K |Ca|Sc|Ti|V |Cr|Mn|Fe|Co|Ni|Cu|Zn|Ga|Ge|As|Se|Br|Kr|  | ");
    hand.PrintUpperHand();
    print!("        Destroy x {}\n", destroy);

    print!("|19|20|21|22|23|24|25|26|27|28|29|30|31|32|33|34|35|36|  | ");
    hand.PrintLowerHand();
    print!("        Evolve x {}\n", evolve);

    print!("-------------------------------------------------------  | ");
    print!("                        Restrict x {}\n", restrict);

    println!("________________________________________________________________________________________________________________");
}

// PrintEmptyHand Function:
// Description: Prints a blank hand for when the game is first launced.
fn PrintEmptyHand() {

    println!("-------------------------------------------------------  | Player Hand:            Remaining Abilities:");
    println!("|K |Ca|Sc|Ti|V |Cr|Mn|Fe|Co|Ni|Cu|Zn|Ga|Ge|As|Se|Br|Kr|  |");
    println!("|19|20|21|22|23|24|25|26|27|28|29|30|31|32|33|34|35|36|  |");
    println!("-------------------------------------------------------  |");
    println!("________________________________________________________________________________________________________________");
}

// CreateCoords Function:
// Parameters:
// - tile: String symbol for a tile supplied by the user.
// Description: Converts the string coordinates given by the user to a numeric representation.
// Return: Tuple containing boolean flag and a pair of coordinates.
fn CreateCoords(tile: String) -> (bool, (usize, usize)) {

    // Coordinate length is two.
    if tile.len() > 2 {

        return (false, (0, 0));
    }

    let x: usize;
    let y: usize;

    // Convert the first index to array coordinates,
    match tile.chars().nth(0) {

        Some('A') | Some('a') => x = 0,
        Some('B') | Some('b') => x = 1,
        Some('C') | Some('c') => x = 2,
        Some('D') | Some('d') => x = 3,
        Some('E') | Some('e') => x = 4,
        Some('F') | Some('f') => x = 5,
        _ => return (false, (0, 0)),
    };

    // Convert the second index to array coordinates.
    match tile.chars().nth(1) {

        Some('1') => y = 0,
        Some('2') => y = 1,
        Some('3') => y = 2,
        Some('4') => y = 3,
        Some('5') => y = 4,
        Some('6') => y = 5,
        _ => return (false, (0, 0)),
    };

    // Return boolean flag for valid coordinates and the created coords.
    return (true, (x, y));
}

// MatchCommand Function:
// Parameters:
// - user_input: The user input to be parsed.
// Description: Sorts the prompt written by user into important elements.
// Return: (command, (parameter, parameter, parameter)). Inside of option because they may not exist.
fn MatchCommand(user_input: &mut String) -> (Option<String>, (Option<String>, Option<String>, Option<String>)) {

    // Check that enter was not accidentally pressed.
    if *user_input == String::from("\n") {

        return (None, (None, None, None));
    }

    // Separate elements of input.
    let command_vector: Vec<&str> = user_input.split_whitespace().collect();

    let command_keyword: Option<String> = VerifyCommand(command_vector[0]);

    let mut parameter_1: Option<String> = None;
    let mut parameter_2: Option<String> = None;
    let mut parameter_3: Option<String> = None;

    
    // Categorize the input.
    if command_vector.len() > 1 {

        parameter_1 = Some(String::from(command_vector[1]));
    }

    if command_vector.len() > 2 {

        parameter_2 = Some(String::from(command_vector[2]));
    }

    if command_vector.len() > 3 {

        parameter_3 = Some(String::from(command_vector[3]));
    }

    return (command_keyword, (parameter_1, parameter_2, parameter_3));
}

// VerifyCommand Function:
// Parameters: 
// - command: The command to compare with valid commands.
// Description: Checks a command against valid commands within the game.
fn VerifyCommand(command: &str) -> Option<String> {

    match command {

        "quit" => return Some(String::from(command)),
        "start"=> return Some(String::from(command)),
        "play" => return Some(String::from(command)),
        "restrict" => return Some(String::from(command)),
        "destroy" => return Some(String::from(command)),
        "evolve" => return Some(String::from(command)),
        "pass" => return Some(String::from(command)),
        _ => return None,
    }
}