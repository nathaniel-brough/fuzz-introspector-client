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
    if root.result != "success" {
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
    if root.result != "success" {
        return Err(IntroSpectorAPIError::IntrospectorAPIError.into());
    }
    Ok(root.functions)
}

pub mod types {
    pub mod annotated_config {
        use serde_derive::Deserialize;
        use serde_derive::Serialize;

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Root {
            pub project: Project,
            pub result: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Project {
            pub annotated_cfg: Vec<AnnotatedCfg>,
            pub name: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct AnnotatedCfg {
            pub destinations: Vec<Destination>,
            pub fuzzer_name: String,
            pub src_file: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Destination {
            pub accummulated_cyclomatic_complexity: i64,
            pub arg_names: Vec<String>,
            pub arg_types: Vec<String>,
            pub cyclomatic_complexity: i64,
            pub function_name: String,
            pub raw_function_name: String,
            pub return_type: String,
            pub source_file: String,
        }
    }

    pub mod far_but_reach_low_coverage {
        use serde_derive::Deserialize;
        use serde_derive::Serialize;

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Root {
            pub functions: Vec<Function>,
            pub result: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Function {
            pub accummulated_complexity: usize,
            pub function_argument_names: Vec<String>,
            pub function_arguments: Vec<String>,
            pub function_name: String,
            pub function_filename: String,
            pub is_reached: bool,
            pub raw_function_name: String,
            pub reached_by_fuzzers: Vec<String>,
            pub return_type: String,
            pub runtime_coverage_percent: f64,
        }
    }

    pub mod project_summary {
        use serde_derive::Deserialize;
        use serde_derive::Serialize;

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Root {
            pub project: Project,
            pub result: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Project {
            pub introspector_data: IntrospectorData,
            pub name: String,
            pub runtime_coverage_data: RuntimeCoverageData,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct IntrospectorData {
            pub annotated_cfg: Vec<AnnotatedCfg>,
            pub branch_pairs: Vec<BranchPair>,
            pub coverage_lines: f64,
            pub function_count: i64,
            pub functions_covered_estimate: f64,
            pub fuzzer_count: i64,
            pub introspector_report_url: String,
            pub static_reachability: f64,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct AnnotatedCfg {
            pub destinations: Vec<Destination>,
            pub fuzzer_name: String,
            pub src_file: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Destination {
            pub accummulated_cyclomatic_complexity: i64,
            pub arg_names: Vec<String>,
            pub arg_types: Vec<String>,
            pub cyclomatic_complexity: i64,
            pub function_name: String,
            pub raw_function_name: String,
            pub return_type: String,
            pub source_file: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct BranchPair {
            pub blocked_runtime_coverage: i64,
            pub blocked_unique_functions: Vec<String>,
            pub function_name: String,
            pub linenumber: String,
            pub project: String,
            pub source_file: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct RuntimeCoverageData {
            pub coverage_url: String,
            pub line_coverage: LineCoverage,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
        pub struct Root {
            pub project_blockers: Vec<Blocker>,
            pub result: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Blocker {
            pub blocked_unique_functions: Vec<String>,
            pub function_name: String,
            pub project_name: String,
            pub source_file: String,
            pub src_linenumber: String,
            pub unique_blocked_coverage: i64,
        }
    }

    pub mod all_functions {
        use serde_derive::Deserialize;
        use serde_derive::Serialize;

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Root {
            pub functions: Vec<Function>,
            pub result: String,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Function {
            pub accummulated_complexity: i64,
            pub function_argument_names: Vec<String>,
            pub function_arguments: Vec<String>,
            pub function_filename: String,
            pub function_name: String,
            pub is_reached: bool,
            pub raw_function_name: String,
            pub reached_by_fuzzers: Vec<String>,
            pub return_type: String,
            pub runtime_coverage_percent: f64,
        }
    }
}
