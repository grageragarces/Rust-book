fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

// fn describe_state_quarter(coin: Coin) -> Option<String> {
//     if let Coin::Quarter(state) = coin {
//         if state.existed_in(1900) {
//             Some(format!("{state:?} is pretty old, for America!"))
//         } else {
//             Some(format!("{state:?} is relatively new."))
//         }
//     } else {
//         None
//     }
// }

// Equiv to:

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
