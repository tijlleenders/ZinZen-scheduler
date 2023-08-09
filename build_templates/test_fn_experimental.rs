
    #[test]
    #[cfg_attr(not(feature = "experimental-testset"), ignore)]
    fn TEST_NAME() {
        let (actual_output, desired_output) = run_test("experimental/DIR_NAME");
        assert_eq!(actual_output, desired_output);
    }
