pub fn do_meta_command(command: &str) -> bool {
    if command == ".exit" {
        std::process::exit(0);
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_meta_nothing() {
        assert_eq!(do_meta_command("nothing"), false);
    }

}
