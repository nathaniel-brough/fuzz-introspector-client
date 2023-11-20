//! This is an unnoficial client library for the
//! [fuzz-introspector api](https://introspector.oss-fuzz.com/api).
//!
//! # Example
//! ```rust,no_run
//! # tokio_test::block_on(async {
//! use fuzz_introspector_client::{
//!    all_functions, annotated_config, branch_blockers, far_reach_but_low_coverage, project_summary,
//! };
//!
//! let project = "json-c";
//! // Query the configs endpoint
//! println!("{:?}", annotated_config(project).await.unwrap());
//!
//! // Query the optimal target analysis endpoint
//! println!(
//!   "{:?}",
//!   far_reach_but_low_coverage(project).await.unwrap()
//! );
//!
//! // Query the project summary endpoint
//! println!("{:?}", project_summary(project).await.unwrap());
//!
//! // Query the fuzz blockers endpoint
//! println!("{:?}", branch_blockers(project).await.unwrap());
//!
//! // Get coverage information about all targets
//! println!("{:?}", all_functions(project).await.unwrap());
//! # });
//! ```

/// The introspector api definitions.
pub mod introspector;
pub use introspector::*;
