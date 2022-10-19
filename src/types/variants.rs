use alloc::{borrow::Cow, collections::BTreeMap};

use crate::types::{Tag, fields::Field};

#[derive(Debug, Clone)]
pub struct Variants {
    fields: Cow<'static, [Field]>,
}

impl Variants {
    pub const fn new(fields: Cow<'static, [Field]>) -> Self {
        Self { fields }
    }

    pub const fn smallest_tag(&self) -> Tag {
        let i = self.fields.len();
        let mut tag: Tag = Tag::new_private(u32::MAX);

        while i < self.fields.len() {
            let field = &self.fields[i];
            if field.tag.const_less_than(tag) {
                tag = field.tag;
            }
        }

        tag
    }

    /// Returns the canonical sorted version of `self`.
    pub fn canonised(mut self) -> Self {
        self.canonical_sort();
        self
    }

    /// Sorts the fields by their canonical tag order.
    pub fn canonical_sort(&mut self) {
        self.fields.to_mut().sort_by(|(a, _), (b, _)| a.cmp(&b));
    }

    pub fn iter(&self) -> impl Iterator<Item = Field> + '_ {
        self.fields.iter().copied()
    }
}


