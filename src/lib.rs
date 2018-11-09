#![allow(non_snake_case)]

pub mod bindings;
pub mod macros;

use bindings::*;
use std::ffi::{ CString, CStr };
use std::os::raw::c_char;

pub fn PG_GETARG_DATUM(fcinfo: FunctionCallInfo, n: usize) -> Datum {
    let val = unsafe { (&*fcinfo).arg[n] };
    val
}

pub fn PG_GETARG_ISIZE(fcinfo: FunctionCallInfo, n: usize) -> isize {
    let val = unsafe { (&*fcinfo).arg[n] };
    val as isize
}

pub fn PG_GETARG_USIZE(fcinfo: FunctionCallInfo, n: usize) -> usize {
    let val = unsafe { (&*fcinfo).arg[n] };
    val as usize
}

pub fn PG_GETARG_I8(fcinfo: FunctionCallInfo, n: usize) -> i8 {
    let val = unsafe { (&*fcinfo).arg[n] };
    val as i8
}

pub fn PG_GETARG_I16(fcinfo: FunctionCallInfo, n: usize) -> i16 {
    let val = unsafe { (&*fcinfo).arg[n] };
    val as i16
}

pub fn PG_GETARG_I32(fcinfo: FunctionCallInfo, n: usize) -> i32 {
    let val = unsafe { (&*fcinfo).arg[n] };
    val as i32
}

pub fn PG_GETARG_I64(fcinfo: FunctionCallInfo, n: usize) -> i64 {
    let val = unsafe { (&*fcinfo).arg[n] };
    val as i64
}

pub fn PG_GETARG_U8(fcinfo: FunctionCallInfo, n: usize) -> u8 {
    let val = unsafe { (&*fcinfo).arg[n] };
    val as u8
}

pub fn PG_GETARG_U16(fcinfo: FunctionCallInfo, n: usize) -> u16 {
    let val = unsafe { (&*fcinfo).arg[n] };
    val as u16
}

pub fn PG_GETARG_U32(fcinfo: FunctionCallInfo, n: usize) -> u32 {
    let val = unsafe { (&*fcinfo).arg[n] };
    val as u32
}

pub fn PG_GETARG_U64(fcinfo: FunctionCallInfo, n: usize) -> u64 {
    let val = unsafe { (&*fcinfo).arg[n] };
    val as u64
}

// pub fn PG_GETARG_CSTRING(fcinfo: FunctionCallInfo, n: usize) -> CString {
//     let c = unsafe { (&*fcinfo).arg[n] as *mut c_char };
//     let cs = unsafe { CStr::from_ptr(c).into_c_string() };
//     cs
// }

pub fn PG_GETARG_STRING(fcinfo: FunctionCallInfo, n: usize) -> String {
    let c = unsafe { (&*fcinfo).arg[n] as *mut c_char };
    let s = unsafe { CStr::from_ptr(c).to_string_lossy().into_owned() };
    s
}

pub fn PG_RETURN_CSTRING(result: CString) -> Datum {
    result.into_raw() as Datum
}

pub fn PG_RETURN_STRING(result: String) -> Datum {
    let cs = CString::new(result).expect("CString::new failed");
    PG_RETURN_CSTRING(cs)
}

pub fn PG_RETURN_STR(result: &str) -> Datum {
    let cs = CString::new(result).expect("CString::new failed");
    PG_RETURN_CSTRING(cs)
}

pub fn PG_RETURN_I32(result: i32) -> Datum {
    result as Datum
}

pub fn PG_RETURN_I64(result: i64) -> Datum {
    result as Datum
}

pub fn PG_RETURN_ISIZE(result: isize) -> Datum {
    result as Datum
}

pub fn PG_RETURN_U32(result: u32) -> Datum {
    result as Datum
}

pub fn PG_RETURN_U64(result: u64) -> Datum {
    result as Datum
}

pub fn PG_RETURN_USIZE(result: usize) -> Datum {
    result as Datum
}

pub fn PG_RETURN_F32(result: f32) -> Datum {
    result as Datum
}

pub fn PG_RETURN_F64(result: f64) -> Datum {
    result as Datum
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
