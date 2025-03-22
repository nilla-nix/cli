use clap::builder::styling::Style;

pub mod build;
pub mod completions;
pub mod nixos;
pub mod run;
pub mod shell;

const HEADER_STYLE: Style = Style::new().bold().underline();
const DIM_STYLE: Style = Style::new().dimmed();
const BOLD_STYLE: Style = Style::new().bold();

pub fn make_examples(examples: &[(&str, &str)]) -> String {
    let mut out = format!("{HEADER_STYLE}Examples:{HEADER_STYLE:#}");

    for ex in examples {
        out.push_str(&format!("\n  {DIM_STYLE}# {}{DIM_STYLE:#}", ex.0));
        out.push_str(&format!(
            "\n  {DIM_STYLE}${DIM_STYLE:#} {BOLD_STYLE}nilla{BOLD_STYLE:#} {}\n",
            ex.1
        ));
    }

    out
}
