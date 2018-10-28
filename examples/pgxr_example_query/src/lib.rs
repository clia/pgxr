extern crate pgxr;

use pgxr::bindings::*;
//use std::os::raw::c_char;
use std::ffi::CString;
use std::slice;

#[no_mangle]
pub extern "C" fn Pg_magic_func() -> *const Pg_magic_struct
{
    &Pg_magic_struct{
        len: 28,
        version: 1000,
        funcmaxargs: 100,
        indexmaxkeys: 32,
        namedatalen: 64,
        float4byval: 1,
        float8byval: 1,
    }
}
// #[no_mangle]
// extern "C" {
//     pub static no_such_variable: c_int;
// }

#[no_mangle]
pub extern "C" fn pg_finfo_pgxr_example_query() -> *const Pg_finfo_record
{
    &Pg_finfo_record{
        api_version: 1
    }
}

#[no_mangle]
pub extern "C" fn pgxr_example_query(fcinfo: FunctionCallInfo) -> Datum
{
    let mut result: Datum = 0;
    unsafe {
        SPI_connect();
        let sql = "select 2";
        let null_terminated = CString::new(sql).unwrap();
        let ret = SPI_execute(null_terminated.as_ptr(), true as bool_, 0);
	    let proc = SPI_processed;
        if ret == SPI_OK_SELECT as i32 && proc > 0 {
            let spi_tuptable = *SPI_tuptable;
            let spi_tupdesc = spi_tuptable.tupdesc;
            let vals = slice::from_raw_parts(spi_tuptable.vals, proc as usize);
            for i in 0..proc {
                let spi_tuple = vals[i as usize];
                result = SPI_getbinval(spi_tuple, spi_tupdesc, 1, &mut (false as bool_));
            }
        }
        SPI_finish();
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
