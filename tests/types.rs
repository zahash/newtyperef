//! Types are kept separately this this file too also test visibility (pub)

use newtyperef::newtyperef;

#[newtyperef]
pub struct UsizeUsizeUsize(pub usize);

#[newtyperef]
pub struct StringStringString(pub String);

#[newtyperef(ref = str)]
pub struct StringStrString(pub String);

#[newtyperef(mut = str)]
pub struct StringStringStr(pub String);

#[newtyperef(ref = str, mut = str)]
pub struct StringStrStr(pub String);

#[newtyperef]
pub struct VecUsizeVecUsizeVecUsize(pub Vec<usize>);

#[newtyperef]
pub struct VecStringVecStringVecString(pub Vec<String>);

#[newtyperef(ref = [String])]
pub struct VecStringSliceStringVecString(pub Vec<String>);

#[newtyperef(mut = [String])]
pub struct VecStringVecStringSliceString(pub Vec<String>);

#[newtyperef(ref = [String], mut = [String])]
pub struct VecStringSliceStringSliceString(pub Vec<String>);

#[newtyperef]
pub struct Generic<T>(pub T);

#[newtyperef(ref = [T])]
pub struct GenericItems<T>(pub Vec<T>);
