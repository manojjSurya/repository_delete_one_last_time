use self::io::*;
use configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;
use calamine::{open_workbook_auto, Reader, Sheets};
use health_report::HealthReport;
use macros;
use rbdate::datevalue_to_naive_date;
use rbdate::NaiveDate;
use std::io::Write;
use std::mem::replace;
//function to wrtite Excel to csv file
pub fn excel_to_csv(
    input_xl_file: &mut Sheets,
    sheet_name: &str,
    output_file_name: &str,
    output_path: &str,
    seperator: &str,
    skip_header: bool,
    header_rows: usize,
    fields_with_date_sheet: String,
    as_on_date: &NaiveDate,
    log: &Logger,
    tot_accounts: &mut i64,
    acc_read_succ: &mut i64,
) {
    let output_path_with_file_name = format!("{}{}{}", output_path, "/", output_file_name);
    let mut record_encountered = 0;
    let mut skip = 0;
    let mut num_skip_rows = header_rows;
    if skip_header {
        skip = num_skip_rows;
    }
    let mut op_writer = get_writer(&output_path_with_file_name);
    let mut len = 0;
    let fields_with_date: Vec<&str> = fields_with_date_sheet.split(",").collect();
    if let Some(Ok(range)) = input_xl_file.worksheet_range(sheet_name) {
        len = range.get_size().1;
        for row in range.rows().skip(skip) {
            record_encountered += 1;
            let mut op_string = String::new();
            if num_skip_rows > 0 && skip == 0 {
                for index in 0..len {
                    op_string.push_str(
                        &row[index]
                            .to_string()
                            .replace(seperator, " ")
                            .replace("\n", " "),
                    );
                    if index != len - 1 {
                        op_string.push_str(seperator);
                    }
                }
                num_skip_rows = num_skip_rows - 1;
            } else {
                for index in 0..len {
                    if fields_with_date.contains(&&(index + 1).to_string().as_str()) {
                        let date = datevalue_to_naive_date(&row[index].to_string())
                            .unwrap_or(*as_on_date)
                            .format("%d-%m-%Y");
                        op_string.push_str(&date.to_string());
                    } else {
                        op_string.push_str(
                            &row[index]
                                .to_string()
                                .replace(seperator, " ")
                                .replace("\n", " "),
                        );
                    }
                    if index != len - 1 {
                        op_string.push_str(seperator);
                    }
                }
            }
            op_string.push_str("\n");
            write!(op_writer, "{}", op_string);
        }
    } else {
        panic!(
            "Could not find the Sheet name - `{}` in input file",
            sheet_name
        );
    }
    *tot_accounts = *tot_accounts + record_encountered;
    *acc_read_succ = *acc_read_succ + record_encountered;
    log_info!(
        log,
        "Total number of records encountered in sheet {:?} : {:?} ",
        sheet_name,
        record_encountered
    );
}
//Call the function for differnt sheets
pub fn processor(config_params: &ConfigurationParameters, log: &Logger, diagnostics: &Logger) {
    let mut tot_accounts: i64 = 0;
    let mut acc_read_succ: i64 = 0;
    let acc_read_fail: i64 = 0;
    let tot_amt_ip: f64 = 0.0;
    let tot_amt_op: f64 = 0.0;
    let tot_no_cf: i64 = 0;
    let output_file_names: Vec<&str> = config_params.output_file_names().split(",").collect();
    let fields_with_date_sheet: Vec<&str> = config_params.fields_with_date().split("|").collect();
    let skip_header: Vec<&str> = config_params.skip_header().split('|').collect();
    let header_rows: Vec<&str> = config_params.header_rows().split('|').collect();
    let mut input_xl_file: Sheets = open_workbook_auto(config_params.input_file()).unwrap();
    let mut index = 0;
    for sheet_name in config_params.sheet_names().split(",") {
        excel_to_csv(
            &mut input_xl_file,
            sheet_name,
            output_file_names[index],
            config_params.output_path(),
            config_params.csv_seperator(),
            skip_header[index]
                .parse::<bool>()
                .expect("skip header value is not correct"),
            header_rows[index]
                .parse::<usize>()
                .expect("header rows value is not correct"),
            fields_with_date_sheet[index].to_string(),
            config_params.as_on_date(),
            log,
            &mut tot_accounts,
            &mut acc_read_succ,
        );
        index += 1;
    }

    //write health check report
    let health_report = HealthReport::new(
        tot_accounts,
        acc_read_succ,
        acc_read_fail,
        tot_amt_ip,
        tot_amt_op,
        tot_no_cf,
    );
    health_report.gen_health_rpt(&config_params.output_path());
}
