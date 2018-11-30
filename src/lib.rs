#![allow(non_snake_case)]

pub mod bindings;
pub mod macros;

use bindings::*;
use std::ffi::{ CString, CStr };
use std::os::raw::c_char;
use std::fmt;

///
/// Result and Error definitions
/// 

pub type PgxrResult<T> = std::result::Result<T, PgxrError>;

#[derive(Debug)]
pub struct PgxrError {
  repr: ErrorRepr,
}

#[derive(Debug)]
enum ErrorRepr {
  IntenalError(ErrorKind, String),
}

/// An enum of all error kinds.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ErrorKind {
    /// The param index is out of the boundary.
    IndexOutOfBound,
    /// Operation failed because of a type mismatch.
    TypeError,
}

impl PgxrError {
    pub fn new(kind: ErrorKind, msg: String) -> PgxrError {
        PgxrError {
            repr: ErrorRepr::IntenalError(kind, msg),
        }
    }

  pub fn kind(&self) -> ErrorKind {
    match self.repr {
      ErrorRepr::IntenalError(kind, ref _msg) => kind,
    }
  }
}

impl fmt::Display for PgxrError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.repr {
      ErrorRepr::IntenalError(_kind, ref err_str) => f.pad(err_str),
    }
  }
}

impl std::error::Error for PgxrError {
  fn description(&self) -> &str {
    match self.repr {
      ErrorRepr::IntenalError(_kind, ref msg) => msg.as_str(),
    }
  }

  fn cause(&self) -> Option<&std::error::Error> {
    match self.repr {
      ErrorRepr::IntenalError(_kind, ref _msg) => None,
    }
  }
}

///
/// Helper macros
/// 

macro_rules! CHECK_PARAM_INDEX {
    ( $fcinfo:expr, $n:expr ) => {
        unsafe {
            if $n >= (&*$fcinfo).nargs as usize {
                return Err(PgxrError::new(ErrorKind::IndexOutOfBound, format!("Index out of boundary, total {}, n is {}", (&*$fcinfo).nargs, $n)));
            }
        }
    };
}

///
/// Get Argument functions
/// 

pub fn PG_GETARG_DATUM(fcinfo: FunctionCallInfo, n: usize) -> PgxrResult<Datum> {
    CHECK_PARAM_INDEX!(fcinfo, n);
    let val = unsafe { (&*fcinfo).arg[n] };
    Ok(val)
}

pub fn PG_GETARG_ISIZE(fcinfo: FunctionCallInfo, n: usize) -> PgxrResult<isize> {
    CHECK_PARAM_INDEX!(fcinfo, n);
    let val = unsafe { (&*fcinfo).arg[n] };
    Ok(val as isize)
}

pub fn PG_GETARG_USIZE(fcinfo: FunctionCallInfo, n: usize) -> PgxrResult<usize> {
    CHECK_PARAM_INDEX!(fcinfo, n);
    let val = unsafe { (&*fcinfo).arg[n] };
    Ok(val as usize)
}

pub fn PG_GETARG_I8(fcinfo: FunctionCallInfo, n: usize) -> PgxrResult<i8> {
    CHECK_PARAM_INDEX!(fcinfo, n);
    let val = unsafe { (&*fcinfo).arg[n] };
    Ok(val as i8)
}

pub fn PG_GETARG_I16(fcinfo: FunctionCallInfo, n: usize) -> PgxrResult<i16> {
    CHECK_PARAM_INDEX!(fcinfo, n);
    let val = unsafe { (&*fcinfo).arg[n] };
    Ok(val as i16)
}

pub fn PG_GETARG_I32(fcinfo: FunctionCallInfo, n: usize) -> PgxrResult<i32> {
    CHECK_PARAM_INDEX!(fcinfo, n);
    let val = unsafe { (&*fcinfo).arg[n] };
    Ok(val as i32)
}

pub fn PG_GETARG_I64(fcinfo: FunctionCallInfo, n: usize) -> PgxrResult<i64> {
    CHECK_PARAM_INDEX!(fcinfo, n);
    let val = unsafe { (&*fcinfo).arg[n] };
    Ok(val as i64)
}

pub fn PG_GETARG_U8(fcinfo: FunctionCallInfo, n: usize) -> PgxrResult<u8> {
    CHECK_PARAM_INDEX!(fcinfo, n);
    let val = unsafe { (&*fcinfo).arg[n] };
    Ok(val as u8)
}

pub fn PG_GETARG_U16(fcinfo: FunctionCallInfo, n: usize) -> PgxrResult<u16> {
    CHECK_PARAM_INDEX!(fcinfo, n);
    let val = unsafe { (&*fcinfo).arg[n] };
    Ok(val as u16)
}

pub fn PG_GETARG_U32(fcinfo: FunctionCallInfo, n: usize) -> PgxrResult<u32> {
    CHECK_PARAM_INDEX!(fcinfo, n);
    let val = unsafe { (&*fcinfo).arg[n] };
    Ok(val as u32)
}

pub fn PG_GETARG_U64(fcinfo: FunctionCallInfo, n: usize) -> PgxrResult<u64> {
    CHECK_PARAM_INDEX!(fcinfo, n);
    let val = unsafe { (&*fcinfo).arg[n] };
    Ok(val as u64)
}

pub fn PG_GETARG_STRING(fcinfo: FunctionCallInfo, n: usize) -> PgxrResult<String> {
    CHECK_PARAM_INDEX!(fcinfo, n);
    let c = unsafe { (&*fcinfo).arg[n] as *mut c_char };
    let s = unsafe { CStr::from_ptr(c).to_string_lossy().into_owned() };
    Ok(s)
}

///
/// Return result functions
/// 

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
