fn main() {
    println!("Hello world!");
    let event_source = event_store::EventSourceId::create();
    println!("{}", event_source);
    println!("{:?}", event_source);
    let version = versioning::Version::not_set();
    println!("{}", version);
    println!("{:?}", version);

    let another_version = versioning::Version::with()
        .major(1)
        .minor(0)
        .patch(2)
        .build();
    let derived_version = another_version
        .build_from()
        .patch(another_version.patch() + 1)
        .build();
    let derived_pre_release = derived_version.build_from().pre_release("beta", 3).build();

    println!("{}", another_version);
    println!("{}", derived_version);
    println!("{}", derived_pre_release);
}
