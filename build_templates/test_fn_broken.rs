
    #[test]
    #[cfg_attr(not(feature = "broken-testset"), ignore)]
    fn TEST_NAME() {
        let (actual_output, desired_output) = run_test("broken/DIR_NAME");
        assert_eq!(actual_output, desired_output);
    }
