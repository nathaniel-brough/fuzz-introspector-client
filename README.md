# fuzz-introspector-client
An unofficial client library for the 
[fuzz-introspector API](https://introspector.oss-fuzz.com/api).

## Quickstart
Add package as a dependency;
`cargo add fuzz-introspector-client`

```rust
use fuzz_introspector_client::introspector::{
    all_functions, annotated_config, branch_blockers, far_reach_but_low_coverage, project_summary,
};
println!("{:?}", all_functions("json-c").await.unwrap());
println!("{:?}", annotated_config("json-c").await.unwrap());
println!("{:?}", branch_blockers("json-c").await.unwrap());
println!("{:?}", far_reach_but_low_coverage("json-c").await.unwrap());
println!("{:?}", project_summary("json-c").await.unwrap());
```
