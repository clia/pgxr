#[macro_use]
extern crate pgxr;

use pgxr::bindings::*;

PG_MODULE_MAGIC!();

PG_FUNCTION_INFO_V1!(pg_finfo_pgxr_example_one);

#[no_mangle]
pub extern "C" fn pgxr_example_one(_fcinfo: FunctionCallInfo) -> Datum
{
    1
}
