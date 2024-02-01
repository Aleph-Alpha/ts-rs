use std::collections::{BTreeSet, HashMap};
use ts_rs::{Dependency, TS};
use ts_rs::typelist::TypeList;
use ts_rs::typelist::TypeVisitor;

fn get_deps<T: TS + 'static>() -> BTreeSet<Dependency> {
    use ts_rs::typelist::{TypeList, TypeVisitor};

    let mut deps = BTreeSet::new();
    struct Visit<'a>(&'a mut BTreeSet<Dependency>);
    impl<'a> TypeVisitor for Visit<'a> {
        fn visit<T: TS + 'static + ?Sized>(&mut self) {
            if let Some(dep) = Dependency::from_ty::<T>() {
                self.0.insert(dep);
            }
        }
    }
    T::dependency_types().for_each(&mut Visit(&mut deps));

    deps
}

#[allow(dead_code)]
#[test]
fn test() {
    #[derive(TS)]
    struct A {}

    #[derive(TS)]
    struct B {}

    #[derive(TS)]
    struct X {
        x: i32,
        a: A,
        a2: A,
        map: HashMap<A, B>,
    }

    println!("{:?}", X::dependencies().into_iter().collect::<BTreeSet<_>>());
    println!("{:?}", get_deps::<X>());
}
