# e2e tests
### [dependencies](./dependencies)
A user creates the crate `consumer`, which depends on some crate `dependency1`, which possibly lives on [crates.io](https://crates.io). 
If a struct in `consumer` contains a type from `dependency1`, it should be exported as well.

### [workspace](./workspace)
A user creates a workspace, containing `crate1`, `crate2`, and `parent`.  
`crate1` and `crate2` are independent, but `parent` depends on both `crate1` and `crate2`.
