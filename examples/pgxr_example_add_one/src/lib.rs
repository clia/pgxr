//#[macro_use]
extern crate pgxr;

use pgxr::bindings::*;
use pgxr::*;

PG_MODULE_MAGIC!();

PG_FUNCTION_INFO_V1!(pg_finfo_pgxr_example_add_one);

#[no_mangle]
pub extern "C" fn pgxr_example_add_one(_fcinfo: FunctionCallInfo) -> Datum
{
	let num = try_return_int!(PG_GETARG_I32(fcinfo, 0));
	PG_RETURN_I32(num + 1)
}
