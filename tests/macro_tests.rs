mod types;
use std::ops::{Deref, DerefMut};

use types::*;

#[test]
fn test_usize_usize() {
    let mut val = UsizeUsize(25);

    let ref_: UsizeUsizeRef<'_> = val.as_ref();
    let _ref_inner: &usize = ref_.deref();

    let mut_: UsizeUsizeRefMut<'_> = val.as_mut();
    let _mut_inner: &usize = mut_.deref();

    let mut mut_: UsizeUsizeRefMut<'_> = val.as_mut();
    let _mut_inner: &mut usize = mut_.deref_mut();
}

#[test]
fn test_string_string() {
    let mut val = StringString("test_string_string".into());

    let ref_: StringStringRef<'_> = val.as_ref();
    let _ref_inner: &String = ref_.deref();

    let mut_: StringStringRefMut<'_> = val.as_mut();
    let _mut_inner: &String = mut_.deref();

    let mut mut_: StringStringRefMut<'_> = val.as_mut();
    let _mut_inner: &mut String = mut_.deref_mut();
}

#[test]
fn test_string_str() {
    let mut val: StringStr = StringStr("test_string_str".into());

    let ref_: StringStrRef<'_> = val.as_ref();
    let _ref_inner: &str = ref_.deref();

    let mut_: StringStrRefMut<'_> = val.as_mut();
    let _mut_inner: &str = mut_.deref();

    let mut mut_: StringStrRefMut<'_> = val.as_mut();
    let _mut_inner: &mut str = mut_.deref_mut();
}
