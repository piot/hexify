#[test_log::test]
fn encountered_too_small() {
    let encountered_buf = &[0xff, 0x13];
    let expected_buf = &[0u8; 32];

    assert!(hexify::compare_eq_slices(encountered_buf, expected_buf).is_err())
}

#[test_log::test]
fn expected_too_small() {
    let expected_buf = &[0xff, 0x13];
    let encountered_buf = &[0u8; 32];

    assert!(hexify::compare_eq_slices(encountered_buf, expected_buf).is_err())
}
