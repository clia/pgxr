#![allow(non_snake_case)]

pub mod bindings;
pub mod macros;

use bindings::*;
use std::ffi::CString;
use std::os::raw::c_char;

pub fn PG_GETARG_DATUM(fcinfo: &FunctionCallInfoData, n: usize) -> Datum
{
    fcinfo.arg[n]
}

pub fn PG_GETARG_ISIZE(fcinfo: &FunctionCallInfoData, n: usize) -> isize
{
    fcinfo.arg[n] as isize
}

pub fn PG_GETARG_USIZE(fcinfo: &FunctionCallInfoData, n: usize) -> usize
{
    fcinfo.arg[n] as usize
}

pub fn PG_GETARG_I8(fcinfo: &FunctionCallInfoData, n: usize) -> i8
{
    fcinfo.arg[n] as i8
}

pub fn PG_GETARG_I16(fcinfo: &FunctionCallInfoData, n: usize) -> i16
{
    fcinfo.arg[n] as i16
}

pub fn PG_GETARG_I32(fcinfo: &FunctionCallInfoData, n: usize) -> i32
{
    fcinfo.arg[n] as i32
}

pub fn PG_GETARG_I64(fcinfo: &FunctionCallInfoData, n: usize) -> i64
{
    fcinfo.arg[n] as i64
}

pub fn PG_GETARG_U8(fcinfo: &FunctionCallInfoData, n: usize) -> u8
{
    fcinfo.arg[n] as u8
}

pub fn PG_GETARG_U16(fcinfo: &FunctionCallInfoData, n: usize) -> u16
{
    fcinfo.arg[n] as u16
}

pub fn PG_GETARG_U32(fcinfo: &FunctionCallInfoData, n: usize) -> u32
{
    fcinfo.arg[n] as u32
}

pub fn PG_GETARG_U64(fcinfo: &FunctionCallInfoData, n: usize) -> u64
{
    fcinfo.arg[n] as u64
}

pub fn PG_GETARG_CSTRING(fcinfo: &FunctionCallInfoData, n: usize) -> CString
{
    let c = fcinfo.arg[n] as *mut c_char;
    let cs = unsafe { CString::from_raw(c) };
    cs
}

pub fn PG_GETARG_STRING(fcinfo: &FunctionCallInfoData, n: usize) -> String
{
    PG_GETARG_CSTRING(fcinfo, n).into_string().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
