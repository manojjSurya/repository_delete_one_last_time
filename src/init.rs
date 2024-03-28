use configuration_parameters::{get_configuration_parameters, ConfigurationParameters};
use log::setup_loggers;
use macros::LOG_PARAMS;
use macros::PERF_PARAMS;
use slog::Logger;

pub fn init_loggers(app_name: &str) -> (ConfigurationParameters, Logger, Logger) {
    let config_params = get_configuration_parameters(app_name);
    let (log, diagnostics_log) = setup_loggers(
        config_params.log_file_path(),
        config_params.diagnostics_file_path(),
    );
    config_params.log_parameters(&log);
    LOG_PARAMS.set_once_diagnostic_level(config_params.log_level().to_string());
    PERF_PARAMS.set_once_perf_diagnostics_enabled(config_params.is_perf_diagnostics_enabled());

    (config_params, log, diagnostics_log)
}
