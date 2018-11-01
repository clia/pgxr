#![allow(non_snake_case)]

pub mod bindings;
pub mod macros;

use bindings::*;
use std::ffi::CString;
use std::os::raw::c_char;

pub fn PG_GETARG_DATUM(fcinfo: FunctionCallInfo, n: usize) -> Datum
{
    let val = unsafe { (&*fcinfo).arg[n] };
    val
}

pub fn PG_GETARG_ISIZE(fcinfo: FunctionCallInfo, n: usize) -> isize
{
    let val = unsafe { (&*fcinfo).arg[n] };
    val as isize
}

pub fn PG_GETARG_USIZE(fcinfo: FunctionCallInfo, n: usize) -> usize
{
    let val = unsafe { (&*fcinfo).arg[n] };
    val as usize
}

pub fn PG_GETARG_I8(fcinfo: FunctionCallInfo, n: usize) -> i8
{
    let val = unsafe { (&*fcinfo).arg[n] };
    val as i8
}

pub fn PG_GETARG_I16(fcinfo: FunctionCallInfo, n: usize) -> i16
{
    let val = unsafe { (&*fcinfo).arg[n] };
    val as i16
}

pub fn PG_GETARG_I32(fcinfo: FunctionCallInfo, n: usize) -> i32
{
    let val = unsafe { (&*fcinfo).arg[n] };
    val as i32
}

pub fn PG_GETARG_I64(fcinfo: FunctionCallInfo, n: usize) -> i64
{
    let val = unsafe { (&*fcinfo).arg[n] };
    val as i64
}

pub fn PG_GETARG_U8(fcinfo: FunctionCallInfo, n: usize) -> u8
{
    let val = unsafe { (&*fcinfo).arg[n] };
    val as u8
}

pub fn PG_GETARG_U16(fcinfo: FunctionCallInfo, n: usize) -> u16
{
    let val = unsafe { (&*fcinfo).arg[n] };
    val as u16
}

pub fn PG_GETARG_U32(fcinfo: FunctionCallInfo, n: usize) -> u32
{
    let val = unsafe { (&*fcinfo).arg[n] };
    val as u32
}

pub fn PG_GETARG_U64(fcinfo: FunctionCallInfo, n: usize) -> u64
{
    let val = unsafe { (&*fcinfo).arg[n] };
    val as u64
}

pub fn PG_GETARG_CSTRING(fcinfo: FunctionCallInfo, n: usize) -> CString
{
    let c = unsafe { (&*fcinfo).arg[n] as *mut c_char };
    let cs = unsafe { CString::from_raw(c) };
    cs
}

pub fn PG_GETARG_STRING(fcinfo: FunctionCallInfo, n: usize) -> String
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
