extern crate rush;

//use crate::variables::Variables;
use rush::variables::{Access, Value, Variable, Variables};

#[test]
fn test_init_shell_vars() {
    let vars = Variables::init_shell_vars();
    match vars.get("RUSH_COMMAND") {
        Some(v) => assert_eq!(v.gets(), ""),
        None => panic!("RUSH_COMMAND should be defined."),
    }
    match vars.get("HISTSIZE") {
        Some(v) => assert_eq!(v.geti(), 1000),
        None => panic!("HISTSIZE should be defined."),
    }
}

#[test]
fn test_unset() {
    let mut vars = Variables::init_shell_vars();
    match vars.get("RUSH_COMMAND") {
        Some(v) => assert_eq!(v.gets(), ""),
        None => panic!("RUSH_COMMAND should be defined."),
    }
    vars.unset(String::from("RUSH_COMMAND"));
    match vars.get("RUSH_COMMAND") {
        Some(_v) => panic!("RUSH_COMMAND should have been unset."),
        None => println!("RUSH_COMMAND is not set."),
    }
}

#[test]
fn test_get_and_getifs() {
    let mut vars = Variables::init_shell_vars();
    match vars.get("RUSH_COMMAND") {
        Some(v) => assert_eq!(v.gets(), ""),
        None => panic!("RUSH_COMMAND should be defined."),
    }
    match vars.get("HISTSIZE") {
        Some(v) => assert_eq!(v.geti(), 1000),
        None => panic!("HISTSIZE should be defined."),
    }
    vars.set(
        String::from("TEST"),
        Variable {
            value: Value::F(-49.3),
            access: Access::ReadWrite,
        },
        );
    match vars.get("TEST") {
        Some(v) => assert_eq!(v.getf(), -49.3),
        None => panic!("TEST variable should be defined."),
    }
}

#[test]
fn test_set() {
    let mut vars = Variables::init_shell_vars();
    vars.set(
        String::from("TESTF"),
        Variable {
            value: Value::F(-49.3),
            access: Access::ReadWrite,
        },
        );
    match vars.get("TESTF") {
        Some(v) => assert_eq!(v.getf(), -49.3),
        None => panic!("TESTF should be defined."),
    }
    vars.set(
        String::from("TESTI"),
        Variable {
            value: Value::I(-42),
            access: Access::ReadWrite,
        },
        );
    match vars.get("TESTI") {
        Some(v) => assert_eq!(v.geti(), -42),
        None => panic!("TESTI should be defined."),
    }
    vars.set(
        String::from("TESTS"),
        Variable {
            value: Value::S(String::from("RuSh will rock (one day)")),
            access: Access::ReadWrite,
        },
        );
    match vars.get("TESTS") {
        Some(v) => assert_eq!(v.gets(), "RuSh will rock (one day)"),
        None => panic!("TESTS variable should be defined."),
    }
}

#[test]
fn test_get_access() {
    let vars = Variables::init_shell_vars();
    match vars.get_access("RUSH_COMMAND") {
        Some(v) => assert_eq!(v, Access::ReadWrite),
        None => panic!("RUSH_COMMAND should be defined and Access::ReadWrite."),
    }
    match vars.get_access("EUID") {
        Some(v) => assert_eq!(v, Access::ReadOnly),
        None => panic!("EUID should be defined and Access::ReadOnly."),
    }
    match vars.get_access("nonexistingvar") {
        Some(v) => panic!("nonexistingvar should not give back {:?}", v),
        None => assert!(true),
    }
}

#[test]
fn test_set_access() {
    let mut vars = Variables::init_shell_vars();
    vars.set_access("TEST".to_string(), Access::ReadWrite);
    match vars.get_access("TEST") {
        Some(v) => assert_eq!(v, Access::ReadWrite),
        None => panic!("TEST should be defined."),
    }
    vars.set_access("TEST".to_string(), Access::ReadOnly);
    match vars.get_access("TEST") {
        Some(v) => assert_eq!(v, Access::ReadOnly),
        None => panic!("TEST should be defined."),
    }
    vars.set_access("doesnotexist".to_string(), Access::ReadWrite);
    match vars.get_access("doesnotexist") {
        Some(v) => assert_eq!(v, Access::ReadWrite),
        None => panic!("doesnotexist variable should be defined and Access::ReadWrite"),
    }
    vars.set_access("doesnotexist2".to_string(), Access::ReadOnly);
    match vars.get_access("doesnotexist2") {
        Some(v) => assert_eq!(v, Access::ReadOnly),
        None => panic!("doesnotexist variable should be defined and Access::ReadOnly"),
    }
}

