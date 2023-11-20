use clap::{Parser, Subcommand};
use fuzz_introspector_client::introspector::{
    all_functions, annotated_config, branch_blockers, far_reach_but_low_coverage, project_summary,
};

#[derive(Subcommand, Debug)]
enum EndPoint {
    AnnotatedConfig,
    FarReachButLowCoverage,
    ProjectSummary,
    BranchBlockers,
    AllFunctions,
}

#[derive(Parser, Debug)]
struct Args {
    project: String,
    #[command(subcommand)]
    endpoint: EndPoint,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    use EndPoint::*;
    match args.endpoint {
        AnnotatedConfig => {
            println!("{:?}", annotated_config(&args.project).await.unwrap());
        }
        FarReachButLowCoverage => {
            println!(
                "{:?}",
                far_reach_but_low_coverage(&args.project).await.unwrap()
            );
        }
        ProjectSummary => {
            println!("{:?}", project_summary(&args.project).await.unwrap());
        }
        BranchBlockers => {
            println!("{:?}", branch_blockers(&args.project).await.unwrap());
        }
        AllFunctions => {
            println!("{:?}", all_functions(&args.project).await.unwrap());
        }
    }
}
