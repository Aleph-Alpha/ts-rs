#![allow(unused)]

mod issue_261 {
    use ts_rs::TS;

    trait Driver {
        type Info: TS;
    }

    struct TsDriver;
    impl Driver for TsDriver {
        type Info = String;
    }

    #[derive(TS)]
    #[ts(export, export_to = "concrete_generic/")]
    struct OtherInfo {
        x: i32,
    }

    #[derive(TS)]
    #[ts(export, export_to = "concrete_generic/")]
    struct OtherDriver;
    impl Driver for OtherDriver {
        type Info = OtherInfo;
    }

    #[derive(TS)]
    #[ts(export, export_to = "concrete_generic/", concrete(T = TsDriver))]
    struct Consumer1<T: Driver> {
        info: T::Info,
    }

    #[derive(TS)]
    #[ts(export, export_to = "concrete_generic/", concrete(T = OtherDriver))]
    struct Consumer2<T: Driver> {
        info: T::Info,
        driver: T,
    }

    #[test]
    fn concrete_generic_param() {
        assert_eq!(
            Consumer1::<TsDriver>::decl(),
            "type Consumer1 = { info: string, };"
        );
        // `decl` must use the concrete generic, no matter what we pass in
        assert_eq!(
            Consumer1::<TsDriver>::decl(),
            Consumer1::<OtherDriver>::decl()
        );

        assert_eq!(
            Consumer2::<OtherDriver>::decl_concrete(),
            "type Consumer2 = { info: OtherInfo, driver: OtherDriver, };"
        );
    }
}
