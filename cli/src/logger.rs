/// ANSI color codes for terminal output
pub mod colors {
    pub const RESET: &str = "\x1b[0m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BOLD: &str = "\x1b[1m";
}

/// Icons for message states (using ASCII-safe characters for terminal compatibility)
pub mod icons {
    /// Warning/alert icon (⚠ or fallback)
    pub const ALERT: &str = "⚠";
    /// Success/checkmark icon (✔ or fallback)
    pub const SUCCESS: &str = "✔";
}

/// Prints an alert message to stderr with a warning icon and yellow color.
///
/// This macro follows the same argument pattern as the `log` crate macros.
///
/// # Examples
///
/// alert!("Something went wrong");
/// alert!("Failed to process {} items", count);
///
#[macro_export]
macro_rules! alert {
    ($($arg:tt)*) => {{
        eprintln!(
            "{}{}{} {}{}",
            $crate::logger::colors::BOLD,
            $crate::logger::colors::YELLOW,
            $crate::logger::icons::ALERT,
            format_args!($($arg)*),
            $crate::logger::colors::RESET
        );
    }};
}

/// Prints a success message to stdout with a checkmark icon and green color.
///
/// This macro follows the same argument pattern as the `log` crate macros.
///
/// # Examples
///
///  success!("Operation completed");
///  success!("Processed {} items successfully", count);
///
#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {{
        println!(
            "{}{}{} {}{}",
            $crate::logger::colors::BOLD,
            $crate::logger::colors::GREEN,
            $crate::logger::icons::SUCCESS,
            format_args!($($arg)*),
            $crate::logger::colors::RESET
        );
    }};
}

/// Prints a message to stdout with a checkmark icon.
/// The first argument is left-aligned with a minimum width of 30 characters.
///
/// # Examples
///
/// inform!("Operation completed");
/// inform!("Label:", value);
/// inform!("Processed {} items:", count, total);
///
#[macro_export]
macro_rules! inform {
    // Single argument (left-aligned with min width 30)
    ($msg:expr) => {{
        println!(
            "{}{}{}  {}{:<30}",
            $crate::logger::colors::BOLD,
            $crate::logger::colors::GREEN,
            $crate::logger::icons::SUCCESS,
            $crate::logger::colors::RESET,
            $msg,
        )
    }};
    // Two or more arguments: first one is left-aligned with min width 30, rest displayed normally
    ($first:expr, $($arg:expr),+ $(,)?) => {{
        let rest_args = [$( $crate::__to_string!($arg) ),+];
        let rest = rest_args.join(" ");
        println!(
            "{}{}{}  {}{:<30}{}",
            $crate::logger::colors::BOLD,
            $crate::logger::colors::GREEN,
            $crate::logger::icons::SUCCESS,
            $crate::logger::colors::RESET,
            $first,
            rest,
        )
    }};
}

/// Helper macro to convert any Display type to String
#[macro_export]
#[doc(hidden)]
macro_rules! __to_string {
    ($e:expr) => {
        ToString::to_string(&$e)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_alert_macro() {
        // This test just verifies the macro compiles and runs without panicking
        alert!("Test alert message");
        alert!("Test alert with {} argument", 1);
    }

    #[test]
    fn test_success_macro() {
        // This test just verifies the macro compiles and runs without panicking
        success!("Test success message");
        success!("Test success with {} argument", "formatted");
    }
}

/// Table rendering utilities for terminal output
pub mod table {
    use super::colors;
    use rustix::termios::tcgetwinsize;
    use std::{io::stdout, os::fd::AsFd};

    const COLUMN_GAP: usize = 5;

    /// Returns the terminal width in columns, or 160 as fallback
    fn get_terminal_width() -> usize {
        tcgetwinsize(stdout().as_fd()).map(|ws| ws.ws_col as usize).unwrap_or(160)
    }

    /// Configuration for a single table column
    #[derive(Clone)]
    pub struct ColumnConfig {
        header: String,
        min_width: Option<usize>,
        max_width: Option<usize>,
        flex_grow: f32,
    }

    impl ColumnConfig {
        /// Create a new column configuration with the given header
        pub fn new(header: &str) -> Self {
            Self {
                header: header.to_string(),
                min_width: None,
                max_width: None,
                flex_grow: 1.0,
            }
        }

        /// Set the minimum width for this column
        pub fn min_width(mut self, min: usize) -> Self {
            self.min_width = Some(min);
            self
        }

        /// Set the maximum width for this column
        pub fn max_width(mut self, max: usize) -> Self {
            self.max_width = Some(max);
            self
        }

        /// Set the flex grow factor for this column (default: 1.0)
        /// Higher values mean this column gets more extra space
        pub fn flex_grow(mut self, flex: f32) -> Self {
            self.flex_grow = flex;
            self
        }
    }

    /// A single row of table data
    pub struct TableRow {
        cells: Vec<String>,
    }

    impl TableRow {
        fn new(cells: Vec<String>) -> Self {
            Self {
                cells,
            }
        }
    }

    /// A table that can be rendered to the terminal
    pub struct Table {
        columns: Vec<ColumnConfig>,
        rows: Vec<TableRow>,
    }

    impl Table {
        /// Create a new table with the given column configurations
        pub fn new(columns: Vec<ColumnConfig>) -> Self {
            Self {
                columns,
                rows: Vec::new(),
            }
        }

        /// Add a row of data to the table
        pub fn push_row(&mut self, cells: Vec<&str>) {
            let row = TableRow::new(cells.into_iter().map(String::from).collect());
            self.rows.push(row);
        }

        /// Calculate the width for each column based on terminal size and content
        fn calculate_column_widths(&self) -> Vec<usize> {
            let terminal_width = get_terminal_width();
            let num_columns = self.columns.len();

            if num_columns == 0 {
                return Vec::new();
            }

            let total_gap = COLUMN_GAP * (num_columns - 1);
            let available_width = terminal_width.saturating_sub(total_gap);
            let equal_width = available_width / num_columns;

            // Calculate natural content width for each column
            let natural_widths: Vec<usize> = self
                .columns
                .iter()
                .enumerate()
                .map(|(i, col)| {
                    let header_len = col.header.len();
                    let max_content_len = self
                        .rows
                        .iter()
                        .map(|row| row.cells.get(i).map(|c| c.len()).unwrap_or(0))
                        .max()
                        .unwrap_or(0);
                    header_len.max(max_content_len)
                })
                .collect();

            // Apply min/max constraints (min defaults to equal_width)
            let mut column_widths: Vec<usize> = self
                .columns
                .iter()
                .zip(natural_widths.iter())
                .map(|(col, &natural)| {
                    let min = col.min_width.unwrap_or(equal_width).max(equal_width);
                    let width = natural.max(min);
                    match col.max_width {
                        Some(max) => width.min(max),
                        None => width,
                    }
                })
                .collect();

            // Calculate remaining space and distribute by flex_grow
            let used_width: usize = column_widths.iter().sum();
            let remaining = available_width.saturating_sub(used_width);

            if remaining > 0 {
                let total_flex: f32 = self.columns.iter().map(|c| c.flex_grow).sum();
                if total_flex > 0.0 {
                    for (i, col) in self.columns.iter().enumerate() {
                        let extra = ((remaining as f32) * (col.flex_grow / total_flex)) as usize;
                        column_widths[i] += extra;

                        // Re-apply max constraint after adding extra
                        if let Some(max) = col.max_width {
                            column_widths[i] = column_widths[i].min(max);
                        }
                    }
                }
            }

            column_widths
        }

        /// Wrap text to fit within the given width, returning lines
        fn wrap_text(text: &str, width: usize) -> Vec<String> {
            if width == 0 {
                return vec![String::new()];
            }

            if text.is_empty() {
                return vec![String::new()];
            }

            let mut lines = Vec::new();
            let mut remaining = text;

            while !remaining.is_empty() {
                if remaining.len() <= width {
                    lines.push(remaining.to_string());
                    break;
                }
                let (chunk, rest) = remaining.split_at(width);
                lines.push(chunk.to_string());
                remaining = rest;
            }

            if lines.is_empty() {
                lines.push(String::new());
            }

            lines
        }

        /// Render the table to stdout
        pub fn display(&self) {
            let column_widths = self.calculate_column_widths();

            if column_widths.is_empty() {
                return;
            }

            // Print header row (bold + underlined)
            let header_cells: Vec<String> = self
                .columns
                .iter()
                .zip(column_widths.iter())
                .map(|(col, &width)| format!("{:width$}", col.header, width = width))
                .collect();

            println!(
                "{}{}{}",
                colors::BOLD,
                header_cells.join(&" ".repeat(COLUMN_GAP)),
                colors::RESET
            );

            // Print separator
            let separator: Vec<String> = column_widths.iter().map(|&w| "─".repeat(w)).collect();
            println!("{}", separator.join(&" ".repeat(COLUMN_GAP)));

            // Print data rows
            for row in &self.rows {
                // Wrap each cell and determine row height
                let wrapped_cells: Vec<Vec<String>> = column_widths
                    .iter()
                    .enumerate()
                    .map(|(i, &width)| {
                        let content = row.cells.get(i).map(|s| s.as_str()).unwrap_or("");
                        Self::wrap_text(content, width)
                    })
                    .collect();

                let row_height = wrapped_cells.iter().map(|c| c.len()).max().unwrap_or(1);

                // Print each line of the row
                for line_idx in 0..row_height {
                    let line_cells: Vec<String> = wrapped_cells
                        .iter()
                        .zip(column_widths.iter())
                        .map(|(lines, &width)| {
                            let content = lines.get(line_idx).map(|s| s.as_str()).unwrap_or("");
                            format!("{:width$}", content, width = width)
                        })
                        .collect();

                    println!("{}", line_cells.join(&" ".repeat(COLUMN_GAP)));
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_column_config_builder() {
            let col = ColumnConfig::new("Name").min_width(10).max_width(50).flex_grow(2.0);

            assert_eq!(col.header, "Name");
            assert_eq!(col.min_width, Some(10));
            assert_eq!(col.max_width, Some(50));
            assert_eq!(col.flex_grow, 2.0);
        }

        #[test]
        fn test_column_config_defaults() {
            let col = ColumnConfig::new("Header");

            assert_eq!(col.header, "Header");
            assert_eq!(col.min_width, None);
            assert_eq!(col.max_width, None);
            assert_eq!(col.flex_grow, 1.0);
        }

        #[test]
        fn test_table_push_row() {
            let mut table = Table::new(vec![ColumnConfig::new("Name"), ColumnConfig::new("Age")]);

            table.push_row(vec!["Alice", "30"]);
            table.push_row(vec!["Bob", "25"]);

            assert_eq!(table.rows.len(), 2);
            assert_eq!(table.rows[0].cells, vec!["Alice", "30"]);
            assert_eq!(table.rows[1].cells, vec!["Bob", "25"]);
        }

        #[test]
        fn test_table_empty_columns() {
            let table = Table::new(vec![]);
            let widths = table.calculate_column_widths();
            assert!(widths.is_empty());
        }

        #[test]
        fn test_wrap_text_short() {
            let lines = Table::wrap_text("hello", 10);
            assert_eq!(lines, vec!["hello"]);
        }

        #[test]
        fn test_wrap_text_exact() {
            let lines = Table::wrap_text("hello", 5);
            assert_eq!(lines, vec!["hello"]);
        }

        #[test]
        fn test_wrap_text_long() {
            let lines = Table::wrap_text("helloworld", 5);
            assert_eq!(lines, vec!["hello", "world"]);
        }

        #[test]
        fn test_wrap_text_multiple_wraps() {
            let lines = Table::wrap_text("abcdefghijklmno", 5);
            assert_eq!(lines, vec!["abcde", "fghij", "klmno"]);
        }

        #[test]
        fn test_wrap_text_empty() {
            let lines = Table::wrap_text("", 10);
            assert_eq!(lines, vec![""]);
        }

        #[test]
        fn test_wrap_text_zero_width() {
            let lines = Table::wrap_text("hello", 0);
            assert_eq!(lines, vec![""]);
        }

        #[test]
        fn test_table_display_empty_rows() {
            // Should print header and separator only
            let table = Table::new(vec![ColumnConfig::new("Name"), ColumnConfig::new("Age")]);
            // This test verifies it doesn't panic
            table.display();
        }

        #[test]
        fn test_table_display_with_data() {
            let mut table = Table::new(vec![
                ColumnConfig::new("Name"),
                ColumnConfig::new("Age"),
                ColumnConfig::new("Description"),
            ]);

            table.push_row(vec!["Alice", "30", "Software engineer"]);
            table.push_row(vec!["Bob", "25", "Designer"]);

            // This test verifies it doesn't panic
            table.display();
        }

        #[test]
        fn test_table_missing_cells() {
            let mut table = Table::new(vec![
                ColumnConfig::new("A"),
                ColumnConfig::new("B"),
                ColumnConfig::new("C"),
            ]);

            // Row with fewer cells than columns
            table.push_row(vec!["only one"]);

            // Should not panic
            table.display();
        }
    }
}
