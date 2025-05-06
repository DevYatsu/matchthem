use matchthem::Router;

#[test]
fn all_matches_static_and_catchall() {
    let mut router = Router::new();
    router.insert("/path/to/resource", "static").unwrap();
    router.insert("/path/{*rest}", "catchall").unwrap();

    let matches = router.all_matches("/path/to/resource");
    let values: Vec<_> = matches.iter().map(|m| *m.value).collect();
    assert_eq!(values, vec!["static", "catchall"]);
}

#[test]
fn all_matches_with_suffix_param() {
    let mut router = Router::new();
    router.insert("/file/report.txt", "text file").unwrap();
    router.insert("/file/{*other}", "catchall").unwrap();

    let matches = router.all_matches("/file/report.txt");
    let values: Vec<_> = matches.iter().map(|m| *m.value).collect();

    assert!(vec!["text file", "catchall"]
        .iter()
        .map(|v| values.contains(&v))
        .all(|v| v));
}

#[test]
fn all_matches_mut_modifies_catchall_and_static() {
    let mut router = Router::new();
    router.insert("/x/y/z", vec!["static"]).unwrap();
    router.insert("/x/{*rest}", vec!["catchall"]).unwrap();

    for m in router.all_matches_mut("/x/y/z").iter_mut() {
        m.value.push("edited");
    }

    let s = router.at("/x/y/z").unwrap().value;
    assert!(vec!["static", "edited"]
        .iter()
        .map(|v| s.contains(&v))
        .all(|v| v));

    let values = router
        .all_matches("/x/y/z")
        .into_iter()
        .find(|m| m.value != s)
        .unwrap()
        .value;

    assert!(vec!["catchall", "edited"]
        .iter()
        .map(|v| values.contains(&v))
        .all(|v| v));
}

#[test]
fn all_matches_handles_suffix_wildcard_conflict() {
    let mut router = Router::new();
    router.insert("/{*all}", "all").unwrap();
    router.insert("/images/42.png", "image").unwrap();
    router.insert("/images/{rest}", "catchall").unwrap();

    let matches = router.all_matches("/images/42.png");
    let values: Vec<_> = matches.iter().map(|m| *m.value).collect();
    assert!(["all", "image", "catchall"]
        .iter()
        .map(|v| values.contains(&v))
        .all(|v| v));
}

#[test]
fn all_matches_handles_param_and_static_conflict() {
    let mut router = Router::new();
    router.insert("/user/profile", "static").unwrap();
    router.insert("/user/{name}", "param").unwrap();

    let matches = router.all_matches("/user/profile");
    let values: Vec<_> = matches.iter().map(|m| *m.value).collect();
    assert!(vec!["static", "param"]
        .iter()
        .map(|v| values.contains(&v))
        .all(|v| v));
}
