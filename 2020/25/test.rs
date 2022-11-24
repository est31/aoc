use super::*;

#[test]
fn test_1() {
	assert_eq!(find_loop_size(5764801), 8);
	assert_eq!(transform(7, 8), 5764801);
	assert_eq!(find_loop_size(17807724), 11);
	assert_eq!(transform(7, 11), 17807724);
	assert_eq!(transform(17807724, 8), 14897079);
	assert_eq!(transform(5764801, 11), 14897079);
	assert_eq!(get_encryption_key((5764801, 17807724)), 14897079);
}
