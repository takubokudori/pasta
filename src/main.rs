use clap::{App, Arg};
winrt::import!(
    dependencies
        os
    types
        windows::application_model::data_transfer::*
);

use windows::application_model::data_transfer::{DataPackage, Clipboard};

fn get_path_str(path: &str, paren: &str, is_espace: bool) -> String {
    let mut ret = match paren {
        "(" | ")" => format!("({})", path),
        "<" | ">" => format!("<{}>", path),
        "{" | "}" => format!("{{{}}}", path),
        "[" | "]" => format!("[{}]", path),
        x => format!("{}{}{}", x, path, x),
    };
    if is_espace {
        ret = ret.replace(r"\", r"\\");
    }
    ret
}

fn copy_to_clipboard(text: &str) -> winrt::Result<()> {
    let dp = DataPackage::new()?;
    dp.set_text(text)?;
    Clipboard::set_content(dp)?;
    Clipboard::flush()?;
    Ok(())
}

fn main() {
    let m = App::new("pasta")
        .arg(Arg::new("path").takes_value(true).required(true))
        .arg(Arg::new("paren").takes_value(true).required(false))
        .arg(Arg::new("escape").short('e').long("escape"))
        .get_matches();
    let path = m.value_of("path").unwrap();
    let paren = m.value_of("paren").unwrap_or("");
    let is_escape = m.is_present("escape");
    let new_path = get_path_str(path, paren, is_escape);
    copy_to_clipboard(&new_path).expect("Failed to copy to clipboard");
}

#[test]
fn test() {
    assert_eq!(get_path_str(r"C:\foo\bar.txt", "", false), r"C:\foo\bar.txt");
    assert_eq!(get_path_str(r"C:\foo\bar.txt", "(", false), r"(C:\foo\bar.txt)");
    assert_eq!(get_path_str(r"C:\foo\bar.txt", "<", false), r"<C:\foo\bar.txt>");
    assert_eq!(get_path_str(r"C:\foo\bar.txt", "{", false), r"{C:\foo\bar.txt}");
    assert_eq!(get_path_str(r"C:\foo\bar.txt", "[", false), r"[C:\foo\bar.txt]");
    assert_eq!(get_path_str(r"C:\foo\bar.txt", "]", false), r"[C:\foo\bar.txt]");
    assert_eq!(get_path_str(r"C:\foo\bar.txt", "\"", false), "\"C:\\foo\\bar.txt\"");
    assert_eq!(get_path_str(r"C:\foo\bar.txt", "'", false), r"'C:\foo\bar.txt'");
    assert_eq!(get_path_str(r"C:\foo\bar.txt", "`", true), r"`C:\\foo\\bar.txt`");
    assert_eq!(get_path_str(r"C:\foo\bar.txt", "abc", true), r"abcC:\\foo\\bar.txtabc");
}
