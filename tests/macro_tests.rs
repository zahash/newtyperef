mod types;
use std::ops::{Deref, DerefMut};

use types::*;

#[test]
fn test_usize_usize_usize() {
    let mut val = UsizeUsizeUsize(25);

    let ref_: UsizeUsizeUsizeRef<'_> = val.as_ref();
    let _: &usize = ref_.deref();

    let mut mut_: UsizeUsizeUsizeRefMut<'_> = val.as_mut();
    let _: &usize = mut_.deref();
    let _: &mut usize = mut_.deref_mut();
}

#[test]
fn test_string_string_string() {
    let mut val = StringStringString("test_string_string_string".into());

    let ref_: StringStringStringRef<'_> = val.as_ref();
    let _: &String = ref_.deref();

    let mut mut_: StringStringStringRefMut<'_> = val.as_mut();
    let _: &String = mut_.deref();
    let _: &mut String = mut_.deref_mut();
}

#[test]
fn test_string_str_string() {
    let mut val: StringStrString = StringStrString("test_string_str_string".into());

    let ref_: StringStrStringRef<'_> = val.as_ref();
    let _: &str = ref_.deref();

    let mut mut_: StringStrStringRefMut<'_> = val.as_mut();
    let _: &String = mut_.deref();
    let _: &mut String = mut_.deref_mut();
}

#[test]
fn test_string_string_str() {
    let mut val: StringStringStr = StringStringStr("test_string_string_str".into());

    let ref_: StringStringStrRef<'_> = val.as_ref();
    let _: &String = ref_.deref();

    let mut mut_: StringStringStrRefMut<'_> = val.as_mut();
    let _: &str = mut_.deref();
    let _: &mut str = mut_.deref_mut();
}

#[test]
fn test_string_str_str() {
    let mut val = StringStrStr("test_string_str_str".into());

    let ref_: StringStrStrRef<'_> = val.as_ref();
    let _: &str = ref_.deref();

    let mut mut_: StringStrStrRefMut<'_> = val.as_mut();
    let _: &str = mut_.deref();
    let _: &mut str = mut_.deref_mut();
}

#[test]
fn test_vec_usize_vec_usize_vec_usize() {
    let mut val: VecUsizeVecUsizeVecUsize = VecUsizeVecUsizeVecUsize(vec![1, 2, 3]);

    let ref_: VecUsizeVecUsizeVecUsizeRef<'_> = val.as_ref();
    let _: &Vec<usize> = ref_.deref();

    let mut mut_: VecUsizeVecUsizeVecUsizeRefMut<'_> = val.as_mut();
    let _: &Vec<usize> = mut_.deref();
    let _: &mut Vec<usize> = mut_.deref_mut();
}

#[test]
fn test_vec_string_vec_string_vec_string() {
    let mut val: VecStringVecStringVecString =
        VecStringVecStringVecString(vec!["test_vec_string_vec_string".into()]);

    let ref_: VecStringVecStringVecStringRef<'_> = val.as_ref();
    let _: &Vec<String> = ref_.deref();

    let mut mut_: VecStringVecStringVecStringRefMut<'_> = val.as_mut();
    let _: &Vec<String> = mut_.deref();
    let _: &mut Vec<String> = mut_.deref_mut();
}

#[test]
fn test_vec_string_slice_string_vec_string() {
    let mut val: VecStringSliceStringVecString =
        VecStringSliceStringVecString(vec!["test_vec_string_slice_string".into()]);

    let ref_: VecStringSliceStringVecStringRef<'_> = val.as_ref();
    let _: &[String] = ref_.deref();

    let mut mut_: VecStringSliceStringVecStringRefMut<'_> = val.as_mut();
    let _: &Vec<String> = mut_.deref();
    let _: &mut Vec<String> = mut_.deref_mut();
}

#[test]
fn test_vec_string_vec_string_slice_string() {
    let mut val: VecStringVecStringSliceString =
        VecStringVecStringSliceString(vec!["test_vec_string_vec_string_slice_string".into()]);

    let ref_: VecStringVecStringSliceStringRef<'_> = val.as_ref();
    let _: &Vec<String> = ref_.deref();

    let mut mut_: VecStringVecStringSliceStringRefMut<'_> = val.as_mut();
    let _: &[String] = mut_.deref();
    let _: &mut [String] = mut_.deref_mut();
}

#[test]
fn test_vec_string_slice_string_slice_string() {
    let mut val: VecStringSliceStringSliceString =
        VecStringSliceStringSliceString(vec!["test_vec_string_slice_string_slice_string".into()]);

    let ref_: VecStringSliceStringSliceStringRef<'_> = val.as_ref();
    let _: &[String] = ref_.deref();

    let mut mut_: VecStringSliceStringSliceStringRefMut<'_> = val.as_mut();
    let _: &[String] = mut_.deref();
    let _: &mut [String] = mut_.deref_mut();
}
