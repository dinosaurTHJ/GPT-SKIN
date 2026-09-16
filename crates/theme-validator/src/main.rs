use retheme_theme_protocol::{ThemeError, validate_development_directory, validate_source_archive};
use serde::Serialize;
use std::env;
use std::path::Path;
use std::process::ExitCode;

#[derive(Debug, Serialize)]
struct Report {
    ok: bool,
    manifest: Option<serde_json::Value>,
    errors: Vec<ReportMessage>,
    warnings: Vec<ReportMessage>,
}

#[derive(Debug, Serialize)]
struct ReportMessage {
    code: &'static str,
    message: String,
}

fn main() -> ExitCode {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let (code, report) = execute(&args);
    print_report(&report);
    ExitCode::from(code)
}

fn execute(args: &[String]) -> (u8, Report) {
    let result = match args {
        [command, path] if command == "--source" => std::fs::read(path)
            .map_err(ThemeError::from)
            .and_then(|source| validate_source_archive(&source).map(|(theme, _)| theme)),
        [command, path] if command == "--directory" => {
            validate_development_directory(Path::new(path)).map(|(theme, _)| theme)
        }
        _ => {
            return (2, Report {
                ok: false,
                manifest: None,
                errors: vec![ReportMessage {
                    code: "usage",
                    message: "用法：retheme-theme-validator --source <theme.zip> 或 --directory <theme-dir>".into(),
                }],
                warnings: vec![],
            });
        }
    };

    match result {
        Ok(theme) => {
            let manifest = serde_json::to_value(theme.manifest).expect("manifest must serialize");
            (
                0,
                Report {
                    ok: true,
                    manifest: Some(manifest),
                    errors: vec![],
                    warnings: vec![],
                },
            )
        }
        Err(error) => (
            1,
            Report {
                ok: false,
                manifest: None,
                errors: vec![ReportMessage {
                    code: "protocol",
                    message: error.to_string(),
                }],
                warnings: vec![],
            },
        ),
    }
}

fn print_report(report: &Report) {
    println!(
        "{}",
        serde_json::to_string(&report).expect("validator report must serialize")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn documented_directory_returns_stable_success_report() {
        let directory =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/theme-example/package");
        let args = vec![
            "--directory".into(),
            directory.to_string_lossy().into_owned(),
        ];
        let (code, report) = execute(&args);

        assert_eq!(code, 0);
        assert!(report.ok);
        assert_eq!(
            report
                .manifest
                .as_ref()
                .and_then(|manifest| manifest["id"].as_str()),
            Some("studio.example.protocol-preview")
        );
        assert_eq!(
            report
                .manifest
                .as_ref()
                .and_then(|manifest| manifest["experience"]["assets"][0]["lightAsset"].as_str()),
            Some("assets/hero.svg")
        );
        assert_eq!(
            report
                .manifest
                .as_ref()
                .and_then(|manifest| manifest["experience"]["assets"][0]["darkAsset"].as_str()),
            Some("assets/hero.svg")
        );
        assert!(report.errors.is_empty());
        assert!(report.warnings.is_empty());
        serde_json::to_string(&report).expect("report must remain JSON serializable");
    }

    #[test]
    fn protocol_failure_and_usage_have_distinct_exit_codes() {
        let missing = vec!["--directory".into(), "/path/that/does/not/exist".into()];
        let (protocol_code, protocol_report) = execute(&missing);
        assert_eq!(protocol_code, 1);
        assert!(!protocol_report.ok);
        assert_eq!(protocol_report.errors[0].code, "protocol");

        let (usage_code, usage_report) = execute(&[]);
        assert_eq!(usage_code, 2);
        assert!(!usage_report.ok);
        assert_eq!(usage_report.errors[0].code, "usage");
    }
}
