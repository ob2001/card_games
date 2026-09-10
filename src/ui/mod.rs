pub fn highlight_print_string(str: &str) {
    let l = str.len();
    print!("{}\n{}\n{}\n", "-".repeat(l), str, "-".repeat(l));
}
