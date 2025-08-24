#[cfg(test)]
mod unit_tests {

    use terminal_client::helper_prelude::io::*;

    // Testing if the print helper function successfully run
    #[test]
    fn test_print_right() {
        print_right("Hello world");
    }

    #[test]
    fn test_print_center() {
        print_center("Hello world");
    }
}
