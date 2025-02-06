use termal::{
    codes, error::Result, formatc, printcln, raw::{enable_raw_mode, term_size}, reset_terminal
};

pub fn show_bell() -> Result<()> {
    enable_raw_mode()?;
    println!("{}", codes::BELL);
    reset_terminal();
    Ok(())
}

pub fn show_backspace() -> Result<()> {
    let mut buf = String::new();

    buf += "Some test";
    buf.push(codes::BACKSPACE);
    buf.push(codes::BACKSPACE);
    buf += "x";

    println!("{buf}");

    Ok(())
}

pub fn show_htab() -> Result<()> {
    println!("1\t: number");
    println!("hello\t: greeting");
    Ok(())
}

pub fn show_newline() -> Result<()> {
    println!("normal:");
    println!("one\ntwo");

    println!("raw:");
    enable_raw_mode()?;
    println!("one\ntwo\r");

    reset_terminal();

    Ok(())
}

pub fn show_vtab() -> Result<()> {
    let mut buf = String::new();

    buf += "hello";
    buf.push(codes::VTAB);
    buf += "there";

    println!("{buf}");

    Ok(())
}

pub fn show_formfeed() -> Result<()> {
    let mut buf = String::new();

    buf += "hello";
    buf.push(codes::FORMFEED);
    buf += "there";

    println!("{buf}");

    Ok(())
}

pub fn show_carriage_return() -> Result<()> {
    println!("hello me\rgreet");
    Ok(())
}

pub fn show_delete() -> Result<()> {
    let mut buf = String::new();

    buf += "hello";
    buf.push(codes::BACKSPACE);
    buf.push(codes::DELETE);

    println!("{buf}");

    Ok(())
}

pub fn show_move_to() -> Result<()> {
    let mut buf = String::new();
    buf += codes::ERASE_ALL;

    let txt = "centered";
    let size = term_size()?;
    let x = (size.char_width - txt.len() + 1) / 2;
    let y = size.char_height / 2;
    // If one of arguments is not literal, produces string.
    let center: String = codes::move_to!(x, y);
    buf += &center;
    buf += txt;

    // With literals, it constructs static slice.
    let home: &'static str = codes::move_to!(1, 1);
    buf += home;
    buf += "top left";

    // Move to the second to last line from bottom.
    buf += &codes::move_to!(0, size.char_height - 1);

    println!("{}", buf);

    Ok(())
}

pub fn show_move_up_down() -> Result<()> {
    assert_eq!(formatc!("{'mu5}"), codes::move_up!(5));
    assert_eq!(formatc!("{'md5}"), codes::move_down!(5));
    assert_eq!(formatc!("{'mu}"), codes::move_up!(1));
    assert_eq!(formatc!("{'md}"), codes::move_down!(1));

    printcln!("{'clear}\n\nhello{'mu2}up{'md}down{'md}");
    Ok(())
}

pub fn show_move_right_left() -> Result<()> {
    assert_eq!(formatc!("{'mr5}"), codes::move_right!(5));
    assert_eq!(formatc!("{'ml5}"), codes::move_left!(5));

    printcln!("{'clear}{'mr7}there{'ml11}hello");
    Ok(())
}
