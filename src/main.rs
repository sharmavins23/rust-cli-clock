use chrono::{DateTime, Local};

// ===== Digit structures ======================================================

const DIGIT0: [&str; 5] = [" _ ", "| |", "|_|", "   ", "   "];

const DIGIT1: [&str; 5] = ["   ", "  |", "  |", "   ", "   "];

const DIGIT2: [&str; 5] = [" _ ", " _|", "|_ ", "   ", "   "];

const DIGIT3: [&str; 5] = [" _ ", " _|", " _|", "   ", "   "];

const DIGIT4: [&str; 5] = ["   ", "|_|", "  |", "   ", "   "];

const DIGIT5: [&str; 5] = [" _ ", "|_ ", " _|", "   ", "   "];

const DIGIT6: [&str; 5] = [" _ ", "|_ ", "|_|", "   ", "   "];

const DIGIT7: [&str; 5] = [" _ ", "  |", "  |", "   ", "   "];

const DIGIT8: [&str; 5] = [" _ ", "|_|", "|_|", "   ", "   "];

const DIGIT9: [&str; 5] = [" _ ", "|_|", " _|", "   ", "   "];

const DIGIT_COLON: [&str; 5] = ["   ", " o ", " o ", "   ", "   "];

const DIGITS: [[&str; 5]; 11] = [
    DIGIT0,
    DIGIT1,
    DIGIT2,
    DIGIT3,
    DIGIT4,
    DIGIT5,
    DIGIT6,
    DIGIT7,
    DIGIT8,
    DIGIT9,
    DIGIT_COLON,
];

// ===== CLI handling ==========================================================

// Clear the CLI screen
fn cli_clear_screen() {
    // https://stackoverflow.com/a/66911945
    print!("{esc}c", esc = 27 as char);
}

// Hide the CLI cursor
fn cli_hide_cursor() {
    // ANSI escape character for hiding the cursor
    print!("\x1b[?25l");
}

// Move CLI cursor up to the top left corner of the screen
fn cli_move_cursor_top_left() {
    // ANSI escape character for moving the cursor
    print!("\x1b[7A");
}

// ===== Timing ================================================================

// Return the current time as a string
fn get_time() -> String {
    // Get the current time
    let time: DateTime<Local> = Local::now();

    // Format the time as a string
    return time.format("%H:%M:%S").to_string();
}

// Wait for a given number of milliseconds
fn wait(milliseconds: u64) {
    // Wait for the given number of milliseconds
    std::thread::sleep(std::time::Duration::from_millis(milliseconds));
}

// ===== Main ==================================================================

fn main() {
    // Clear the screen
    cli_clear_screen();
    // Hide the cursor
    cli_hide_cursor();

    loop {
        // Get the time
        let time: String = get_time();

        // Compute a digit's sizing
        let digit_width: usize = DIGIT0[0].len();

        // Iterate from 0 to <5 (for digit sizing)...
        for i in 0..digit_width {
            // Then, iterate through the digits on the clock...
            for time_character in time.chars() {
                // Find which digit should be displayed
                let digit: [&str; 5] = match time_character {
                    '0'..='9' => {
                        // Digits can easily be grabbed from the DIGITS array
                        DIGITS[time_character.to_digit(10).unwrap() as usize]
                    }
                    ':' => {
                        // The colon is a special case
                        DIGITS[10]
                    }
                    _ => {
                        // Something went wrong here
                        panic!("Invalid time character: {}", time_character);
                    }
                };

                // Print the digit's current line
                print!("{} ", digit[i]);
            }

            // Print a newline to begin printing the next line of digits
            println!();
        }

        // Wait for a short amount of time
        wait(1000); // ! Might be 999 for a more accurate clock

        // Move the cursor back to the top left corner of the screen
        cli_move_cursor_top_left();
    }
} // fn_main
