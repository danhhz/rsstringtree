// Copyright 2017 Daniel Harrison. All Rights Reserved.

// TODO(dan): Rustdoc for all of this.

use std::fmt;
use std::io::Write;

pub trait StringTree: fmt::Display {
    // TODO(dan): Is this really necessary?
    fn get_ref<'a>(&'a self) -> &'a [u8];
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
    fn get_ref<'a>(&'a self) -> &'a [u8] {
        return self.as_bytes();
    }
}

// TODO(dan): Make this private.
pub struct StringTreeBuf {
    buf: std::vec::Vec<u8>,
}

impl StringTree for StringTreeBuf {
    fn get_ref<'a>(&'a self) -> &'a [u8] {
        return self.buf.as_slice();
    }
}

impl fmt::Display for StringTreeBuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO(dan): Real error handling.
        write!(f, "{}", std::str::from_utf8(self.get_ref()).unwrap())
    }
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

impl fmt::Display for StringTreeIndent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO(dan): Real error handling.
        write!(f, "{}", std::str::from_utf8(self.get_ref()).unwrap())
    }
}

impl StringTree for StringTreeIndent {
    fn get_ref(&self) -> &[u8] {
        return self.buf.as_slice();
    }
}

#[cfg(test)]
mod test {
    use super::StringTree;

    #[test]
    fn test_stringtree_indent() {
        let i0 = StringTree::indent(&"x");
        assert_eq!(i0.to_string(), "");
        assert_eq!(i0.next().to_string(), "x");
        assert_eq!(i0.next().next().to_string(), "xx");

        // None of the .next() calls should have mutated i0.
        assert_eq!(i0.to_string(), "");
    }

    #[test]
    fn test_stringtree_new() {
        assert_eq!(StringTree::new(&[]).to_string(), "");
        assert_eq!(StringTree::new(&[&"foo"]).to_string(), "foo");
        assert_eq!(StringTree::new(&[&"foo", &"bar"]).to_string(), "foobar");
        let i2 = StringTree::indent("x").next().next();
        assert_eq!(StringTree::new(&[&i2, &"bar"]).to_string(), "xxbar");
    }
}
