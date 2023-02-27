pub mod Error {
    use colored::*;

    pub fn heading(text: String) -> ColoredString {
        text.to_uppercase().on_color(Color::Red).color(Color::White)
    }
    pub fn line(text: String) -> ColoredString {
        text.italic().on_color(Color::Red).color(Color::White)
    }
}
