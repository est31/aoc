use super::*;

#[test]
fn test_1() {
	assert_eq!(parse_num(b"FBFBBFF"), 44);
	assert_eq!(parse_num(b"RLR"), 5);
	assert_eq!(parse_row_col_seat_id("FBFBBFFRLR"), (44, 5, 357));

	assert_eq!(parse_row_col_seat_id("BFFFBBFRRR"), (70, 7, 567));
	assert_eq!(parse_row_col_seat_id("FFFBBBFRRR"), (14, 7, 119));
	assert_eq!(parse_row_col_seat_id("BBFFBBFRLL"), (102, 4, 820));
}
