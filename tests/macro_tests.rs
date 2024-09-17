mod types;
use std::ops::{Deref, DerefMut};

use types::*;

#[test]
fn test_usize_usize() {
    let mut val = UsizeUsize(25);

    let ref_: UsizeUsizeRef<'_> = val.as_ref();
    let _ref_inner: &usize = ref_.deref();

    let mut mut_: UsizeUsizeRefMut<'_> = val.as_mut();
    let _mut_inner: &usize = mut_.deref();
    let _mut_inner: &mut usize = mut_.deref_mut();
}

#[test]
fn test_string_string() {
    let mut val = StringString("test_string_string".into());

    let ref_: StringStringRef<'_> = val.as_ref();
    let _ref_inner: &String = ref_.deref();

    let mut mut_: StringStringRefMut<'_> = val.as_mut();
    let _mut_inner: &String = mut_.deref();
    let _mut_inner: &mut String = mut_.deref_mut();
}

#[test]
fn test_string_str() {
    let mut val: StringStr = StringStr("test_string_str".into());

    let ref_: StringStrRef<'_> = val.as_ref();
    let _ref_inner: &str = ref_.deref();

    let mut mut_: StringStrRefMut<'_> = val.as_mut();
    let _mut_inner: &str = mut_.deref();
    let _mut_inner: &mut str = mut_.deref_mut();
}

#[test]
fn test_vec_usize_vec_usize() {
    let mut val: VecUsizeVecUsize = VecUsizeVecUsize(vec![1, 2, 3]);

    let ref_: VecUsizeVecUsizeRef<'_> = val.as_ref();
    let _ref_inner: &Vec<usize> = ref_.deref();

    let mut mut_: VecUsizeVecUsizeRefMut<'_> = val.as_mut();
    let _mut_inner: &Vec<usize> = mut_.deref();
    let _mut_inner: &mut Vec<usize> = mut_.deref_mut();
}

#[test]
fn test_vec_string_vec_string() {
    let mut val: VecStringVecString = VecStringVecString(vec!["test_vec_string_vec_string".into()]);

    let ref_: VecStringVecStringRef<'_> = val.as_ref();
    let _ref_inner: &Vec<String> = ref_.deref();

    let mut mut_: VecStringVecStringRefMut<'_> = val.as_mut();
    let _mut_inner: &Vec<String> = mut_.deref();
    let _mut_inner: &mut Vec<String> = mut_.deref_mut();
}

#[test]
fn test_vec_string_slice_string() {
    let mut val: VecStringSliceString =
        VecStringSliceString(vec!["test_vec_string_slice_string".into()]);

    let ref_: VecStringSliceStringRef<'_> = val.as_ref();
    let _ref_inner: &[String] = ref_.deref();

    let mut mut_: VecStringSliceStringRefMut<'_> = val.as_mut();
    let _mut_inner: &[String] = mut_.deref();
    let _mut_inner: &mut [String] = mut_.deref_mut();
}
