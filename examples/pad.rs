extern crate console;

use console::*;

fn main() {
    let s = format!("foo {}", style("bar").red().force_styling(true));
    println!("truncate_str aaa---: '{}'", &truncate_str(&s, 5, ""));
    let s = format!("foo {}", style("bar").red().force_styling(true));
    println!("truncate_str bbb---: '{}'", &truncate_str(&s, 5, "!"));
    let s = format!("foo {} baz", style("bar").red().force_styling(true));
    println!("truncate_str ccc---: '{}'", &truncate_str(&s, 10, "..."));

    println!(
        "pad_str aaa+++: '{}'",
        pad_str("foo", 7, Alignment::Center, None)
    );
    println!(
        "pad_str 111+++: '{}'",
        pad_str("foo", 6, Alignment::Center, None)
    );
    println!(
        "pad_str 222+++: '{}'",
        pad_str("foo", 8, Alignment::Center, None)
    );
    println!(
        "pad_str bbb+++: '{}'",
        pad_str("foo", 7, Alignment::Left, None)
    );
    println!(
        "pad_str ccc+++: '{}'",
        pad_str("foo", 7, Alignment::Right, None)
    );
    println!(
        "pad_str ddd+++: '{}'",
        pad_str("foo", 3, Alignment::Left, None)
    );
    println!(
        "pad_str eee+++: '{}'",
        pad_str("foobar", 3, Alignment::Left, None)
    );
    println!(
        "pad_str fff+++: '{}'",
        pad_str("foobar", 3, Alignment::Left, Some(""))
    );
    println!(
        "pad_str ggg+++: '{}'",
        pad_str("foobarbaz", 6, Alignment::Left, Some("..."))
    );
}
