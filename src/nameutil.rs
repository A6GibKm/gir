use std::path::*;
use std::string::String;

use case::*;


pub fn split_namespace_name(name: &str) -> (Option<&str>, &str) {
    let mut parts = name.split('.');
    let name = parts.next_back().unwrap();
    let ns = parts.next_back();
    assert!(ns.is_none() || parts.next().is_none());
    (ns, name)
}

pub fn file_name(full_name: &str) -> String {
    let (_, class_name) = split_namespace_name(full_name);
    let mut name = PathBuf::from(module_name(class_name));
    let added = name.set_extension("rs");
    assert!(added);
    name.to_str().unwrap().into()
}

pub fn file_name_sys(library_name: &str, name: &str) -> String {
    let mut path = PathBuf::from(format!("{}-sys", module_name(library_name)))
        .join("src").join(name);
    let added = path.set_extension("rs");
    assert!(added);
    path.to_str().unwrap().into()
}

pub fn module_name(name: &str) -> String {
    name.to_snake()
}

#[cfg(test)]
mod tests {
    use std::path::*;
    use super::*;

    #[test]
    fn split_no_namespace() {
        let (ns, name) = split_namespace_name("GObject");
        assert_eq!(ns, None);
        assert_eq!(name, "GObject");
    }

    #[test]
    fn split_full_name() {
        let (ns, name) = split_namespace_name("Gtk.StatusIcon");
        assert_eq!(ns, Some("Gtk"));
        assert_eq!(name, "StatusIcon");
    }

    #[test]
    fn file_name_works() {
        assert_eq!(file_name("Gtk.StatusIcon"), "status_icon.rs");
    }

    #[test]
    fn file_name_sys_works() {
        let expected: String = PathBuf::from("gtk-sys").join("src").join("funcs.rs")
            .to_str().unwrap().into();
        assert_eq!(file_name_sys("Gtk", "funcs"), expected);
    }
}
