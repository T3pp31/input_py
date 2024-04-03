#[cfg(test)]

mod tests{
    use input_py::input;

    #[test]
    fn test_input() {
        let text:&str = "input_text";
        let input_data = input(text);
        assert_eq!(input_data, Ok("input_text".to_string()));
    }
}
