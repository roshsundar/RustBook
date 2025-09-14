fn main() {
    // if-let allows matching for one pattern while ignoring the rest
    {
        // This match only looks for a Some
        let config_max = Some(3u8);
        match config_max {
            Some(max) => println!("The maximum is configured to be {max}"),
            _ => println!("Not a valid maximum!"),
        }

        // if-let equivalent of the above match is more concise
        let config_max = Some(3u8);
        if let Some(max) = config_max {
            println!("The maximum is configured to be {max}");
        }
        else {
            println!("Not a valid maximum!");
        }
    }
}
