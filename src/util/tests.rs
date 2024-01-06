use super::endianness::{as_i32_be, as_i32_le, i32_bytes_be, i32_bytes_le};

#[test]
fn test_as_i32_be() {
    let mut bytes = [0, 0, 0, 0];
    assert_eq!(as_i32_be(&bytes), 0);
    bytes = [0, 0, 0, 1];
    assert_eq!(as_i32_be(&bytes), 1);
    bytes = [0, 0, 1, 1];
    assert_eq!(as_i32_be(&bytes), 257);
    bytes = [0, 1, 1, 0];
    assert_eq!(as_i32_be(&bytes), 65792);
}

#[test]
fn test_as_i32_le() {
    let mut bytes = [0, 0, 0, 0];
    assert_eq!(as_i32_le(&bytes), 0);
    bytes = [1, 0, 0, 0];
    assert_eq!(as_i32_le(&bytes), 1);
    bytes = [0, 0, 1, 1];
    assert_eq!(as_i32_le(&bytes), 16842752);
    bytes = [0, 1, 1, 0];
    assert_eq!(as_i32_le(&bytes), 65792);
}

#[test]
fn test_i32_bytes_be() {
    let mut bytes = [0, 0, 0, 0];
    assert_eq!(i32_bytes_be(0), bytes);
    bytes = [0, 0, 0, 1];
    assert_eq!(i32_bytes_be(1), bytes);
    bytes = [0, 0, 1, 1];
    assert_eq!(i32_bytes_be(257), bytes);
    bytes = [0, 1, 1, 0];
    assert_eq!(i32_bytes_be(65792), bytes);
}

#[test]
fn test_i32_bytes_le() {
    let mut bytes = [0, 0, 0, 0];
    assert_eq!(i32_bytes_le(0), bytes);
    bytes = [1, 0, 0, 0];
    assert_eq!(i32_bytes_le(1), bytes);
    bytes = [1, 1, 0, 0];
    assert_eq!(i32_bytes_le(257), bytes);
    bytes = [0, 1, 1, 0];
    assert_eq!(i32_bytes_le(65792), bytes);
}
