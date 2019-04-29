extern crate rush;

use rush::prompt::Prompt;
use rush::rush::RuSh;

#[test]
fn test_get() {
    let mut rush = RuSh::default();
    let mut p = Prompt::get(&mut rush, "PS2");
    assert_eq!(p.prompt, ">");
    p = Prompt::get(&mut rush, "PS3");
    assert_eq!(p.prompt, ">");
    p = Prompt::get(&mut rush, "PS4");
    assert_eq!(p.prompt, ">");
}

