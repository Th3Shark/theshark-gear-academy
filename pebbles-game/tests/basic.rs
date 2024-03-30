// Unit tests for the init() function
#[cfg(test)]
mod init_tests {
    

    #[test]
    fn test_init_valid_input() {
        // Test initialization with valid input data
        // Assert that the returned GameState is as expected
    }

    #[test]
    fn test_init_invalid_input() {
        // Test initialization with invalid input data
        // For example, negative pebbles count or max pebbles per turn
        // Assert that the function returns an error or handles the invalid input gracefully
    }

    #[test]
    fn test_init_first_player_random() {
        // Test if the first player is chosen randomly
        // Run the init() function multiple times and check if the first player varies
    }
}

// Unit tests for the handle() function
#[cfg(test)]
mod handle_tests {
    

    #[test]
    fn test_handle_user_turn() {
        // Test handling of User's turn action
        // Assert that the returned PebblesEvent is as expected
    }

    #[test]
    fn test_handle_program_turn() {
        // Test handling of Program's turn action
        // Assert that the returned PebblesEvent is as expected
    }

    #[test]
    fn test_handle_restart_action() {
        // Test handling of Restart action
        // Assert that the game state is reset appropriately
    }
}

// Unit tests for the state() function
#[cfg(test)]
mod state_tests {
    

    #[test]
    fn test_state_initial() {
        // Test the initial game state
        // Assert that the returned GameState matches the expected initial state
    }

    #[test]
    fn test_state_after_user_turn() {
        // Test the game state after the User's turn
        // Assert that the returned GameState reflects the changes made by the User's turn
    }

    #[test]
    fn test_state_after_program_turn() {
        // Test the game state after the Program's turn
        // Assert that the returned GameState reflects the changes made by the Program's turn
    }
}
