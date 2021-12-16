use super::*;

#[test]
fn test_literal() -> Result<()> {
	let p = parse_packet("D2FE28")?;
	assert_eq!(p, Packet {
		version : 6,
		id : 4,
		kind : PacketKind::Literal(2021),
	});
	assert_eq!(p.version_sum(), 6);
	Ok(())
}

#[test]
fn test_operator_1() -> Result<()> {
	let p = parse_packet("38006F45291200")?;
	assert_eq!(p.version, 1);
	assert_eq!(p.id, 6);
	if let PacketKind::Operator(ps) = &p.kind {
		assert_eq!(ps.len(), 2);
		assert_eq!(ps[0].kind, PacketKind::Literal(10));
		assert_eq!(ps[1].kind, PacketKind::Literal(20));
	} else {
		panic!();
	}
	Ok(())
}

#[test]
fn test_operator_2() -> Result<()> {
	let p = parse_packet("EE00D40C823060")?;
	assert_eq!(p.version, 7);
	assert_eq!(p.id, 3);
	if let PacketKind::Operator(ps) = &p.kind {
		assert_eq!(ps.len(), 3);
		assert_eq!(ps[0].kind, PacketKind::Literal(1));
		assert_eq!(ps[1].kind, PacketKind::Literal(2));
		assert_eq!(ps[2].kind, PacketKind::Literal(3));
	} else {
		panic!();
	}
	Ok(())
}

#[test]
fn test_operator_3() -> Result<()> {
	let p = parse_packet("8A004A801A8002F478")?;
	assert_eq!(p.version, 4);
	// TODO use let_else once available
	if let PacketKind::Operator(ps) = &p.kind {
		assert_eq!(ps.len(), 1);
		assert_eq!(ps[0].version, 1);
		if let PacketKind::Operator(ps) = &ps[0].kind {
			assert_eq!(ps[0].version, 5);
			if let PacketKind::Operator(ps) = &ps[0].kind {
				assert_eq!(ps[0].version, 6);
				// TODO use assert_matches once available
				assert!(matches!(ps[0].kind, PacketKind::Literal(_)));
			} else {
				panic!()
			}
		} else {
			panic!()
		}
	} else {
		panic!();
	}
	assert_eq!(p.version_sum(), 16);
	Ok(())
}

#[test]
fn test_operator_4() -> Result<()> {
	let p = parse_packet("620080001611562C8802118E34")?;
	assert_eq!(p.version, 3);
	// TODO more detailed testing
	assert_eq!(p.version_sum(), 12);
	Ok(())
}

#[test]
fn test_operator_5() -> Result<()> {
	let p = parse_packet("C0015000016115A2E0802F182340")?;
	// TODO more detailed testing
	assert_eq!(p.version_sum(), 23);
	Ok(())
}

#[test]
fn test_operator_6() -> Result<()> {
	let p = parse_packet("A0016C880162017C3686B18A3D4780")?;
	// TODO more detailed testing
	assert_eq!(p.version_sum(), 31);
	Ok(())
}

#[test]
fn test_operator_ext_1() -> Result<()> {
	let p = parse_packet("C200B40A82")?;
	assert_eq!(p.compute_value(), 3);
	Ok(())
}
