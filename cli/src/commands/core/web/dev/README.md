# dev

Web development command owners for launch planning, runtime selection and process lifecycle.
Các owner lệnh dev Web gồm lập kế hoạch chạy, chọn runtime và vòng đời process.

`dev.rs` owns the command entry; sibling owners handle endpoint/script rules, native/package launch, runtime validation, target aggregation, environments, local binary lookup, and process supervision.

`dev.rs` sở hữu entry lệnh; các owner con xử lý endpoint/script, launch native/package, kiểm tra runtime, gom target, môi trường, binary cục bộ và giám sát process.
