use binrw::binrw;
use std::marker::PhantomData;

#[binrw]
pub struct Ptr<T>(i32, #[bw(ignore)] PhantomData<T>);

#[binrw]
pub struct Ref<T>(i32, #[bw(ignore)] PhantomData<T>);

#[binrw]
pub struct NiFixedString(i32);

#[binrw]
pub struct Float16([u8; 2]);

#[binrw]
pub struct NormByte(u8);
