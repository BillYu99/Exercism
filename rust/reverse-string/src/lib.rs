pub fn reverse(input: &str) -> String {
    let rev = input.chars();
    let mut output = String::new();

    for char in rev.rev() {
        output.push(char);
    }

    output
}
