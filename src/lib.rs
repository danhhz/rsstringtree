// Copyright 2017 Daniel Harrison. All Rights Reserved.

// TODO(dan): Rustdoc for all of this.

use std::io::Write;
use std::vec;

pub trait StringTree {
    fn get_ref(&self) -> &[u8];
    fn to_string(&self) -> String { String::from_utf8(vec::Vec::from(self.get_ref())).unwrap() }
}

impl StringTree {
    pub fn new(s: &[&StringTree]) -> StringTreeBuf {
        let mut buf: std::vec::Vec<u8> = std::vec::Vec::new();
        for c in s {
            // TODO(dan): Real error handling.
            buf.write(c.get_ref()).unwrap();
        }
        StringTreeBuf { buf: buf }
    }

    pub fn indent(s: &'static str) -> StringTreeIndent {
        StringTreeIndent {
            s: s,
            level: 0,
            buf: std::vec::Vec::new(),
        }
    }
}

pub type StringTreeStr = &'static str;

impl StringTree for StringTreeStr {
    fn get_ref<'a>(&'a self) -> &'a [u8] { return self.as_bytes(); }
}

// TODO(dan): Make this private.
pub struct StringTreeBuf {
    buf: std::vec::Vec<u8>,
}

impl StringTree for StringTreeBuf {
    fn get_ref<'a>(&'a self) -> &'a [u8] { return self.buf.as_slice(); }
}

pub struct StringTreeIndent {
    s: &'static str,
    level: i64,
    buf: std::vec::Vec<u8>,
}

impl StringTreeIndent {
    pub fn next(&self) -> StringTreeIndent {
        let level = self.level + 1;
        let mut buf: std::vec::Vec<u8> = std::vec::Vec::new();
        for _ in 0..level {
            // TODO(dan): Real error handling.
            buf.write(self.s.as_bytes()).unwrap();
        }
        StringTreeIndent {
            s: self.s,
            level: level,
            buf: buf,
        }
    }
}

impl StringTree for StringTreeIndent {
    fn get_ref(&self) -> &[u8] { return self.buf.as_slice(); }
}

pub type StringTreeOption<'a> = Option<&'a StringTree>;

impl<'a> StringTree for StringTreeOption<'a> {
    fn get_ref(&self) -> &[u8] {
        match self {
            &Some(x) => x.get_ref(),
            &None => &[],
        }
    }
}

pub type StringTreeFn = Box<Fn() -> &'static str>;

impl StringTree for StringTreeFn {
    fn get_ref(&self) -> &[u8] { return self().as_bytes(); }
}


#[cfg(test)]
mod test {
    use super::{StringTree, StringTreeFn, StringTreeOption};

    use std::option;

    #[test]
    fn test_stringtree_indent() {
        let i0 = StringTree::indent(&"x");
        assert_eq!(i0.to_string(), "");
        assert_eq!(i0.next().to_string(), "x");
        assert_eq!(i0.next().next().to_string(), "xx");

        // None of the .next() calls should have mutated i0.
        assert_eq!(i0.to_string(), "");
    }

    fn strfoo() -> &'static str { "foo" }

    #[test]
    fn test_stringtree_new() {
        assert_eq!(StringTree::new(&[]).to_string(), "");
        assert_eq!(StringTree::new(&[&"foo"]).to_string(), "foo");
        assert_eq!(StringTree::new(&[&"foo", &"bar"]).to_string(), "foobar");

        let i2 = StringTree::indent("x").next().next();
        assert_eq!(StringTree::new(&[&i2, &"bar"]).to_string(), "xxbar");

        let f: StringTreeFn = Box::new(strfoo);
        assert_eq!(StringTree::new(&[&f]).to_string(), "foo");
        assert_eq!(StringTree::new(&[&"bar", &f]).to_string(), "barfoo");

        let none: StringTreeOption = option::Option::None;
        let c: &StringTree = &StringTree::new(&[&"opt"]);
        let some: StringTreeOption = option::Option::Some(c);
        assert_eq!(StringTree::new(&[&none]).to_string(), "");
        assert_eq!(StringTree::new(&[&some]).to_string(), "opt");
        assert_eq!(StringTree::new(&[&"foo", &none, &some]).to_string(), "fooopt");
    }
}
