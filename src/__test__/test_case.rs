use file_searcher::{search, search_sensitive};

#[test]
fn test_search() {
    let s = "aaa 111
    bbb
    ccc";
    let ex = vec!["aaa 111"];
    assert_eq!(search(s, "aaa"), ex);
}
#[test]
fn test_search_sensitive() {
    let s = "aaa 111
    AAA 111
    bbb
    ccc";
    let ex = vec!["aaa 111","    AAA 111"];
    assert_eq!(search_sensitive(s, "aaa"), ex);
}
