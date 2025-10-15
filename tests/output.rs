//! Serves as a test for the crate

extern crate simply_colored;

use simply_colored::*;

macro_rules! test {
    (
        $(
            $COLOR:ident
        )*
    ) => {{
        $(
            print!("{}", $COLOR);
            print!(stringify!($COLOR));
            println!("{RESET}");
        )*
    }};
}

macro_rules! test_single_reset {
    ($($STYLE:ident $NO_STYLE:ident),*) => {
        {{$(
        print!("{}", $STYLE);
        print!(stringify!($STYLE));
        println!("{}", $NO_STYLE);
    )*}}
    };
}

fn main() {
    test! {
        DIM_RED BG_DIM_RED RED BG_RED

        DIM_GREEN BG_DIM_GREEN GREEN BG_GREEN

        DIM_YELLOW BG_DIM_YELLOW YELLOW BG_YELLOW

        DIM_BLUE BG_DIM_BLUE BLUE BG_BLUE

        DIM_MAGENTA BG_DIM_MAGENTA MAGENTA BG_MAGENTA

        DIM_CYAN BG_DIM_CYAN CYAN BG_CYAN

        DIM_WHITE BG_DIM_WHITE WHITE BG_WHITE

        DIM_DEFAULT BG_DIM_DEFAULT DEFAULT BG_DEFAULT
    }
    test_single_reset!(
        BOLD NO_BOLD,
        DIM NO_DIM,
        ITALIC NO_ITALIC,
        UNDERLINE NO_UNDERLINE,
        BLINK NO_BLINK,
        REVERSE NO_REVERSE,
        HIDE NO_HIDE,
        STRIKETHROUGH NO_STRIKETHROUGH
    )
}
