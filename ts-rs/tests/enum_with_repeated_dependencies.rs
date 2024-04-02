use ts_rs::{
    typelist::{TypeList, TypeVisitor},
    TS,
};

#[derive(Debug, ts_rs::TS)]
#[ts(export)]
pub enum EnumWith1Dependency {
    V001(String),
    V002(String),
}

struct DependencyCounter(usize);

impl TypeVisitor for DependencyCounter {
    fn visit<T: TS + 'static + ?Sized>(&mut self) {
        self.0 += 1;
    }
}

#[test]
fn dedup_deps() {
    let mut dependency_counter = DependencyCounter(0);

    EnumWith1Dependency::dependency_types().for_each(&mut dependency_counter);

    assert_eq!(dependency_counter.0, 1);
}
