#[macro_use]
extern crate og_fmt;

#[test]
fn test() {
    assert_eq!(fmt!("{}", "foo"),
               format!("{}", "foo"));
    assert_eq!(::std::fmt::format(fmt_args!("{}", "foo")),
               ::std::fmt::format(format_args!("{}", "foo")));
}
