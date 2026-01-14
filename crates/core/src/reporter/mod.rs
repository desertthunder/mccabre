pub mod coverage_jsonl;
pub mod coverage_term;
pub mod legacy;

pub use coverage_jsonl::JsonlReporter;
pub use coverage_term::{format_file_coverage, report_coverage};
pub use legacy::{FileReport, Report, Summary};
