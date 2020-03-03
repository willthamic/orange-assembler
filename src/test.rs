use super::*;

#[test]
fn instruction_test() {
    let tests = [
        ("add r1, r2, r3", "60443000"),
        ("andi r1,r1,1", "a8420001"),
        // ("brnz r31,r3", "403e3003"),
        // ("br r29", "403a0001"),
        // ("stop", "f8000000"),
        // ("st r1,0(r30)", "187c0000"),
        // ("", ""),
        // ("", ""),
    ];
    for test in &tests {
        let result = process_instruction(test.0).unwrap().unwrap();
        let result = encode_instruction(&result).unwrap();
        assert_eq!(result, test.1);
    }
}

#[test]
fn process_branch_test() {
    let tests = [
        ("r1", &Opcode::BR, 
        Arguments{ra: None, rb: Some(1), rc: Some(0), c1: None, c2: None, c3: None}),
        ("", &Opcode::BRNV, 
        Arguments{ra: None, rb: Some(0), rc: Some(0), c1: None, c2: None, c3: None}),
        ("r1, r2", &Opcode::BRZR, 
        Arguments{ra: None, rb: Some(1), rc: Some(2), c1: None, c2: None, c3: None}),
        ("r1, r2", &Opcode::BRNZ, 
        Arguments{ra: None, rb: Some(1), rc: Some(2), c1: None, c2: None, c3: None}),
        ("r1, r2", &Opcode::BRPL, 
        Arguments{ra: None, rb: Some(1), rc: Some(2), c1: None, c2: None, c3: None}),
        ("r1, r2", &Opcode::BRMI, 
        Arguments{ra: None, rb: Some(1), rc: Some(2), c1: None, c2: None, c3: None}),
    ];
    for test in &tests {
        let result = process_branch(test.0, test.1).unwrap();
        assert_eq!(result, test.2);
    }

    let invalid_tests = [
        ("test", &Opcode::NOP),
        ("r1, r31", &Opcode::BR),
        ("r1", &Opcode::BRNV),
        ("a3,r5", &Opcode::BRNZ),
        ("r3,, r5", &Opcode::BRNZ),
        ("r3,r5,r6", &Opcode::BRNZ),
    ];
    for test in &invalid_tests {
        let result = process_branch(test.0, test.1);
        assert!(result.is_err(), format!("failed with [{}]", test.0));
    }
}

#[test]
fn process_op_test() {
    let tests = [
        ("", 
        Arguments{ra: None, rb: None, rc: None, c1: None, c2: None, c3: None}),
        (" ", 
        Arguments{ra: None, rb: None, rc: None, c1: None, c2: None, c3: None}),
    ];
    for test in &tests {
        let result = process_op(test.0).unwrap();
        assert_eq!(result, test.1);
    }

    let invalid_tests = [
        "test"
    ];
    for test in &invalid_tests {
        let result = process_op(test);
        assert!(result.is_err());
    }
}

#[test]
fn process_op_ra_rb_rc_test() {
    let tests = [
        ("r1, r2, r3",
        Arguments{ra: Some(1), rb: Some(2), rc: Some(3), c1: None, c2: None, c3: None}),
        ("  r23,   r6,r11 ", 
        Arguments{ra: Some(23), rb: Some(6), rc: Some(11), c1: None, c2: None, c3: None}),
    ];
    for test in &tests {
        let result = process_op_ra_rb_rc(test.0).unwrap();
        assert_eq!(result, test.1);
    }

    let invalid_tests = [
        "test",
        "r1, r3, r3123",
        "r2,,r3 r3",
        "a3,b3",
    ];
    for test in &invalid_tests {
        let result = process_op_ra_rb_rc(test);
        assert!(result.is_err(), format!("failed with [{}]", test));
    }
}

#[test]
fn process_op_ra_c2_rb_test() {
    let tests = [
        ("r1, 5(r2)",
        Arguments{ra: Some(1), rb: Some(2), rc: None, c1: None, c2: Some(Con::C(5)), c3: None}),
        ("  r23,   16 ( r11 ) ",
        Arguments{ra: Some(23), rb: Some(11), rc: None, c1: None, c2: Some(Con::C(16)), c3: None}),
        ("  r23 ,16( r11)",
        Arguments{ra: Some(23), rb: Some(11), rc: None, c1: None, c2: Some(Con::C(16)), c3: None}),
        ("r1, 5",
        Arguments{ra: Some(1), rb: Some(0), rc: None, c1: None, c2: Some(Con::C(5)), c3: None}),
    ];
    for test in &tests {
        let result = process_op_ra_c2_rb(test.0).unwrap();
        assert_eq!(result, test.1);
    }

    let invalid_tests = [
        "test",
        "r1, r3, r3123",
        "r2,,r3 r3",
        "a3,b3",
    ];
    for test in &invalid_tests {
        let result = process_op_ra_c2_rb(test);
        assert!(result.is_err(), format!("failed with [{}]", test));
    }
}

#[test]
fn process_op_ra_rb_c2_test() {
    let tests = [
        ("r1, r2, 5",
        Arguments{ra: Some(1), rb: Some(2), rc: None, c1: None, c2: Some(Con::C(5)), c3: None}),
        ("  r23,     r11 , 16 ",
        Arguments{ra: Some(23), rb: Some(11), rc: None, c1: None, c2: Some(Con::C(16)), c3: None}),
        ("  r23 , r11 ,16",
        Arguments{ra: Some(23), rb: Some(11), rc: None, c1: None, c2: Some(Con::C(16)), c3: None}),
        ("r1, r1, 16",
        Arguments{ra: Some(1), rb: Some(1), rc: None, c1: None, c2: Some(Con::C(16)), c3: None}),
    ];
    for test in &tests {
        let result = process_op_ra_rb_c2(test.0).unwrap();
        assert_eq!(result, test.1);
    }

    let invalid_tests = [
        "test",
        "r1, r3,,389",
        "r2,,r3 r3",
        "a3,b3",
    ];
    for test in &invalid_tests {
        let result = process_op_ra_rb_c2(test);
        assert!(result.is_err(), format!("failed with [{}]", test));
    }
}

#[test]
fn process_op_ra_c1_test() {
    let tests = [
        ("r1, 5",
        Arguments{ra: Some(1),  rb: None, rc: None, c1: Some(Con::C(5)),  c2: None, c3: None}),
        ("  r23,   16",
        Arguments{ra: Some(23), rb: None, rc: None, c1: Some(Con::C(16)), c2: None, c3: None}),
        ("  r23,16",
        Arguments{ra: Some(23), rb: None, rc: None, c1: Some(Con::C(16)), c2: None, c3: None}),
    ];
    for test in &tests {
        let result = process_op_ra_c1(test.0).unwrap();
        assert_eq!(result, test.1);
    }

    let invalid_tests = [
        "test",
        "r1, 31a23",
        "r2,3,,,r3",
        "a3,5",
    ];
    for test in &invalid_tests {
        let result = process_op_ra_c1(test);
        assert!(result.is_err(), format!("failed with [{}]", test));
    }
}

#[test]
fn process_op_ra_rc_test() {
    let tests = [
        ("r1, r3",
        Arguments{ra: Some(1), rb: None, rc: Some(3), c1: None, c2: None, c3: None}),
        ("  r23,   r6",
        Arguments{ra: Some(23), rb: None, rc: Some(6), c1: None, c2: None, c3: None}),
        ("  r23,r31",
        Arguments{ra: Some(23), rb: None, rc: Some(31), c1: None, c2: None, c3: None}),
    ];
    for test in &tests {
        let result = process_op_ra_rc(test.0).unwrap();
        assert_eq!(result, test.1);
    }

    let invalid_tests = [
        "test",
        "r1, r31a23",
        "r2,r3,,,r3",
        "a3,r5",
    ];
    for test in &invalid_tests {
        let result = process_op_ra_rc(test);
        assert!(result.is_err(), format!("failed with [{}]", test));
    }
}

#[test]
fn process_op_ra_rb_rc_c3_test() {
    let tests = [
        ("r1, r2, r3",
        Arguments{ra: Some(1), rb: Some(2), rc: Some(3), c1: None, c2: None, c3: Some(Con::C(0))}),
        ("r1, r2, 3",
        Arguments{ra: Some(1), rb: Some(2), rc: Some(0), c1: None, c2: None, c3: Some(Con::C(3))}),
        (" r1,r2,  r3 ",
        Arguments{ra: Some(1), rb: Some(2), rc: Some(3), c1: None, c2: None, c3: Some(Con::C(0))}),
        ("r1,r2,3 ",
        Arguments{ra: Some(1), rb: Some(2), rc: Some(0), c1: None, c2: None, c3: Some(Con::C(3))}),
    ];
    for test in &tests {
        let result = process_op_ra_rb_rc_c3(test.0).unwrap();
        assert_eq!(result, test.1);
    }

    let invalid_tests = [
        "test",
        "r1, r31,a23",
        "r2,r3,,,r3",
        "a3,r5,5",
    ];
    for test in &invalid_tests {
        let result = process_op_ra_rb_rc_c3(test);
        assert!(result.is_err(), format!("failed with [{}]", test));
    }
}

#[test]
fn register_string_parse_test() {
    for i in 0..32 {
        let input = format!("r{}", i);
        let result = register_string_parse(&input).unwrap();
        assert_eq!(i, result);
    }

    let invalid_tests = [
        "a23",
        "123",
        "",
        "rrr",
        "r 23",
        "r32",
        "r-2",
    ];
    for test in &invalid_tests {
        let result = register_string_parse(test);
        assert!(result.is_err(), format!("failed with [{}]", test));
    }
}