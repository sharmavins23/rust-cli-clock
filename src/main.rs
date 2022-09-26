use chrono::Local;

// https://www.w3.org/TR/xml-entity-names/025.html
// TODO: Redo this for a simpler structure
const DIGITS: [[&str; 11]; 7] = [
    [
        "┏━┓ ",
        "  ╻  ",
        " ┏━┓ ",
        " ┏━┓ ",
        " ╻ ╻ ",
        " ┏━┓ ",
        " ┏   ",
        " ┏━┓ ",
        " ┏━┓ ",
        " ┏━┓ ",
        "   ",
    ],
    [
        "┃ ┃ ",
        "  ┃  ",
        "   ┃ ",
        "   ┃ ",
        " ┃ ┃ ",
        " ┃   ",
        " ┃   ",
        "   ┃ ",
        " ┃ ┃ ",
        " ┃ ┃ ",
        " ╻ ",
    ],
    [
        "┃ ┃ ",
        "  ┃  ",
        "   ┃ ",
        "   ┃ ",
        " ┃ ┃ ",
        " ┃   ",
        " ┃   ",
        "   ┃ ",
        " ┃ ┃ ",
        " ┃ ┃ ",
        "   ",
    ],
    [
        "┃ ┃ ",
        "  ┃  ",
        " ┏━┛ ",
        " ┣━┫ ",
        " ┗━┫ ",
        " ┗━┓ ",
        " ┣━┓ ",
        "   ┃ ",
        " ┣━┫ ",
        " ┗━┫ ",
        "   ",
    ],
    [
        "┃ ┃ ",
        "  ┃  ",
        " ┃   ",
        "   ┃ ",
        "   ┃ ",
        "   ┃ ",
        " ┃ ┃ ",
        "   ┃ ",
        " ┃ ┃ ",
        "   ┃ ",
        "   ",
    ],
    [
        "┃ ┃ ",
        "  ┃  ",
        " ┃   ",
        "   ┃ ",
        "   ┃ ",
        "   ┃ ",
        " ┃ ┃ ",
        "   ┃ ",
        " ┃ ┃ ",
        "   ┃ ",
        " ╹ ",
    ],
    [
        "┗━┛ ",
        "  ╹  ",
        " ┗━━ ",
        " ┗━┛ ",
        "   ╹ ",
        " ┗━┛ ",
        " ┗━┛ ",
        "   ╹ ",
        " ┗━┛ ",
        " ┗━┛ ",
        "   ",
    ],
];

// ===== CLI handling ==========================================================

// Clear the CLI screen
fn cli_clear_screen() {
    // ANSI escape character for clearing the screen
    print!("\x1b[2J");
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

// ===== Main ==================================================================

fn main() {
    // Clear the screen
    cli_clear_screen();
    // Hide the cursor
    cli_hide_cursor();

    loop {
        let t = Local::now();
        let time = t.format("%H:%M:%S").to_string();
        for row in &DIGITS {
            for c in time.chars() {
                let col = match c {
                    '0'..='9' => c as usize - '0' as usize,
                    ':' => 10,
                    _ => 10,
                };
                print!("{} ", row[col]);
            }
            println!();
        }

        // Wait for Almost 1 second
        std::thread::sleep(std::time::Duration::from_millis(999));

        // Move the cursor to the top left corner of the screen
        cli_move_cursor_top_left();
    }
} // fn_main
