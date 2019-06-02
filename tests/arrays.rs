extern crate rush;

use rush::arrays::{Array, Index};
use rush::variables::{Access, Value};

#[test]
fn test_init_shell_array_vars() {
    let array = Array::init_shell_array_vars();
    match array.get("RUSH_VERSINFO", &Index::I(1)) {
        // RUSH_VERSINFO[1]=0
        Some(v) => match v {
            Value::I(i) => assert_eq!(i, 0),
            _ => panic!("RUSH_VERSINFO[1] should be Value::I."),
        },
        None => panic!("RUSH_VERSINFO[1] should be defined."),
    }
    match array.get("RUSH_VERSINFO", &Index::I(4)) {
        Some(v) => match v {
            Value::S(s) => assert_eq!(s, "alpha0"),
            _ => panic!("RUSH_VERSINFO[4] should be Value::S."),
        },
        None => panic!("RUSH_VERSINFO[4] should be defined."),
    }
}

#[test]
fn test_unset() {
    let mut array = Array::init_shell_array_vars();
    match array.get("RUSH_VERSINFO", &Index::I(1)) {
        Some(v) => match v {
            Value::I(i) => assert_eq!(i, 0),
            _ => panic!("RUSH_VERSINFO[1] should be Value::I."),
        },
        None => panic!("RUSH_VERSINFO[1] should be defined."),
    }
    array.unset("RUSH_VERSINFO", &Index::I(1));
    match array.get("RUSH_VERSINFO", &Index::I(1)) {
        Some(_v) => panic!("RUSH_VERSINFO[1] should have been unset."),
        None => println!("RUSH_VERSINFO[1] is not set."),
    }
}

#[test]
fn test_get_and_getifs() {
    let mut array = Array::init_shell_array_vars();
    match array.get("RUSH_VERSINFO", &Index::I(4)) {
        Some(v) => match v {
            Value::S(s) => assert_eq!(s, "alpha0"),
            _ => panic!("RUSH_VERSINFO[4] should be Value::S."),
        },
        None => panic!("RUSH_VERSINFO[4] should be defined."),
    }
    match array.get("RUSH_VERSINFO", &Index::I(1)) {
        Some(v) => match v {
            Value::I(i) => assert_eq!(i, 0),
            _ => panic!("RUSH_VERSINFO[1] should be Value::I."),
        },
        None => panic!("RUSH_VERSINFO[1] should be defined."),
    }
    array.set("TEST", Index::A("IT".to_string()), Value::F(-49.3));
    match array.get("TEST", &Index::A("IT".to_string())) {
        Some(v) => match v {
            Value::F(f) => assert_eq!(f, -49.3),
            _ => panic!("TEST[IT] should be Value::F."),
        },
        None => panic!("TEST[IT] variable should be defined."),
    }
    array.set("TEST", Index::I(0), Value::F(-49.3));
    match array.get("TEST", &Index::I(0)) {
        Some(v) => match v {
            Value::F(f) => assert_eq!(f, -49.3),
            _ => panic!("TEST[0] should be Value::F."),
        },
        None => panic!("TEST[0] variable should be defined."),
    }
}

#[test]
fn test_set_access() {
    let mut array = Array::init_shell_array_vars();
    array.set("TESTF", Index::A("BLA".to_string()), Value::F(-49.3));
    array.set_access("TESTF", Access::ReadOnly);
    assert_eq!(array.get_access("TESTF"), Some(Access::ReadOnly));
    array.set_access("TESTF", Access::ReadWrite);
    assert_eq!(array.get_access("TESTF"), Some(Access::ReadWrite));
    match array.get("TESTF", &Index::A("BLA".to_string())) {
        Some(f) => match f {
            Value::F(f) => assert_eq!(f, -49.3),
            _ => panic!("TESTF[\"BLA\"] should be Value::F."),
        },
        None => panic!("TESTF[\"BLA\"] variable should be defined."),
    }
    array.set_access("nonexistingentry", Access::ReadOnly);
    assert_eq!(array.get_access("nonexistingentry"), Some(Access::ReadOnly));
    array.set_access("nonexistingentry2", Access::ReadWrite);
    assert_eq!(
        array.get_access("nonexistingentry2"),
        Some(Access::ReadWrite)
    );
    array.set("nonexistingentry2", Index::I(0), Value::I(1));
    match array.get("nonexistingentry2", &Index::I(0)) {
        Some(i) => match i {
            Value::I(i) => assert_eq!(i, 1),
            _ => panic!("nonexistingentry2[0] should be Value::I."),
        },
        None => panic!("nonexistingentry2[0] should be defined and Value::I."),
    }
}

#[test]
fn test_set() {
    let mut array = Array::init_shell_array_vars();
    array.set("TESTF", Index::A("A".to_string()), Value::F(-49.3));
    match array.get("TESTF", &Index::A("A".to_string())) {
        Some(v) => match v {
            Value::F(f) => assert_eq!(f, -49.3),
            _ => panic!("TESTF[A] should be Value::F."),
        },
        None => panic!("TESTF[A] should be defined."),
    }
    array.set("TESTI", Index::I(42), Value::I(-42));
    match array.get("TESTI", &Index::I(42)) {
        Some(v) => match v {
            Value::I(i) => assert_eq!(i, -42),
            _ => panic!("TESTI[42] should be Value::I."),
        },
        None => panic!("TESTI[42] should be defined."),
    }
    array.set(
        "TESTS",
        Index::A("or not".to_string()),
        Value::S(String::from("RuSh will rock (one day)")),
    );
    match array.get("TESTS", &Index::A("or not".to_string())) {
        Some(v) => match v {
            Value::S(s) => assert_eq!(s, "RuSh will rock (one day)"),
            _ => panic!("TESTS[or not] should be Value::I."),
        },
        None => panic!("TESTS[or not] array should be defined."),
    }
}
