
    #[test]
    fn TEST_NAME() {
        let (actual_output, desired_output) = run_test("TEST_NAME");
        soft::assert_eq!(actual_output, desired_output).unwrap();
    }
