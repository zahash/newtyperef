//! Types are kept separately this this file too also test visibility (pub)

use newtyperef::newtyperef;

#[newtyperef]
pub struct UsizeUsize(pub usize);

#[newtyperef]
pub struct StringString(pub String);

#[newtyperef(str)]
pub struct StringStr(pub String);

#[newtyperef]
pub struct VecUsizeVecUsize(pub Vec<usize>);

#[newtyperef]
pub struct VecStringVecString(pub Vec<String>);

#[newtyperef([String])]
pub struct VecStringSliceString(pub Vec<String>);
