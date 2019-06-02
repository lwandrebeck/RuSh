extern crate rush;

use rush::opt::Opt;
use rush::opt::OptionRW;
use rush::variables::Access;

#[test]
fn test_opt_get() {
    let o = Opt::init_set_options();
    match o.get("notify") {
        Some(v) => {
            assert_eq!(v.set, false);
            assert_eq!(v.access, Access::ReadWrite);
        }
        None => panic!("notify set option should be defined."),
    }
}

#[test]
fn test_opt_init_set_options() {
    let o = Opt::init_set_options();
    match o.get("xtrace") {
        Some(v) => {
            assert_eq!(v.set, false);
            assert_eq!(v.access, Access::ReadWrite);
        }
        None => panic!("xtrace set option should be defined."),
    }
}

#[test]
fn test_opt_init_shopt_options() {
    let o = Opt::init_shopt_options();
    match o.get("histappend") {
        Some(v) => {
            assert_eq!(v.set, true);
            assert_eq!(v.access, Access::ReadWrite);
        }
        None => panic!("histappend shopt option should be defined."),
    }
}

#[test]
fn test_opt_set() {
    let mut o = Opt::init_shopt_options();
    o.set(
        String::from("opttest"),
        OptionRW {
            set: false,
            access: Access::ReadOnly,
        },
    );
    match o.get("opttest") {
        Some(v) => {
            assert_eq!(v.set, false);
            assert_eq!(v.access, Access::ReadOnly);
        }
        None => panic!("opttest shopt option should be defined."),
    }
}
