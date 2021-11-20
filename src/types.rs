use std::mem;

#[repr(u8)]
pub(crate) enum SelectorType {
    Tag,
    String,
    Selector,
    Root,
    Pseudo,
    Nesting,
    Id,
    Comment,
    Combinator,
    Class,
    Attribute,
    Universal,
}

impl From<u8> for SelectorType {
    #[inline]
    fn from(it: u8) -> Self {
        assert!(it <= <u8>::from(SelectorType::Universal));
        // SAFETY: Always valid 'cause we assert it above
        unsafe { mem::transmute(it) }
    }
}

impl From<SelectorType> for u8 {
    #[inline]
    fn from(it: SelectorType) -> Self {
        // SAFETY: Always valid 'cause SelectorType is indeed a `u8` in memory representation.
        unsafe { mem::transmute(it) }
    }
}
