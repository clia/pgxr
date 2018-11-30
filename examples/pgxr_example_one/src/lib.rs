//#[macro_use]
extern crate pgxr;

use pgxr::bindings::*;
use pgxr::*;

PG_MODULE_MAGIC!();

PG_FUNCTION_INFO_V1!(pg_finfo_pgxr_example_one);

#[no_mangle]
pub extern "C" fn pgxr_example_one(_fcinfo: FunctionCallInfo) -> Datum
{
    PG_RETURN_I32(1)
}
