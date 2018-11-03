#[macro_use]
extern crate pgxr;

use pgxr::bindings::*;
use std::ffi::CString;
use std::slice;

PG_MODULE_MAGIC!();

PG_FUNCTION_INFO_V1!(pg_finfo_pgxr_example_query);

#[no_mangle]
pub extern "C" fn pgxr_example_query(_fcinfo: FunctionCallInfo) -> Datum
{
    let mut result: Datum = 0;
    unsafe { SPI_connect(); }
    let sql = "select 2";
    let null_terminated = CString::new(sql).unwrap();
    let ret = unsafe { SPI_execute(null_terminated.as_ptr(), true as bool_, 0) };
    let proc = unsafe { SPI_processed };
    if ret == SPI_OK_SELECT as i32 && proc > 0 {
        let spi_tuptable = unsafe { *SPI_tuptable };
        let spi_tupdesc = spi_tuptable.tupdesc;
        let vals = unsafe { slice::from_raw_parts(spi_tuptable.vals, proc as usize) };
        for i in 0..proc {
            let spi_tuple = vals[i as usize];
            result = unsafe { SPI_getbinval(spi_tuple, spi_tupdesc, 1, &mut (false as bool_)) };
        }
    }
    unsafe { SPI_finish(); }
    result
}
