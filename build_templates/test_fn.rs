#[test]
fn TEST_NAME() {
    #[cfg(feature = "dhat-heap")]
    use dhat::Profiler;
    #[cfg(feature = "dhat-heap")]
    let _profiler = Profiler::new_heap();
    test("FOLDER_NAME/DIR_NAME");
}
