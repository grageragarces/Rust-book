fn main(){

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), //explicitly ignoring all other values in the last arm
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}