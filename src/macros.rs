
#[macro_export]
macro_rules! PG_MODULE_MAGIC {
    () => {
        #[no_mangle]
        pub extern "C" fn Pg_magic_func() -> *const Pg_magic_struct
        {
            &Pg_magic_struct{
                len: 28,
                version: 1100,
                funcmaxargs: 100,
                indexmaxkeys: 32,
                namedatalen: 64,
                float4byval: 1,
                float8byval: 1,
            }
        }
    };
}

#[macro_export]
macro_rules! PG_FUNCTION_INFO_V1 {
    ($funcname:ident) => {
        #[no_mangle]
        pub extern "C" fn $funcname() -> *const Pg_finfo_record
        {
            &Pg_finfo_record{
                api_version: 1
            }
        }
    };
}

///
/// Some macros for PG_GETARG_* error handling
/// 

/// While returning String
#[macro_export]
macro_rules! try_return_string {
    ($expr:expr) => (match $expr {
        std::result::Result::Ok(val) => val,
        std::result::Result::Err(err) => {
            return PG_RETURN_STRING(format!("ERROR: {}", err.description()))
        }
    });
}

/// While returning integer
#[macro_export]
macro_rules! try_return_int {
    ($expr:expr) => (match $expr {
        std::result::Result::Ok(val) => val,
        std::result::Result::Err(err) => {
            return PG_RETURN_I32(-1)
        }
    });
}
