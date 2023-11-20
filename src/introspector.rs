use anyhow::Result;
use serde::de::DeserializeOwned;
use thiserror::Error;

fn render_url(endpoint: &str, project: &str) -> String {
    format!("https://introspector.oss-fuzz.com/api/{endpoint}?project={project}")
}

#[derive(Debug, Error)]
pub enum IntroSpectorAPIError {
    #[error(transparent)]
    ReqWestError(#[from] reqwest::Error),
    #[error("Introspector api query unsuccesful.")]
    IntrospectorAPIError,
}

async fn get<T: DeserializeOwned>(
    endpoint: &str,
    project: &str,
) -> std::result::Result<T, reqwest::Error> {
    let url = render_url(endpoint, project);
    let response = reqwest::get(&url).await?;
    Ok(response.json().await?)
}

pub async fn annotated_config(project: &str) -> Result<types::annotated_config::Project> {
    let root: types::annotated_config::Root = get("annotated-cfg", project).await?;
    if root.result != "success" {
        return Err(IntroSpectorAPIError::IntrospectorAPIError.into());
    }
    Ok(root.project)
}

pub async fn far_reach_but_low_coverage(
    project: &str,
) -> Result<Vec<types::far_but_reach_low_coverage::Function>> {
    let root: types::far_but_reach_low_coverage::Root =
        get("far-reach-but-low-coverage", project).await?;
    if root.result != "succes" {
        return Err(IntroSpectorAPIError::IntrospectorAPIError.into());
    }
    Ok(root.functions)
}

pub async fn project_summary(project: &str) -> Result<types::project_summary::Project> {
    let root: types::project_summary::Root = get("project-summary", project).await?;
    if root.result != "success" {
        return Err(IntroSpectorAPIError::IntrospectorAPIError.into());
    }
    Ok(root.project)
}

pub async fn branch_blockers(project: &str) -> Result<Vec<types::branch_blockers::Blocker>> {
    let root: types::branch_blockers::Root = get("branch-blockers", project).await?;
    if root.result != "success" {
        return Err(IntroSpectorAPIError::IntrospectorAPIError.into());
    }
    Ok(root.project_blockers)
}

pub async fn all_functions(project: &str) -> Result<Vec<types::all_functions::Function>> {
    let root: types::all_functions::Root = get("all-functions", project).await?;
    if root.result != "succes" {
        return Err(IntroSpectorAPIError::IntrospectorAPIError.into());
    }
    Ok(root.functions)
}

pub mod types {
    pub mod annotated_config {
        use serde_derive::Deserialize;
        use serde_derive::Serialize;

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Root {
            pub project: Project,
            pub result: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Project {
            #[serde(rename = "annotated-cfg")]
            pub annotated_cfg: AnnotatedCfg,
            pub name: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct AnnotatedCfg {
            #[serde(rename = "tokener_parse_ex_fuzzer")]
            pub tokener_parse_ex_fuzzer: TokenerParseExFuzzer,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct TokenerParseExFuzzer {
            pub destinations: Vec<Destination>,
            #[serde(rename = "src_file")]
            pub src_file: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Destination {
            #[serde(rename = "accummulated-cyclomatic-complexity")]
            pub accummulated_cyclomatic_complexity: i64,
            #[serde(rename = "arg-names")]
            pub arg_names: Vec<String>,
            #[serde(rename = "arg-types")]
            pub arg_types: Vec<String>,
            #[serde(rename = "cyclomatic-complexity")]
            pub cyclomatic_complexity: i64,
            #[serde(rename = "function-name")]
            pub function_name: String,
            #[serde(rename = "raw-function-name")]
            pub raw_function_name: String,
            #[serde(rename = "return-type")]
            pub return_type: String,
            #[serde(rename = "source-file")]
            pub source_file: String,
        }
    }

    pub mod far_but_reach_low_coverage {
        use serde_derive::Deserialize;
        use serde_derive::Serialize;
        use serde_json::Value;

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Root {
            pub functions: Vec<Function>,
            pub result: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Function {
            #[serde(rename = "accummulated-complexity")]
            pub accummulated_complexity: i64,
            #[serde(rename = "function-argument-names")]
            pub function_argument_names: Vec<String>,
            #[serde(rename = "function-arguments")]
            pub function_arguments: Vec<String>,
            #[serde(rename = "function-name")]
            pub function_name: String,
            #[serde(rename = "function_filename")]
            pub function_filename: String,
            #[serde(rename = "is-reached")]
            pub is_reached: bool,
            #[serde(rename = "raw-function-name")]
            pub raw_function_name: String,
            #[serde(rename = "reached-by-fuzzers")]
            pub reached_by_fuzzers: Vec<Value>,
            #[serde(rename = "return-type")]
            pub return_type: String,
            #[serde(rename = "runtime-coverage-percent")]
            pub runtime_coverage_percent: f64,
        }
    }

    pub mod project_summary {
        use serde_derive::Deserialize;
        use serde_derive::Serialize;

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Root {
            pub project: Project,
            pub result: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Project {
            #[serde(rename = "introspector-data")]
            pub introspector_data: IntrospectorData,
            pub name: String,
            #[serde(rename = "runtime-coverage-data")]
            pub runtime_coverage_data: RuntimeCoverageData,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct IntrospectorData {
            #[serde(rename = "annotated_cfg")]
            pub annotated_cfg: AnnotatedCfg,
            #[serde(rename = "branch_pairs")]
            pub branch_pairs: Vec<BranchPair>,
            #[serde(rename = "coverage_lines")]
            pub coverage_lines: f64,
            #[serde(rename = "function_count")]
            pub function_count: i64,
            #[serde(rename = "functions_covered_estimate")]
            pub functions_covered_estimate: f64,
            #[serde(rename = "fuzzer_count")]
            pub fuzzer_count: i64,
            #[serde(rename = "introspector_report_url")]
            pub introspector_report_url: String,
            #[serde(rename = "static_reachability")]
            pub static_reachability: f64,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct AnnotatedCfg {
            #[serde(rename = "tokener_parse_ex_fuzzer")]
            pub tokener_parse_ex_fuzzer: TokenerParseExFuzzer,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct TokenerParseExFuzzer {
            pub destinations: Vec<Destination>,
            #[serde(rename = "src_file")]
            pub src_file: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Destination {
            #[serde(rename = "accummulated-cyclomatic-complexity")]
            pub accummulated_cyclomatic_complexity: i64,
            #[serde(rename = "arg-names")]
            pub arg_names: Vec<String>,
            #[serde(rename = "arg-types")]
            pub arg_types: Vec<String>,
            #[serde(rename = "cyclomatic-complexity")]
            pub cyclomatic_complexity: i64,
            #[serde(rename = "function-name")]
            pub function_name: String,
            #[serde(rename = "raw-function-name")]
            pub raw_function_name: String,
            #[serde(rename = "return-type")]
            pub return_type: String,
            #[serde(rename = "source-file")]
            pub source_file: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct BranchPair {
            #[serde(rename = "blocked-runtime-coverage")]
            pub blocked_runtime_coverage: i64,
            #[serde(rename = "function-name")]
            pub function_name: String,
            pub project: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct RuntimeCoverageData {
            #[serde(rename = "coverage_url")]
            pub coverage_url: String,
            #[serde(rename = "line_coverage")]
            pub line_coverage: LineCoverage,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct LineCoverage {
            pub count: i64,
            pub covered: i64,
            pub percent: f64,
        }
    }

    pub mod branch_blockers {
        use serde_derive::Deserialize;
        use serde_derive::Serialize;

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Root {
            #[serde(rename = "project-blockers")]
            pub project_blockers: Vec<Blocker>,
            pub result: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Blocker {
            #[serde(rename = "blocked_unique_functions")]
            pub blocked_unique_functions: Vec<String>,
            #[serde(rename = "function-name")]
            pub function_name: String,
            #[serde(rename = "project-name")]
            pub project_name: String,
            #[serde(rename = "source_file")]
            pub source_file: String,
            #[serde(rename = "src_linenumber")]
            pub src_linenumber: String,
            #[serde(rename = "unique_blocked_coverage")]
            pub unique_blocked_coverage: i64,
        }
    }

    pub mod all_functions {
        use serde_derive::Deserialize;
        use serde_derive::Serialize;
        use serde_json::Value;

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Root {
            pub functions: Vec<Function>,
            pub result: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Function {
            #[serde(rename = "accummulated-complexity")]
            pub accummulated_complexity: i64,
            #[serde(rename = "function-argument-names")]
            pub function_argument_names: Vec<String>,
            #[serde(rename = "function-arguments")]
            pub function_arguments: Vec<String>,
            #[serde(rename = "function-filename")]
            pub function_filename: String,
            #[serde(rename = "function-name")]
            pub function_name: String,
            #[serde(rename = "is-reached")]
            pub is_reached: bool,
            #[serde(rename = "raw-function-name")]
            pub raw_function_name: String,
            #[serde(rename = "reached-by-fuzzers")]
            pub reached_by_fuzzers: Vec<Value>,
            #[serde(rename = "return-type")]
            pub return_type: String,
            #[serde(rename = "runtime-coverage-percent")]
            pub runtime_coverage_percent: f64,
        }
    }
}
