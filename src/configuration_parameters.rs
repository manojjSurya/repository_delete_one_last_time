use clap;
use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    input_file: String,
    sheet_names: String,
    output_path: String,
    output_file_names: String,
    skip_header: String,
    csv_seperator: String,
    fields_with_date: String,
    header_rows: String,
    as_on_date: NaiveDate,
    log_file_path: String,
    diagnostics_file_path: String,
    log_level: String,
    is_perf_diagnostics_enabled: bool,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "input_file: {}", self.input_file());
        info!(logger, "output_path: {}", self.output_path());
        info!(logger, "output_file_names: {}", self.output_file_names());
        info!(logger, "sheet_names: {}", self.sheet_names());
        info!(logger, "skip_header: {}", self.skip_header());
        info!(logger, "csv_seperator: {}", self.csv_seperator());
        info!(logger, "fields_with_date: {}", self.fields_with_date());
        info!(logger, "header_rows: {}", self.header_rows());
        info!(logger, "as_on_date: {}", self.as_on_date());
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let input_file = matches
            .value_of("input_file")
            .expect("Error getting `input_file`.")
            .to_string();
        let output_path = matches
            .value_of("output_path")
            .expect("Error getting `output_path`.")
            .to_string();
        let output_file_names = matches
            .value_of("output_file_names")
            .expect("Error getting `output_file_names`.")
            .to_string();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
        );
        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let csv_seperator = matches
            .value_of("csv_seperator")
            .expect("Error getting `csv_seperator`.")
            .to_string();
        let fields_with_date = matches
            .value_of("fields_with_date")
            .expect("Error getting `fields_with_date`.")
            .to_string();
        let header_rows = matches
            .value_of("header_rows")
            .expect("Error getting `header_rows`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file_path`.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level`.")
            .to_string();
        let sheet_names = matches
            .value_of("sheet_names")
            .expect("Error getting `sheet_names`.")
            .to_string();
        let skip_header = matches
            .value_of("skip_header")
            .expect("Error getting `skip_header`")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");

        ConfigurationParameters {
            input_file,
            sheet_names,
            output_path,
            output_file_names,
            skip_header,
            csv_seperator,
            fields_with_date,
            header_rows,
            as_on_date,
            log_file_path,
            diagnostics_file_path,
            log_level,
            is_perf_diagnostics_enabled,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn input_file(&self) -> &str {
        &self.input_file
    }
    pub fn skip_header(&self) -> &str {
        &self.skip_header
    }
    pub fn output_path(&self) -> &str {
        &self.output_path
    }
    pub fn output_file_names(&self) -> &str {
        &self.output_file_names
    }
    pub fn sheet_names(&self) -> &str {
        &self.sheet_names
    }
    pub fn csv_seperator(&self) -> &str {
        &self.csv_seperator
    }
    pub fn fields_with_date(&self) -> &str {
        &self.fields_with_date
    }
    pub fn header_rows(&self) -> &str {
        &self.header_rows
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Program for converting excel files to csv files!!")
        .version("1.0.2591")
        .author("ravidar-01 <ravindar.sr@surya-soft.com>")
        .arg(
            Arg::with_name("input_file")
                .long("input-file")
                .value_name("Input File")
                .help("Input file path.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_path")
                .long("output-path")
                .value_name("output File Path")
                .help("Path to output files.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file_names")
                .long("output-file-names")
                .value_name("output File names")
                .help("output file names")
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("DATE")
                .help("The date for which the program has to run.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("Log File Path")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("sheet_names")
                .long("sheet-names")
                .value_name("Input File Sheet Names.")
                .help("Input File  Sheet Names.")
                .required(true)
        )
        .arg(
            Arg::with_name("csv_seperator")
                .long("csv-seperator")
                .value_name("CSV SEPERATOR")
                .help("CSV Seperator.")
                .default_value("|")
                .required(true)
        )
        .arg(
            Arg::with_name("fields_with_date")
                .long("fields-with-date")
                .value_name("fields with date")
                .help("fields with date.")
                .default_value("1")
                .required(true)
        )
        .arg(
            Arg::with_name("header_rows")
                .long("header-rows")
                .value_name("header rows")
                .help("header rows.")
                .default_value("1")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("Diagnostics File Path")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::with_name("skip_header")
                .long("skip-header")
                .value_name("SKIP HEADER")
                .help("This flag helps decide whether the file header should be skipped.")
                .default_value("false")
                .required(true)
        )
        .arg(
            Arg::with_name("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .default_value("false")
                .required(false)
        )
        .get_matches()
}
