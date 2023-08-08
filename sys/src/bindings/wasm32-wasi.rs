// copied from i686-unknown-linux-gnu

pub const JS_PROP_CONFIGURABLE: u32 = 1;
pub const JS_PROP_WRITABLE: u32 = 2;
pub const JS_PROP_ENUMERABLE: u32 = 4;
pub const JS_PROP_C_W_E: u32 = 7;
pub const JS_PROP_LENGTH: u32 = 8;
pub const JS_PROP_TMASK: u32 = 48;
pub const JS_PROP_NORMAL: u32 = 0;
pub const JS_PROP_GETSET: u32 = 16;
pub const JS_PROP_VARREF: u32 = 32;
pub const JS_PROP_AUTOINIT: u32 = 48;
pub const JS_PROP_HAS_SHIFT: u32 = 8;
pub const JS_PROP_HAS_CONFIGURABLE: u32 = 256;
pub const JS_PROP_HAS_WRITABLE: u32 = 512;
pub const JS_PROP_HAS_ENUMERABLE: u32 = 1024;
pub const JS_PROP_HAS_GET: u32 = 2048;
pub const JS_PROP_HAS_SET: u32 = 4096;
pub const JS_PROP_HAS_VALUE: u32 = 8192;
pub const JS_PROP_THROW: u32 = 16384;
pub const JS_PROP_THROW_STRICT: u32 = 32768;
pub const JS_PROP_NO_ADD: u32 = 65536;
pub const JS_PROP_NO_EXOTIC: u32 = 131072;
pub const JS_DEFAULT_STACK_SIZE: u32 = 262144;
pub const JS_EVAL_TYPE_GLOBAL: u32 = 0;
pub const JS_EVAL_TYPE_MODULE: u32 = 1;
pub const JS_EVAL_TYPE_DIRECT: u32 = 2;
pub const JS_EVAL_TYPE_INDIRECT: u32 = 3;
pub const JS_EVAL_TYPE_MASK: u32 = 3;
pub const JS_EVAL_FLAG_STRICT: u32 = 8;
pub const JS_EVAL_FLAG_STRIP: u32 = 16;
pub const JS_EVAL_FLAG_COMPILE_ONLY: u32 = 32;
pub const JS_EVAL_FLAG_BACKTRACE_BARRIER: u32 = 64;
pub const JS_ATOM_NULL: u32 = 0;
pub const JS_CALL_FLAG_CONSTRUCTOR: u32 = 1;
pub const JS_GPN_STRING_MASK: u32 = 1;
pub const JS_GPN_SYMBOL_MASK: u32 = 2;
pub const JS_GPN_PRIVATE_MASK: u32 = 4;
pub const JS_GPN_ENUM_ONLY: u32 = 16;
pub const JS_GPN_SET_ENUM: u32 = 32;
pub const JS_PARSE_JSON_EXT: u32 = 1;
pub const JS_WRITE_OBJ_BYTECODE: u32 = 1;
pub const JS_WRITE_OBJ_BSWAP: u32 = 2;
pub const JS_WRITE_OBJ_SAB: u32 = 4;
pub const JS_WRITE_OBJ_REFERENCE: u32 = 8;
pub const JS_READ_OBJ_BYTECODE: u32 = 1;
pub const JS_READ_OBJ_ROM_DATA: u32 = 2;
pub const JS_READ_OBJ_SAB: u32 = 4;
pub const JS_READ_OBJ_REFERENCE: u32 = 8;
pub const JS_DEF_CFUNC: u32 = 0;
pub const JS_DEF_CGETSET: u32 = 1;
pub const JS_DEF_CGETSET_MAGIC: u32 = 2;
pub const JS_DEF_PROP_STRING: u32 = 3;
pub const JS_DEF_PROP_INT32: u32 = 4;
pub const JS_DEF_PROP_INT64: u32 = 5;
pub const JS_DEF_PROP_DOUBLE: u32 = 6;
pub const JS_DEF_PROP_UNDEFINED: u32 = 7;
pub const JS_DEF_OBJECT: u32 = 8;
pub const JS_DEF_ALIAS: u32 = 9;
pub type size_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSRuntime {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSContext {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSObject {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSClass {
    _unused: [u8; 0],
}
pub type JSClassID = u32;
pub type JSAtom = u32;
pub const JS_TAG_FIRST: _bindgen_ty_1 = -11;
pub const JS_TAG_BIG_DECIMAL: _bindgen_ty_1 = -11;
pub const JS_TAG_BIG_INT: _bindgen_ty_1 = -10;
pub const JS_TAG_BIG_FLOAT: _bindgen_ty_1 = -9;
pub const JS_TAG_SYMBOL: _bindgen_ty_1 = -8;
pub const JS_TAG_STRING: _bindgen_ty_1 = -7;
pub const JS_TAG_MODULE: _bindgen_ty_1 = -3;
pub const JS_TAG_FUNCTION_BYTECODE: _bindgen_ty_1 = -2;
pub const JS_TAG_OBJECT: _bindgen_ty_1 = -1;
pub const JS_TAG_INT: _bindgen_ty_1 = 0;
pub const JS_TAG_BOOL: _bindgen_ty_1 = 1;
pub const JS_TAG_NULL: _bindgen_ty_1 = 2;
pub const JS_TAG_UNDEFINED: _bindgen_ty_1 = 3;
pub const JS_TAG_UNINITIALIZED: _bindgen_ty_1 = 4;
pub const JS_TAG_CATCH_OFFSET: _bindgen_ty_1 = 5;
pub const JS_TAG_EXCEPTION: _bindgen_ty_1 = 6;
pub const JS_TAG_FLOAT64: _bindgen_ty_1 = 7;
pub type _bindgen_ty_1 = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSRefCountHeader {
    pub ref_count: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_JSRefCountHeader() {
    const UNINIT: ::std::mem::MaybeUninit<JSRefCountHeader> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<JSRefCountHeader>(),
        4usize,
        concat!("Size of: ", stringify!(JSRefCountHeader))
    );
    assert_eq!(
        ::std::mem::align_of::<JSRefCountHeader>(),
        4usize,
        concat!("Alignment of ", stringify!(JSRefCountHeader))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ref_count) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSRefCountHeader),
            "::",
            stringify!(ref_count)
        )
    );
}
pub type JSValue = u64;
pub type JSCFunction = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut JSContext,
        this_val: JSValue,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
    ) -> JSValue,
>;
pub type JSCFunctionMagic = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut JSContext,
        this_val: JSValue,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
        magic: ::std::os::raw::c_int,
    ) -> JSValue,
>;
pub type JSCFunctionData = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut JSContext,
        this_val: JSValue,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
        magic: ::std::os::raw::c_int,
        func_data: *mut JSValue,
    ) -> JSValue,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSMallocState {
    pub malloc_count: size_t,
    pub malloc_size: size_t,
    pub malloc_limit: size_t,
    pub opaque: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_JSMallocState() {
    const UNINIT: ::std::mem::MaybeUninit<JSMallocState> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<JSMallocState>(),
        16usize,
        concat!("Size of: ", stringify!(JSMallocState))
    );
    assert_eq!(
        ::std::mem::align_of::<JSMallocState>(),
        4usize,
        concat!("Alignment of ", stringify!(JSMallocState))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).malloc_count) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMallocState),
            "::",
            stringify!(malloc_count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).malloc_size) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMallocState),
            "::",
            stringify!(malloc_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).malloc_limit) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMallocState),
            "::",
            stringify!(malloc_limit)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).opaque) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMallocState),
            "::",
            stringify!(opaque)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSMallocFunctions {
    pub js_malloc: ::std::option::Option<
        unsafe extern "C" fn(s: *mut JSMallocState, size: size_t) -> *mut ::std::os::raw::c_void,
    >,
    pub js_free: ::std::option::Option<
        unsafe extern "C" fn(s: *mut JSMallocState, ptr: *mut ::std::os::raw::c_void),
    >,
    pub js_realloc: ::std::option::Option<
        unsafe extern "C" fn(
            s: *mut JSMallocState,
            ptr: *mut ::std::os::raw::c_void,
            size: size_t,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub js_malloc_usable_size:
        ::std::option::Option<unsafe extern "C" fn(ptr: *const ::std::os::raw::c_void) -> size_t>,
}
#[test]
fn bindgen_test_layout_JSMallocFunctions() {
    const UNINIT: ::std::mem::MaybeUninit<JSMallocFunctions> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<JSMallocFunctions>(),
        16usize,
        concat!("Size of: ", stringify!(JSMallocFunctions))
    );
    assert_eq!(
        ::std::mem::align_of::<JSMallocFunctions>(),
        4usize,
        concat!("Alignment of ", stringify!(JSMallocFunctions))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).js_malloc) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMallocFunctions),
            "::",
            stringify!(js_malloc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).js_free) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMallocFunctions),
            "::",
            stringify!(js_free)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).js_realloc) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMallocFunctions),
            "::",
            stringify!(js_realloc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).js_malloc_usable_size) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMallocFunctions),
            "::",
            stringify!(js_malloc_usable_size)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSGCObjectHeader {
    _unused: [u8; 0],
}
extern "C" {
    pub fn JS_NewRuntime() -> *mut JSRuntime;
}
extern "C" {
    pub fn JS_SetRuntimeInfo(rt: *mut JSRuntime, info: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn JS_SetMemoryLimit(rt: *mut JSRuntime, limit: size_t);
}
extern "C" {
    pub fn JS_SetGCThreshold(rt: *mut JSRuntime, gc_threshold: size_t);
}
extern "C" {
    pub fn JS_SetMaxStackSize(rt: *mut JSRuntime, stack_size: size_t);
}
extern "C" {
    pub fn JS_UpdateStackTop(rt: *mut JSRuntime);
}
extern "C" {
    pub fn JS_NewRuntime2(
        mf: *const JSMallocFunctions,
        opaque: *mut ::std::os::raw::c_void,
    ) -> *mut JSRuntime;
}
extern "C" {
    pub fn JS_FreeRuntime(rt: *mut JSRuntime);
}
extern "C" {
    pub fn JS_GetRuntimeOpaque(rt: *mut JSRuntime) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn JS_SetRuntimeOpaque(rt: *mut JSRuntime, opaque: *mut ::std::os::raw::c_void);
}
pub type JS_MarkFunc =
    ::std::option::Option<unsafe extern "C" fn(rt: *mut JSRuntime, gp: *mut JSGCObjectHeader)>;
extern "C" {
    pub fn JS_MarkValue(rt: *mut JSRuntime, val: JSValue, mark_func: JS_MarkFunc);
}
extern "C" {
    pub fn JS_RunGC(rt: *mut JSRuntime);
}
extern "C" {
    pub fn JS_IsLiveObject(rt: *mut JSRuntime, obj: JSValue) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_NewContext(rt: *mut JSRuntime) -> *mut JSContext;
}
extern "C" {
    pub fn JS_FreeContext(s: *mut JSContext);
}
extern "C" {
    pub fn JS_DupContext(ctx: *mut JSContext) -> *mut JSContext;
}
extern "C" {
    pub fn JS_GetContextOpaque(ctx: *mut JSContext) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn JS_SetContextOpaque(ctx: *mut JSContext, opaque: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn JS_GetRuntime(ctx: *mut JSContext) -> *mut JSRuntime;
}
extern "C" {
    pub fn JS_SetClassProto(ctx: *mut JSContext, class_id: JSClassID, obj: JSValue);
}
extern "C" {
    pub fn JS_GetClassProto(ctx: *mut JSContext, class_id: JSClassID) -> JSValue;
}
extern "C" {
    pub fn JS_NewContextRaw(rt: *mut JSRuntime) -> *mut JSContext;
}
extern "C" {
    pub fn JS_AddIntrinsicBaseObjects(ctx: *mut JSContext);
}
extern "C" {
    pub fn JS_AddIntrinsicDate(ctx: *mut JSContext);
}
extern "C" {
    pub fn JS_AddIntrinsicEval(ctx: *mut JSContext);
}
extern "C" {
    pub fn JS_AddIntrinsicStringNormalize(ctx: *mut JSContext);
}
extern "C" {
    pub fn JS_AddIntrinsicRegExpCompiler(ctx: *mut JSContext);
}
extern "C" {
    pub fn JS_AddIntrinsicRegExp(ctx: *mut JSContext);
}
extern "C" {
    pub fn JS_AddIntrinsicJSON(ctx: *mut JSContext);
}
extern "C" {
    pub fn JS_AddIntrinsicProxy(ctx: *mut JSContext);
}
extern "C" {
    pub fn JS_AddIntrinsicMapSet(ctx: *mut JSContext);
}
extern "C" {
    pub fn JS_AddIntrinsicTypedArrays(ctx: *mut JSContext);
}
extern "C" {
    pub fn JS_AddIntrinsicPromise(ctx: *mut JSContext);
}
extern "C" {
    pub fn JS_AddIntrinsicBigInt(ctx: *mut JSContext);
}
extern "C" {
    pub fn JS_AddIntrinsicBigFloat(ctx: *mut JSContext);
}
extern "C" {
    pub fn JS_AddIntrinsicBigDecimal(ctx: *mut JSContext);
}
extern "C" {
    pub fn JS_AddIntrinsicOperators(ctx: *mut JSContext);
}
extern "C" {
    pub fn JS_EnableBignumExt(ctx: *mut JSContext, enable: ::std::os::raw::c_int);
}
extern "C" {
    pub fn js_string_codePointRange(
        ctx: *mut JSContext,
        this_val: JSValue,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
    ) -> JSValue;
}
extern "C" {
    pub fn js_malloc_rt(rt: *mut JSRuntime, size: size_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn js_free_rt(rt: *mut JSRuntime, ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn js_realloc_rt(
        rt: *mut JSRuntime,
        ptr: *mut ::std::os::raw::c_void,
        size: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn js_malloc_usable_size_rt(
        rt: *mut JSRuntime,
        ptr: *const ::std::os::raw::c_void,
    ) -> size_t;
}
extern "C" {
    pub fn js_mallocz_rt(rt: *mut JSRuntime, size: size_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn js_malloc(ctx: *mut JSContext, size: size_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn js_free(ctx: *mut JSContext, ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn js_realloc(
        ctx: *mut JSContext,
        ptr: *mut ::std::os::raw::c_void,
        size: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn js_malloc_usable_size(ctx: *mut JSContext, ptr: *const ::std::os::raw::c_void)
        -> size_t;
}
extern "C" {
    pub fn js_realloc2(
        ctx: *mut JSContext,
        ptr: *mut ::std::os::raw::c_void,
        size: size_t,
        pslack: *mut size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn js_mallocz(ctx: *mut JSContext, size: size_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn js_strdup(
        ctx: *mut JSContext,
        str_: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn js_strndup(
        ctx: *mut JSContext,
        s: *const ::std::os::raw::c_char,
        n: size_t,
    ) -> *mut ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSMemoryUsage {
    pub malloc_size: i64,
    pub malloc_limit: i64,
    pub memory_used_size: i64,
    pub malloc_count: i64,
    pub memory_used_count: i64,
    pub atom_count: i64,
    pub atom_size: i64,
    pub str_count: i64,
    pub str_size: i64,
    pub obj_count: i64,
    pub obj_size: i64,
    pub prop_count: i64,
    pub prop_size: i64,
    pub shape_count: i64,
    pub shape_size: i64,
    pub js_func_count: i64,
    pub js_func_size: i64,
    pub js_func_code_size: i64,
    pub js_func_pc2line_count: i64,
    pub js_func_pc2line_size: i64,
    pub c_func_count: i64,
    pub array_count: i64,
    pub fast_array_count: i64,
    pub fast_array_elements: i64,
    pub binary_object_count: i64,
    pub binary_object_size: i64,
}
#[test]
fn bindgen_test_layout_JSMemoryUsage() {
    const UNINIT: ::std::mem::MaybeUninit<JSMemoryUsage> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<JSMemoryUsage>(),
        208usize,
        concat!("Size of: ", stringify!(JSMemoryUsage))
    );
    assert_eq!(
        ::std::mem::align_of::<JSMemoryUsage>(),
        4usize,
        concat!("Alignment of ", stringify!(JSMemoryUsage))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).malloc_size) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(malloc_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).malloc_limit) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(malloc_limit)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).memory_used_size) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(memory_used_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).malloc_count) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(malloc_count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).memory_used_count) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(memory_used_count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).atom_count) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(atom_count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).atom_size) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(atom_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).str_count) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(str_count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).str_size) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(str_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).obj_count) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(obj_count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).obj_size) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(obj_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).prop_count) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(prop_count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).prop_size) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(prop_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).shape_count) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(shape_count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).shape_size) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(shape_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).js_func_count) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(js_func_count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).js_func_size) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(js_func_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).js_func_code_size) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(js_func_code_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).js_func_pc2line_count) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(js_func_pc2line_count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).js_func_pc2line_size) as usize - ptr as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(js_func_pc2line_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).c_func_count) as usize - ptr as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(c_func_count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).array_count) as usize - ptr as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(array_count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fast_array_count) as usize - ptr as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(fast_array_count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fast_array_elements) as usize - ptr as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(fast_array_elements)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).binary_object_count) as usize - ptr as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(binary_object_count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).binary_object_size) as usize - ptr as usize },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(JSMemoryUsage),
            "::",
            stringify!(binary_object_size)
        )
    );
}
extern "C" {
    pub fn JS_ComputeMemoryUsage(rt: *mut JSRuntime, s: *mut JSMemoryUsage);
}
extern "C" {
    pub fn JS_NewAtomLen(
        ctx: *mut JSContext,
        str_: *const ::std::os::raw::c_char,
        len: size_t,
    ) -> JSAtom;
}
extern "C" {
    pub fn JS_NewAtom(ctx: *mut JSContext, str_: *const ::std::os::raw::c_char) -> JSAtom;
}
extern "C" {
    pub fn JS_NewAtomUInt32(ctx: *mut JSContext, n: u32) -> JSAtom;
}
extern "C" {
    pub fn JS_DupAtom(ctx: *mut JSContext, v: JSAtom) -> JSAtom;
}
extern "C" {
    pub fn JS_FreeAtom(ctx: *mut JSContext, v: JSAtom);
}
extern "C" {
    pub fn JS_FreeAtomRT(rt: *mut JSRuntime, v: JSAtom);
}
extern "C" {
    pub fn JS_AtomToValue(ctx: *mut JSContext, atom: JSAtom) -> JSValue;
}
extern "C" {
    pub fn JS_AtomToString(ctx: *mut JSContext, atom: JSAtom) -> JSValue;
}
extern "C" {
    pub fn JS_AtomToCString(ctx: *mut JSContext, atom: JSAtom) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn JS_ValueToAtom(ctx: *mut JSContext, val: JSValue) -> JSAtom;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSPropertyEnum {
    pub is_enumerable: ::std::os::raw::c_int,
    pub atom: JSAtom,
}
#[test]
fn bindgen_test_layout_JSPropertyEnum() {
    const UNINIT: ::std::mem::MaybeUninit<JSPropertyEnum> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<JSPropertyEnum>(),
        8usize,
        concat!("Size of: ", stringify!(JSPropertyEnum))
    );
    assert_eq!(
        ::std::mem::align_of::<JSPropertyEnum>(),
        4usize,
        concat!("Alignment of ", stringify!(JSPropertyEnum))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).is_enumerable) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSPropertyEnum),
            "::",
            stringify!(is_enumerable)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).atom) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(JSPropertyEnum),
            "::",
            stringify!(atom)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSPropertyDescriptor {
    pub flags: ::std::os::raw::c_int,
    pub value: JSValue,
    pub getter: JSValue,
    pub setter: JSValue,
}
#[test]
fn bindgen_test_layout_JSPropertyDescriptor() {
    const UNINIT: ::std::mem::MaybeUninit<JSPropertyDescriptor> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<JSPropertyDescriptor>(),
        28usize,
        concat!("Size of: ", stringify!(JSPropertyDescriptor))
    );
    assert_eq!(
        ::std::mem::align_of::<JSPropertyDescriptor>(),
        4usize,
        concat!("Alignment of ", stringify!(JSPropertyDescriptor))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).flags) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSPropertyDescriptor),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(JSPropertyDescriptor),
            "::",
            stringify!(value)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).getter) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(JSPropertyDescriptor),
            "::",
            stringify!(getter)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).setter) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(JSPropertyDescriptor),
            "::",
            stringify!(setter)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSClassExoticMethods {
    pub get_own_property: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut JSContext,
            desc: *mut JSPropertyDescriptor,
            obj: JSValue,
            prop: JSAtom,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_own_property_names: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut JSContext,
            ptab: *mut *mut JSPropertyEnum,
            plen: *mut u32,
            obj: JSValue,
        ) -> ::std::os::raw::c_int,
    >,
    pub delete_property: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut JSContext,
            obj: JSValue,
            prop: JSAtom,
        ) -> ::std::os::raw::c_int,
    >,
    pub define_own_property: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut JSContext,
            this_obj: JSValue,
            prop: JSAtom,
            val: JSValue,
            getter: JSValue,
            setter: JSValue,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub has_property: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut JSContext,
            obj: JSValue,
            atom: JSAtom,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_property: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut JSContext,
            obj: JSValue,
            atom: JSAtom,
            receiver: JSValue,
        ) -> JSValue,
    >,
    pub set_property: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut JSContext,
            obj: JSValue,
            atom: JSAtom,
            value: JSValue,
            receiver: JSValue,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
}
#[test]
fn bindgen_test_layout_JSClassExoticMethods() {
    const UNINIT: ::std::mem::MaybeUninit<JSClassExoticMethods> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<JSClassExoticMethods>(),
        28usize,
        concat!("Size of: ", stringify!(JSClassExoticMethods))
    );
    assert_eq!(
        ::std::mem::align_of::<JSClassExoticMethods>(),
        4usize,
        concat!("Alignment of ", stringify!(JSClassExoticMethods))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).get_own_property) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSClassExoticMethods),
            "::",
            stringify!(get_own_property)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).get_own_property_names) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(JSClassExoticMethods),
            "::",
            stringify!(get_own_property_names)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).delete_property) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(JSClassExoticMethods),
            "::",
            stringify!(delete_property)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).define_own_property) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(JSClassExoticMethods),
            "::",
            stringify!(define_own_property)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).has_property) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(JSClassExoticMethods),
            "::",
            stringify!(has_property)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).get_property) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(JSClassExoticMethods),
            "::",
            stringify!(get_property)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).set_property) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(JSClassExoticMethods),
            "::",
            stringify!(set_property)
        )
    );
}
pub type JSClassFinalizer =
    ::std::option::Option<unsafe extern "C" fn(rt: *mut JSRuntime, val: JSValue)>;
pub type JSClassGCMark = ::std::option::Option<
    unsafe extern "C" fn(rt: *mut JSRuntime, val: JSValue, mark_func: JS_MarkFunc),
>;
pub type JSClassCall = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut JSContext,
        func_obj: JSValue,
        this_val: JSValue,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
        flags: ::std::os::raw::c_int,
    ) -> JSValue,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSClassDef {
    pub class_name: *const ::std::os::raw::c_char,
    pub finalizer: JSClassFinalizer,
    pub gc_mark: JSClassGCMark,
    pub call: JSClassCall,
    pub exotic: *mut JSClassExoticMethods,
}
#[test]
fn bindgen_test_layout_JSClassDef() {
    const UNINIT: ::std::mem::MaybeUninit<JSClassDef> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<JSClassDef>(),
        20usize,
        concat!("Size of: ", stringify!(JSClassDef))
    );
    assert_eq!(
        ::std::mem::align_of::<JSClassDef>(),
        4usize,
        concat!("Alignment of ", stringify!(JSClassDef))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).class_name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSClassDef),
            "::",
            stringify!(class_name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).finalizer) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(JSClassDef),
            "::",
            stringify!(finalizer)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gc_mark) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(JSClassDef),
            "::",
            stringify!(gc_mark)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).call) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(JSClassDef),
            "::",
            stringify!(call)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).exotic) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(JSClassDef),
            "::",
            stringify!(exotic)
        )
    );
}
extern "C" {
    pub fn JS_NewClassID(pclass_id: *mut JSClassID) -> JSClassID;
}
extern "C" {
    pub fn JS_NewClass(
        rt: *mut JSRuntime,
        class_id: JSClassID,
        class_def: *const JSClassDef,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_IsRegisteredClass(rt: *mut JSRuntime, class_id: JSClassID) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_NewBigInt64(ctx: *mut JSContext, v: i64) -> JSValue;
}
extern "C" {
    pub fn JS_NewBigUint64(ctx: *mut JSContext, v: u64) -> JSValue;
}
extern "C" {
    pub fn JS_Throw(ctx: *mut JSContext, obj: JSValue) -> JSValue;
}
extern "C" {
    pub fn JS_GetException(ctx: *mut JSContext) -> JSValue;
}
extern "C" {
    pub fn JS_IsError(ctx: *mut JSContext, val: JSValue) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_ResetUncatchableError(ctx: *mut JSContext);
}
extern "C" {
    pub fn JS_NewError(ctx: *mut JSContext) -> JSValue;
}
extern "C" {
    pub fn JS_ThrowSyntaxError(
        ctx: *mut JSContext,
        fmt: *const ::std::os::raw::c_char,
        ...
    ) -> JSValue;
}
extern "C" {
    pub fn JS_ThrowTypeError(
        ctx: *mut JSContext,
        fmt: *const ::std::os::raw::c_char,
        ...
    ) -> JSValue;
}
extern "C" {
    pub fn JS_ThrowReferenceError(
        ctx: *mut JSContext,
        fmt: *const ::std::os::raw::c_char,
        ...
    ) -> JSValue;
}
extern "C" {
    pub fn JS_ThrowRangeError(
        ctx: *mut JSContext,
        fmt: *const ::std::os::raw::c_char,
        ...
    ) -> JSValue;
}
extern "C" {
    pub fn JS_ThrowInternalError(
        ctx: *mut JSContext,
        fmt: *const ::std::os::raw::c_char,
        ...
    ) -> JSValue;
}
extern "C" {
    pub fn JS_ThrowOutOfMemory(ctx: *mut JSContext) -> JSValue;
}
extern "C" {
    pub fn __JS_FreeValue(ctx: *mut JSContext, v: JSValue);
}
extern "C" {
    pub fn __JS_FreeValueRT(rt: *mut JSRuntime, v: JSValue);
}
extern "C" {
    pub fn JS_ToBool(ctx: *mut JSContext, val: JSValue) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_ToInt32(ctx: *mut JSContext, pres: *mut i32, val: JSValue) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_ToInt64(ctx: *mut JSContext, pres: *mut i64, val: JSValue) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_ToIndex(ctx: *mut JSContext, plen: *mut u64, val: JSValue) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_ToFloat64(ctx: *mut JSContext, pres: *mut f64, val: JSValue)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_ToBigInt64(
        ctx: *mut JSContext,
        pres: *mut i64,
        val: JSValue,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_ToInt64Ext(
        ctx: *mut JSContext,
        pres: *mut i64,
        val: JSValue,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_NewStringLen(
        ctx: *mut JSContext,
        str1: *const ::std::os::raw::c_char,
        len1: size_t,
    ) -> JSValue;
}
extern "C" {
    pub fn JS_NewString(ctx: *mut JSContext, str_: *const ::std::os::raw::c_char) -> JSValue;
}
extern "C" {
    pub fn JS_NewAtomString(ctx: *mut JSContext, str_: *const ::std::os::raw::c_char) -> JSValue;
}
extern "C" {
    pub fn JS_ToString(ctx: *mut JSContext, val: JSValue) -> JSValue;
}
extern "C" {
    pub fn JS_ToPropertyKey(ctx: *mut JSContext, val: JSValue) -> JSValue;
}
extern "C" {
    pub fn JS_ToCStringLen2(
        ctx: *mut JSContext,
        plen: *mut size_t,
        val1: JSValue,
        cesu8: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn JS_FreeCString(ctx: *mut JSContext, ptr: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn JS_NewObjectProtoClass(
        ctx: *mut JSContext,
        proto: JSValue,
        class_id: JSClassID,
    ) -> JSValue;
}
extern "C" {
    pub fn JS_NewObjectClass(ctx: *mut JSContext, class_id: ::std::os::raw::c_int) -> JSValue;
}
extern "C" {
    pub fn JS_NewObjectProto(ctx: *mut JSContext, proto: JSValue) -> JSValue;
}
extern "C" {
    pub fn JS_NewObject(ctx: *mut JSContext) -> JSValue;
}
extern "C" {
    pub fn JS_IsFunction(ctx: *mut JSContext, val: JSValue) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_IsConstructor(ctx: *mut JSContext, val: JSValue) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_SetConstructorBit(
        ctx: *mut JSContext,
        func_obj: JSValue,
        val: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_NewArray(ctx: *mut JSContext) -> JSValue;
}
extern "C" {
    pub fn JS_IsArray(ctx: *mut JSContext, val: JSValue) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_GetPropertyInternal(
        ctx: *mut JSContext,
        obj: JSValue,
        prop: JSAtom,
        receiver: JSValue,
        throw_ref_error: ::std::os::raw::c_int,
    ) -> JSValue;
}
extern "C" {
    pub fn JS_GetPropertyStr(
        ctx: *mut JSContext,
        this_obj: JSValue,
        prop: *const ::std::os::raw::c_char,
    ) -> JSValue;
}
extern "C" {
    pub fn JS_GetPropertyUint32(ctx: *mut JSContext, this_obj: JSValue, idx: u32) -> JSValue;
}
extern "C" {
    pub fn JS_SetPropertyInternal(
        ctx: *mut JSContext,
        this_obj: JSValue,
        prop: JSAtom,
        val: JSValue,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_SetPropertyUint32(
        ctx: *mut JSContext,
        this_obj: JSValue,
        idx: u32,
        val: JSValue,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_SetPropertyInt64(
        ctx: *mut JSContext,
        this_obj: JSValue,
        idx: i64,
        val: JSValue,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_SetPropertyStr(
        ctx: *mut JSContext,
        this_obj: JSValue,
        prop: *const ::std::os::raw::c_char,
        val: JSValue,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_HasProperty(
        ctx: *mut JSContext,
        this_obj: JSValue,
        prop: JSAtom,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_IsExtensible(ctx: *mut JSContext, obj: JSValue) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_PreventExtensions(ctx: *mut JSContext, obj: JSValue) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_DeleteProperty(
        ctx: *mut JSContext,
        obj: JSValue,
        prop: JSAtom,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_SetPrototype(
        ctx: *mut JSContext,
        obj: JSValue,
        proto_val: JSValue,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_GetPrototype(ctx: *mut JSContext, val: JSValue) -> JSValue;
}
extern "C" {
    pub fn JS_GetOwnPropertyNames(
        ctx: *mut JSContext,
        ptab: *mut *mut JSPropertyEnum,
        plen: *mut u32,
        obj: JSValue,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_GetOwnProperty(
        ctx: *mut JSContext,
        desc: *mut JSPropertyDescriptor,
        obj: JSValue,
        prop: JSAtom,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_Call(
        ctx: *mut JSContext,
        func_obj: JSValue,
        this_obj: JSValue,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
    ) -> JSValue;
}
extern "C" {
    pub fn JS_Invoke(
        ctx: *mut JSContext,
        this_val: JSValue,
        atom: JSAtom,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
    ) -> JSValue;
}
extern "C" {
    pub fn JS_CallConstructor(
        ctx: *mut JSContext,
        func_obj: JSValue,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
    ) -> JSValue;
}
extern "C" {
    pub fn JS_CallConstructor2(
        ctx: *mut JSContext,
        func_obj: JSValue,
        new_target: JSValue,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
    ) -> JSValue;
}
extern "C" {
    pub fn JS_DetectModule(
        input: *const ::std::os::raw::c_char,
        input_len: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_Eval(
        ctx: *mut JSContext,
        input: *const ::std::os::raw::c_char,
        input_len: size_t,
        filename: *const ::std::os::raw::c_char,
        eval_flags: ::std::os::raw::c_int,
    ) -> JSValue;
}
extern "C" {
    pub fn JS_EvalThis(
        ctx: *mut JSContext,
        this_obj: JSValue,
        input: *const ::std::os::raw::c_char,
        input_len: size_t,
        filename: *const ::std::os::raw::c_char,
        eval_flags: ::std::os::raw::c_int,
    ) -> JSValue;
}
extern "C" {
    pub fn JS_GetGlobalObject(ctx: *mut JSContext) -> JSValue;
}
extern "C" {
    pub fn JS_GetFunctionProto(ctx: *mut JSContext) -> JSValue;
}
extern "C" {
    pub fn JS_IsInstanceOf(
        ctx: *mut JSContext,
        val: JSValue,
        obj: JSValue,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_DefineProperty(
        ctx: *mut JSContext,
        this_obj: JSValue,
        prop: JSAtom,
        val: JSValue,
        getter: JSValue,
        setter: JSValue,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_DefinePropertyValue(
        ctx: *mut JSContext,
        this_obj: JSValue,
        prop: JSAtom,
        val: JSValue,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_DefinePropertyValueUint32(
        ctx: *mut JSContext,
        this_obj: JSValue,
        idx: u32,
        val: JSValue,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_DefinePropertyValueStr(
        ctx: *mut JSContext,
        this_obj: JSValue,
        prop: *const ::std::os::raw::c_char,
        val: JSValue,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_DefinePropertyGetSet(
        ctx: *mut JSContext,
        this_obj: JSValue,
        prop: JSAtom,
        getter: JSValue,
        setter: JSValue,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_SetOpaque(obj: JSValue, opaque: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn JS_GetOpaque(obj: JSValue, class_id: JSClassID) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn JS_GetOpaque2(
        ctx: *mut JSContext,
        obj: JSValue,
        class_id: JSClassID,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn JS_ParseJSON(
        ctx: *mut JSContext,
        buf: *const ::std::os::raw::c_char,
        buf_len: size_t,
        filename: *const ::std::os::raw::c_char,
    ) -> JSValue;
}
extern "C" {
    pub fn JS_ParseJSON2(
        ctx: *mut JSContext,
        buf: *const ::std::os::raw::c_char,
        buf_len: size_t,
        filename: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ) -> JSValue;
}
extern "C" {
    pub fn JS_JSONStringify(
        ctx: *mut JSContext,
        obj: JSValue,
        replacer: JSValue,
        space0: JSValue,
    ) -> JSValue;
}
pub type JSFreeArrayBufferDataFunc = ::std::option::Option<
    unsafe extern "C" fn(
        rt: *mut JSRuntime,
        opaque: *mut ::std::os::raw::c_void,
        ptr: *mut ::std::os::raw::c_void,
    ),
>;
extern "C" {
    pub fn JS_NewArrayBuffer(
        ctx: *mut JSContext,
        buf: *mut u8,
        len: size_t,
        free_func: JSFreeArrayBufferDataFunc,
        opaque: *mut ::std::os::raw::c_void,
        is_shared: ::std::os::raw::c_int,
    ) -> JSValue;
}
extern "C" {
    pub fn JS_NewArrayBufferCopy(ctx: *mut JSContext, buf: *const u8, len: size_t) -> JSValue;
}
extern "C" {
    pub fn JS_DetachArrayBuffer(ctx: *mut JSContext, obj: JSValue);
}
extern "C" {
    pub fn JS_GetArrayBuffer(ctx: *mut JSContext, psize: *mut size_t, obj: JSValue) -> *mut u8;
}
extern "C" {
    pub fn JS_GetTypedArrayBuffer(
        ctx: *mut JSContext,
        obj: JSValue,
        pbyte_offset: *mut size_t,
        pbyte_length: *mut size_t,
        pbytes_per_element: *mut size_t,
    ) -> JSValue;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSSharedArrayBufferFunctions {
    pub sab_alloc: ::std::option::Option<
        unsafe extern "C" fn(
            opaque: *mut ::std::os::raw::c_void,
            size: size_t,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub sab_free: ::std::option::Option<
        unsafe extern "C" fn(opaque: *mut ::std::os::raw::c_void, ptr: *mut ::std::os::raw::c_void),
    >,
    pub sab_dup: ::std::option::Option<
        unsafe extern "C" fn(opaque: *mut ::std::os::raw::c_void, ptr: *mut ::std::os::raw::c_void),
    >,
    pub sab_opaque: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_JSSharedArrayBufferFunctions() {
    const UNINIT: ::std::mem::MaybeUninit<JSSharedArrayBufferFunctions> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<JSSharedArrayBufferFunctions>(),
        16usize,
        concat!("Size of: ", stringify!(JSSharedArrayBufferFunctions))
    );
    assert_eq!(
        ::std::mem::align_of::<JSSharedArrayBufferFunctions>(),
        4usize,
        concat!("Alignment of ", stringify!(JSSharedArrayBufferFunctions))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sab_alloc) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSSharedArrayBufferFunctions),
            "::",
            stringify!(sab_alloc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sab_free) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(JSSharedArrayBufferFunctions),
            "::",
            stringify!(sab_free)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sab_dup) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(JSSharedArrayBufferFunctions),
            "::",
            stringify!(sab_dup)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sab_opaque) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(JSSharedArrayBufferFunctions),
            "::",
            stringify!(sab_opaque)
        )
    );
}
extern "C" {
    pub fn JS_SetSharedArrayBufferFunctions(
        rt: *mut JSRuntime,
        sf: *const JSSharedArrayBufferFunctions,
    );
}
extern "C" {
    pub fn JS_NewPromiseCapability(ctx: *mut JSContext, resolving_funcs: *mut JSValue) -> JSValue;
}
pub type JSHostPromiseRejectionTracker = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut JSContext,
        promise: JSValue,
        reason: JSValue,
        is_handled: ::std::os::raw::c_int,
        opaque: *mut ::std::os::raw::c_void,
    ),
>;
extern "C" {
    pub fn JS_SetHostPromiseRejectionTracker(
        rt: *mut JSRuntime,
        cb: JSHostPromiseRejectionTracker,
        opaque: *mut ::std::os::raw::c_void,
    );
}
pub type JSInterruptHandler = ::std::option::Option<
    unsafe extern "C" fn(
        rt: *mut JSRuntime,
        opaque: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn JS_SetInterruptHandler(
        rt: *mut JSRuntime,
        cb: JSInterruptHandler,
        opaque: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn JS_SetCanBlock(rt: *mut JSRuntime, can_block: ::std::os::raw::c_int);
}
extern "C" {
    pub fn JS_SetIsHTMLDDA(ctx: *mut JSContext, obj: JSValue);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSModuleDef {
    _unused: [u8; 0],
}
pub type JSModuleNormalizeFunc = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut JSContext,
        module_base_name: *const ::std::os::raw::c_char,
        module_name: *const ::std::os::raw::c_char,
        opaque: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_char,
>;
pub type JSModuleLoaderFunc = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut JSContext,
        module_name: *const ::std::os::raw::c_char,
        opaque: *mut ::std::os::raw::c_void,
    ) -> *mut JSModuleDef,
>;
extern "C" {
    pub fn JS_SetModuleLoaderFunc(
        rt: *mut JSRuntime,
        module_normalize: JSModuleNormalizeFunc,
        module_loader: JSModuleLoaderFunc,
        opaque: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn JS_GetImportMeta(ctx: *mut JSContext, m: *mut JSModuleDef) -> JSValue;
}
extern "C" {
    pub fn JS_GetModuleName(ctx: *mut JSContext, m: *mut JSModuleDef) -> JSAtom;
}
pub type JSJobFunc = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut JSContext,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
    ) -> JSValue,
>;
extern "C" {
    pub fn JS_EnqueueJob(
        ctx: *mut JSContext,
        job_func: JSJobFunc,
        argc: ::std::os::raw::c_int,
        argv: *mut JSValue,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_IsJobPending(rt: *mut JSRuntime) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_ExecutePendingJob(
        rt: *mut JSRuntime,
        pctx: *mut *mut JSContext,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_WriteObject(
        ctx: *mut JSContext,
        psize: *mut size_t,
        obj: JSValue,
        flags: ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn JS_WriteObject2(
        ctx: *mut JSContext,
        psize: *mut size_t,
        obj: JSValue,
        flags: ::std::os::raw::c_int,
        psab_tab: *mut *mut *mut u8,
        psab_tab_len: *mut size_t,
    ) -> *mut u8;
}
extern "C" {
    pub fn JS_ReadObject(
        ctx: *mut JSContext,
        buf: *const u8,
        buf_len: size_t,
        flags: ::std::os::raw::c_int,
    ) -> JSValue;
}
extern "C" {
    pub fn JS_EvalFunction(ctx: *mut JSContext, fun_obj: JSValue) -> JSValue;
}
extern "C" {
    pub fn JS_ResolveModule(ctx: *mut JSContext, obj: JSValue) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_GetScriptOrModuleName(
        ctx: *mut JSContext,
        n_stack_levels: ::std::os::raw::c_int,
    ) -> JSAtom;
}
extern "C" {
    pub fn JS_RunModule(
        ctx: *mut JSContext,
        basename: *const ::std::os::raw::c_char,
        filename: *const ::std::os::raw::c_char,
    ) -> *mut JSModuleDef;
}
pub const JSCFunctionEnum_JS_CFUNC_generic: JSCFunctionEnum = 0;
pub const JSCFunctionEnum_JS_CFUNC_generic_magic: JSCFunctionEnum = 1;
pub const JSCFunctionEnum_JS_CFUNC_constructor: JSCFunctionEnum = 2;
pub const JSCFunctionEnum_JS_CFUNC_constructor_magic: JSCFunctionEnum = 3;
pub const JSCFunctionEnum_JS_CFUNC_constructor_or_func: JSCFunctionEnum = 4;
pub const JSCFunctionEnum_JS_CFUNC_constructor_or_func_magic: JSCFunctionEnum = 5;
pub const JSCFunctionEnum_JS_CFUNC_f_f: JSCFunctionEnum = 6;
pub const JSCFunctionEnum_JS_CFUNC_f_f_f: JSCFunctionEnum = 7;
pub const JSCFunctionEnum_JS_CFUNC_getter: JSCFunctionEnum = 8;
pub const JSCFunctionEnum_JS_CFUNC_setter: JSCFunctionEnum = 9;
pub const JSCFunctionEnum_JS_CFUNC_getter_magic: JSCFunctionEnum = 10;
pub const JSCFunctionEnum_JS_CFUNC_setter_magic: JSCFunctionEnum = 11;
pub const JSCFunctionEnum_JS_CFUNC_iterator_next: JSCFunctionEnum = 12;
pub type JSCFunctionEnum = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub union JSCFunctionType {
    pub generic: JSCFunction,
    pub generic_magic: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut JSContext,
            this_val: JSValue,
            argc: ::std::os::raw::c_int,
            argv: *mut JSValue,
            magic: ::std::os::raw::c_int,
        ) -> JSValue,
    >,
    pub constructor: JSCFunction,
    pub constructor_magic: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut JSContext,
            new_target: JSValue,
            argc: ::std::os::raw::c_int,
            argv: *mut JSValue,
            magic: ::std::os::raw::c_int,
        ) -> JSValue,
    >,
    pub constructor_or_func: JSCFunction,
    pub f_f: ::std::option::Option<unsafe extern "C" fn(arg1: f64) -> f64>,
    pub f_f_f: ::std::option::Option<unsafe extern "C" fn(arg1: f64, arg2: f64) -> f64>,
    pub getter: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut JSContext, this_val: JSValue) -> JSValue,
    >,
    pub setter: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut JSContext, this_val: JSValue, val: JSValue) -> JSValue,
    >,
    pub getter_magic: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut JSContext,
            this_val: JSValue,
            magic: ::std::os::raw::c_int,
        ) -> JSValue,
    >,
    pub setter_magic: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut JSContext,
            this_val: JSValue,
            val: JSValue,
            magic: ::std::os::raw::c_int,
        ) -> JSValue,
    >,
    pub iterator_next: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut JSContext,
            this_val: JSValue,
            argc: ::std::os::raw::c_int,
            argv: *mut JSValue,
            pdone: *mut ::std::os::raw::c_int,
            magic: ::std::os::raw::c_int,
        ) -> JSValue,
    >,
}
#[test]
fn bindgen_test_layout_JSCFunctionType() {
    const UNINIT: ::std::mem::MaybeUninit<JSCFunctionType> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<JSCFunctionType>(),
        4usize,
        concat!("Size of: ", stringify!(JSCFunctionType))
    );
    assert_eq!(
        ::std::mem::align_of::<JSCFunctionType>(),
        4usize,
        concat!("Alignment of ", stringify!(JSCFunctionType))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).generic) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionType),
            "::",
            stringify!(generic)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).generic_magic) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionType),
            "::",
            stringify!(generic_magic)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).constructor) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionType),
            "::",
            stringify!(constructor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).constructor_magic) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionType),
            "::",
            stringify!(constructor_magic)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).constructor_or_func) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionType),
            "::",
            stringify!(constructor_or_func)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).f_f) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionType),
            "::",
            stringify!(f_f)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).f_f_f) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionType),
            "::",
            stringify!(f_f_f)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).getter) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionType),
            "::",
            stringify!(getter)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).setter) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionType),
            "::",
            stringify!(setter)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).getter_magic) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionType),
            "::",
            stringify!(getter_magic)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).setter_magic) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionType),
            "::",
            stringify!(setter_magic)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).iterator_next) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionType),
            "::",
            stringify!(iterator_next)
        )
    );
}
extern "C" {
    pub fn JS_NewCFunction2(
        ctx: *mut JSContext,
        func: JSCFunction,
        name: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_int,
        cproto: JSCFunctionEnum,
        magic: ::std::os::raw::c_int,
    ) -> JSValue;
}
extern "C" {
    pub fn JS_NewCFunctionData(
        ctx: *mut JSContext,
        func: JSCFunctionData,
        length: ::std::os::raw::c_int,
        magic: ::std::os::raw::c_int,
        data_len: ::std::os::raw::c_int,
        data: *mut JSValue,
    ) -> JSValue;
}
extern "C" {
    pub fn JS_SetConstructor(ctx: *mut JSContext, func_obj: JSValue, proto: JSValue);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSCFunctionListEntry {
    pub name: *const ::std::os::raw::c_char,
    pub prop_flags: u8,
    pub def_type: u8,
    pub magic: i16,
    pub u: JSCFunctionListEntry__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union JSCFunctionListEntry__bindgen_ty_1 {
    pub func: JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_1,
    pub getset: JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_2,
    pub alias: JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_3,
    pub prop_list: JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_4,
    pub str_: *const ::std::os::raw::c_char,
    pub i32_: i32,
    pub i64_: i64,
    pub f64_: f64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_1 {
    pub length: u8,
    pub cproto: u8,
    pub cfunc: JSCFunctionType,
}
#[test]
fn bindgen_test_layout_JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_1> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cproto) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(cproto)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cfunc) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(cfunc)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_2 {
    pub get: JSCFunctionType,
    pub set: JSCFunctionType,
}
#[test]
fn bindgen_test_layout_JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_2() {
    const UNINIT: ::std::mem::MaybeUninit<JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_2> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_2>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_2>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).get) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(get)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).set) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(set)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_3 {
    pub name: *const ::std::os::raw::c_char,
    pub base: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_3() {
    const UNINIT: ::std::mem::MaybeUninit<JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_3> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_3>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_3>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).base) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(base)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_4 {
    pub tab: *const JSCFunctionListEntry,
    pub len: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_4() {
    const UNINIT: ::std::mem::MaybeUninit<JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_4> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_4>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_4)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_4>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_4)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tab) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(tab)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).len) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(len)
        )
    );
}
#[test]
fn bindgen_test_layout_JSCFunctionListEntry__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<JSCFunctionListEntry__bindgen_ty_1> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<JSCFunctionListEntry__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(JSCFunctionListEntry__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<JSCFunctionListEntry__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).func) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1),
            "::",
            stringify!(func)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).getset) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1),
            "::",
            stringify!(getset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).alias) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1),
            "::",
            stringify!(alias)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).prop_list) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1),
            "::",
            stringify!(prop_list)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).str_) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1),
            "::",
            stringify!(str_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).i32_) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1),
            "::",
            stringify!(i32_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).i64_) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1),
            "::",
            stringify!(i64_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).f64_) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionListEntry__bindgen_ty_1),
            "::",
            stringify!(f64_)
        )
    );
}
#[test]
fn bindgen_test_layout_JSCFunctionListEntry() {
    const UNINIT: ::std::mem::MaybeUninit<JSCFunctionListEntry> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<JSCFunctionListEntry>(),
        16usize,
        concat!("Size of: ", stringify!(JSCFunctionListEntry))
    );
    assert_eq!(
        ::std::mem::align_of::<JSCFunctionListEntry>(),
        4usize,
        concat!("Alignment of ", stringify!(JSCFunctionListEntry))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionListEntry),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).prop_flags) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionListEntry),
            "::",
            stringify!(prop_flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).def_type) as usize - ptr as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionListEntry),
            "::",
            stringify!(def_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).magic) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionListEntry),
            "::",
            stringify!(magic)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).u) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(JSCFunctionListEntry),
            "::",
            stringify!(u)
        )
    );
}
extern "C" {
    pub fn JS_SetPropertyFunctionList(
        ctx: *mut JSContext,
        obj: JSValue,
        tab: *const JSCFunctionListEntry,
        len: ::std::os::raw::c_int,
    );
}
pub type JSModuleInitFunc = ::std::option::Option<
    unsafe extern "C" fn(ctx: *mut JSContext, m: *mut JSModuleDef) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn JS_NewCModule(
        ctx: *mut JSContext,
        name_str: *const ::std::os::raw::c_char,
        func: JSModuleInitFunc,
    ) -> *mut JSModuleDef;
}
extern "C" {
    pub fn JS_AddModuleExport(
        ctx: *mut JSContext,
        m: *mut JSModuleDef,
        name_str: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_AddModuleExportList(
        ctx: *mut JSContext,
        m: *mut JSModuleDef,
        tab: *const JSCFunctionListEntry,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_SetModuleExport(
        ctx: *mut JSContext,
        m: *mut JSModuleDef,
        export_name: *const ::std::os::raw::c_char,
        val: JSValue,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_SetModuleExportList(
        ctx: *mut JSContext,
        m: *mut JSModuleDef,
        tab: *const JSCFunctionListEntry,
        len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_GetModuleExport(
        ctx: *mut JSContext,
        m: *mut JSModuleDef,
        export_name: *const ::std::os::raw::c_char,
    ) -> JSValue;
}
extern "C" {
    pub fn JS_GetModuleExportEntriesCount(m: *mut JSModuleDef) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn JS_GetModuleExportEntry(
        ctx: *mut JSContext,
        m: *mut JSModuleDef,
        idx: ::std::os::raw::c_int,
    ) -> JSValue;
}
extern "C" {
    pub fn JS_GetModuleExportEntryName(
        ctx: *mut JSContext,
        m: *mut JSModuleDef,
        idx: ::std::os::raw::c_int,
    ) -> JSAtom;
}
pub const __JS_ATOM_NULL: _bindgen_ty_2 = 0;
pub const JS_ATOM_null: _bindgen_ty_2 = 1;
pub const JS_ATOM_false: _bindgen_ty_2 = 2;
pub const JS_ATOM_true: _bindgen_ty_2 = 3;
pub const JS_ATOM_if: _bindgen_ty_2 = 4;
pub const JS_ATOM_else: _bindgen_ty_2 = 5;
pub const JS_ATOM_return: _bindgen_ty_2 = 6;
pub const JS_ATOM_var: _bindgen_ty_2 = 7;
pub const JS_ATOM_this: _bindgen_ty_2 = 8;
pub const JS_ATOM_delete: _bindgen_ty_2 = 9;
pub const JS_ATOM_void: _bindgen_ty_2 = 10;
pub const JS_ATOM_typeof: _bindgen_ty_2 = 11;
pub const JS_ATOM_new: _bindgen_ty_2 = 12;
pub const JS_ATOM_in: _bindgen_ty_2 = 13;
pub const JS_ATOM_instanceof: _bindgen_ty_2 = 14;
pub const JS_ATOM_do: _bindgen_ty_2 = 15;
pub const JS_ATOM_while: _bindgen_ty_2 = 16;
pub const JS_ATOM_for: _bindgen_ty_2 = 17;
pub const JS_ATOM_break: _bindgen_ty_2 = 18;
pub const JS_ATOM_continue: _bindgen_ty_2 = 19;
pub const JS_ATOM_switch: _bindgen_ty_2 = 20;
pub const JS_ATOM_case: _bindgen_ty_2 = 21;
pub const JS_ATOM_default: _bindgen_ty_2 = 22;
pub const JS_ATOM_throw: _bindgen_ty_2 = 23;
pub const JS_ATOM_try: _bindgen_ty_2 = 24;
pub const JS_ATOM_catch: _bindgen_ty_2 = 25;
pub const JS_ATOM_finally: _bindgen_ty_2 = 26;
pub const JS_ATOM_function: _bindgen_ty_2 = 27;
pub const JS_ATOM_debugger: _bindgen_ty_2 = 28;
pub const JS_ATOM_with: _bindgen_ty_2 = 29;
pub const JS_ATOM_class: _bindgen_ty_2 = 30;
pub const JS_ATOM_const: _bindgen_ty_2 = 31;
pub const JS_ATOM_enum: _bindgen_ty_2 = 32;
pub const JS_ATOM_export: _bindgen_ty_2 = 33;
pub const JS_ATOM_extends: _bindgen_ty_2 = 34;
pub const JS_ATOM_import: _bindgen_ty_2 = 35;
pub const JS_ATOM_super: _bindgen_ty_2 = 36;
pub const JS_ATOM_implements: _bindgen_ty_2 = 37;
pub const JS_ATOM_interface: _bindgen_ty_2 = 38;
pub const JS_ATOM_let: _bindgen_ty_2 = 39;
pub const JS_ATOM_package: _bindgen_ty_2 = 40;
pub const JS_ATOM_private: _bindgen_ty_2 = 41;
pub const JS_ATOM_protected: _bindgen_ty_2 = 42;
pub const JS_ATOM_public: _bindgen_ty_2 = 43;
pub const JS_ATOM_static: _bindgen_ty_2 = 44;
pub const JS_ATOM_yield: _bindgen_ty_2 = 45;
pub const JS_ATOM_await: _bindgen_ty_2 = 46;
pub const JS_ATOM_empty_string: _bindgen_ty_2 = 47;
pub const JS_ATOM_length: _bindgen_ty_2 = 48;
pub const JS_ATOM_fileName: _bindgen_ty_2 = 49;
pub const JS_ATOM_lineNumber: _bindgen_ty_2 = 50;
pub const JS_ATOM_message: _bindgen_ty_2 = 51;
pub const JS_ATOM_errors: _bindgen_ty_2 = 52;
pub const JS_ATOM_stack: _bindgen_ty_2 = 53;
pub const JS_ATOM_name: _bindgen_ty_2 = 54;
pub const JS_ATOM_toString: _bindgen_ty_2 = 55;
pub const JS_ATOM_toLocaleString: _bindgen_ty_2 = 56;
pub const JS_ATOM_valueOf: _bindgen_ty_2 = 57;
pub const JS_ATOM_eval: _bindgen_ty_2 = 58;
pub const JS_ATOM_prototype: _bindgen_ty_2 = 59;
pub const JS_ATOM_constructor: _bindgen_ty_2 = 60;
pub const JS_ATOM_configurable: _bindgen_ty_2 = 61;
pub const JS_ATOM_writable: _bindgen_ty_2 = 62;
pub const JS_ATOM_enumerable: _bindgen_ty_2 = 63;
pub const JS_ATOM_value: _bindgen_ty_2 = 64;
pub const JS_ATOM_get: _bindgen_ty_2 = 65;
pub const JS_ATOM_set: _bindgen_ty_2 = 66;
pub const JS_ATOM_of: _bindgen_ty_2 = 67;
pub const JS_ATOM___proto__: _bindgen_ty_2 = 68;
pub const JS_ATOM_undefined: _bindgen_ty_2 = 69;
pub const JS_ATOM_number: _bindgen_ty_2 = 70;
pub const JS_ATOM_boolean: _bindgen_ty_2 = 71;
pub const JS_ATOM_string: _bindgen_ty_2 = 72;
pub const JS_ATOM_object: _bindgen_ty_2 = 73;
pub const JS_ATOM_symbol: _bindgen_ty_2 = 74;
pub const JS_ATOM_integer: _bindgen_ty_2 = 75;
pub const JS_ATOM_unknown: _bindgen_ty_2 = 76;
pub const JS_ATOM_arguments: _bindgen_ty_2 = 77;
pub const JS_ATOM_callee: _bindgen_ty_2 = 78;
pub const JS_ATOM_caller: _bindgen_ty_2 = 79;
pub const JS_ATOM__eval_: _bindgen_ty_2 = 80;
pub const JS_ATOM__ret_: _bindgen_ty_2 = 81;
pub const JS_ATOM__var_: _bindgen_ty_2 = 82;
pub const JS_ATOM__arg_var_: _bindgen_ty_2 = 83;
pub const JS_ATOM__with_: _bindgen_ty_2 = 84;
pub const JS_ATOM_lastIndex: _bindgen_ty_2 = 85;
pub const JS_ATOM_target: _bindgen_ty_2 = 86;
pub const JS_ATOM_index: _bindgen_ty_2 = 87;
pub const JS_ATOM_input: _bindgen_ty_2 = 88;
pub const JS_ATOM_defineProperties: _bindgen_ty_2 = 89;
pub const JS_ATOM_apply: _bindgen_ty_2 = 90;
pub const JS_ATOM_join: _bindgen_ty_2 = 91;
pub const JS_ATOM_concat: _bindgen_ty_2 = 92;
pub const JS_ATOM_split: _bindgen_ty_2 = 93;
pub const JS_ATOM_construct: _bindgen_ty_2 = 94;
pub const JS_ATOM_getPrototypeOf: _bindgen_ty_2 = 95;
pub const JS_ATOM_setPrototypeOf: _bindgen_ty_2 = 96;
pub const JS_ATOM_isExtensible: _bindgen_ty_2 = 97;
pub const JS_ATOM_preventExtensions: _bindgen_ty_2 = 98;
pub const JS_ATOM_has: _bindgen_ty_2 = 99;
pub const JS_ATOM_deleteProperty: _bindgen_ty_2 = 100;
pub const JS_ATOM_defineProperty: _bindgen_ty_2 = 101;
pub const JS_ATOM_getOwnPropertyDescriptor: _bindgen_ty_2 = 102;
pub const JS_ATOM_ownKeys: _bindgen_ty_2 = 103;
pub const JS_ATOM_add: _bindgen_ty_2 = 104;
pub const JS_ATOM_done: _bindgen_ty_2 = 105;
pub const JS_ATOM_next: _bindgen_ty_2 = 106;
pub const JS_ATOM_values: _bindgen_ty_2 = 107;
pub const JS_ATOM_source: _bindgen_ty_2 = 108;
pub const JS_ATOM_flags: _bindgen_ty_2 = 109;
pub const JS_ATOM_global: _bindgen_ty_2 = 110;
pub const JS_ATOM_unicode: _bindgen_ty_2 = 111;
pub const JS_ATOM_raw: _bindgen_ty_2 = 112;
pub const JS_ATOM_new_target: _bindgen_ty_2 = 113;
pub const JS_ATOM_this_active_func: _bindgen_ty_2 = 114;
pub const JS_ATOM_home_object: _bindgen_ty_2 = 115;
pub const JS_ATOM_computed_field: _bindgen_ty_2 = 116;
pub const JS_ATOM_static_computed_field: _bindgen_ty_2 = 117;
pub const JS_ATOM_class_fields_init: _bindgen_ty_2 = 118;
pub const JS_ATOM_brand: _bindgen_ty_2 = 119;
pub const JS_ATOM_hash_constructor: _bindgen_ty_2 = 120;
pub const JS_ATOM_as: _bindgen_ty_2 = 121;
pub const JS_ATOM_from: _bindgen_ty_2 = 122;
pub const JS_ATOM_meta: _bindgen_ty_2 = 123;
pub const JS_ATOM__default_: _bindgen_ty_2 = 124;
pub const JS_ATOM__star_: _bindgen_ty_2 = 125;
pub const JS_ATOM_Module: _bindgen_ty_2 = 126;
pub const JS_ATOM_then: _bindgen_ty_2 = 127;
pub const JS_ATOM_resolve: _bindgen_ty_2 = 128;
pub const JS_ATOM_reject: _bindgen_ty_2 = 129;
pub const JS_ATOM_promise: _bindgen_ty_2 = 130;
pub const JS_ATOM_proxy: _bindgen_ty_2 = 131;
pub const JS_ATOM_revoke: _bindgen_ty_2 = 132;
pub const JS_ATOM_async: _bindgen_ty_2 = 133;
pub const JS_ATOM_exec: _bindgen_ty_2 = 134;
pub const JS_ATOM_groups: _bindgen_ty_2 = 135;
pub const JS_ATOM_status: _bindgen_ty_2 = 136;
pub const JS_ATOM_reason: _bindgen_ty_2 = 137;
pub const JS_ATOM_globalThis: _bindgen_ty_2 = 138;
pub const JS_ATOM_bigint: _bindgen_ty_2 = 139;
pub const JS_ATOM_bigfloat: _bindgen_ty_2 = 140;
pub const JS_ATOM_bigdecimal: _bindgen_ty_2 = 141;
pub const JS_ATOM_roundingMode: _bindgen_ty_2 = 142;
pub const JS_ATOM_maximumSignificantDigits: _bindgen_ty_2 = 143;
pub const JS_ATOM_maximumFractionDigits: _bindgen_ty_2 = 144;
pub const JS_ATOM_not_equal: _bindgen_ty_2 = 145;
pub const JS_ATOM_timed_out: _bindgen_ty_2 = 146;
pub const JS_ATOM_ok: _bindgen_ty_2 = 147;
pub const JS_ATOM_toJSON: _bindgen_ty_2 = 148;
pub const JS_ATOM_Object: _bindgen_ty_2 = 149;
pub const JS_ATOM_Array: _bindgen_ty_2 = 150;
pub const JS_ATOM_Error: _bindgen_ty_2 = 151;
pub const JS_ATOM_Number: _bindgen_ty_2 = 152;
pub const JS_ATOM_String: _bindgen_ty_2 = 153;
pub const JS_ATOM_Boolean: _bindgen_ty_2 = 154;
pub const JS_ATOM_Symbol: _bindgen_ty_2 = 155;
pub const JS_ATOM_Arguments: _bindgen_ty_2 = 156;
pub const JS_ATOM_Math: _bindgen_ty_2 = 157;
pub const JS_ATOM_JSON: _bindgen_ty_2 = 158;
pub const JS_ATOM_Date: _bindgen_ty_2 = 159;
pub const JS_ATOM_Function: _bindgen_ty_2 = 160;
pub const JS_ATOM_GeneratorFunction: _bindgen_ty_2 = 161;
pub const JS_ATOM_ForInIterator: _bindgen_ty_2 = 162;
pub const JS_ATOM_RegExp: _bindgen_ty_2 = 163;
pub const JS_ATOM_ArrayBuffer: _bindgen_ty_2 = 164;
pub const JS_ATOM_SharedArrayBuffer: _bindgen_ty_2 = 165;
pub const JS_ATOM_Uint8ClampedArray: _bindgen_ty_2 = 166;
pub const JS_ATOM_Int8Array: _bindgen_ty_2 = 167;
pub const JS_ATOM_Uint8Array: _bindgen_ty_2 = 168;
pub const JS_ATOM_Int16Array: _bindgen_ty_2 = 169;
pub const JS_ATOM_Uint16Array: _bindgen_ty_2 = 170;
pub const JS_ATOM_Int32Array: _bindgen_ty_2 = 171;
pub const JS_ATOM_Uint32Array: _bindgen_ty_2 = 172;
pub const JS_ATOM_BigInt64Array: _bindgen_ty_2 = 173;
pub const JS_ATOM_BigUint64Array: _bindgen_ty_2 = 174;
pub const JS_ATOM_Float32Array: _bindgen_ty_2 = 175;
pub const JS_ATOM_Float64Array: _bindgen_ty_2 = 176;
pub const JS_ATOM_DataView: _bindgen_ty_2 = 177;
pub const JS_ATOM_BigInt: _bindgen_ty_2 = 178;
pub const JS_ATOM_BigFloat: _bindgen_ty_2 = 179;
pub const JS_ATOM_BigFloatEnv: _bindgen_ty_2 = 180;
pub const JS_ATOM_BigDecimal: _bindgen_ty_2 = 181;
pub const JS_ATOM_OperatorSet: _bindgen_ty_2 = 182;
pub const JS_ATOM_Operators: _bindgen_ty_2 = 183;
pub const JS_ATOM_Map: _bindgen_ty_2 = 184;
pub const JS_ATOM_Set: _bindgen_ty_2 = 185;
pub const JS_ATOM_WeakMap: _bindgen_ty_2 = 186;
pub const JS_ATOM_WeakSet: _bindgen_ty_2 = 187;
pub const JS_ATOM_Map_Iterator: _bindgen_ty_2 = 188;
pub const JS_ATOM_Set_Iterator: _bindgen_ty_2 = 189;
pub const JS_ATOM_Array_Iterator: _bindgen_ty_2 = 190;
pub const JS_ATOM_String_Iterator: _bindgen_ty_2 = 191;
pub const JS_ATOM_RegExp_String_Iterator: _bindgen_ty_2 = 192;
pub const JS_ATOM_Generator: _bindgen_ty_2 = 193;
pub const JS_ATOM_Proxy: _bindgen_ty_2 = 194;
pub const JS_ATOM_Promise: _bindgen_ty_2 = 195;
pub const JS_ATOM_PromiseResolveFunction: _bindgen_ty_2 = 196;
pub const JS_ATOM_PromiseRejectFunction: _bindgen_ty_2 = 197;
pub const JS_ATOM_AsyncFunction: _bindgen_ty_2 = 198;
pub const JS_ATOM_AsyncFunctionResolve: _bindgen_ty_2 = 199;
pub const JS_ATOM_AsyncFunctionReject: _bindgen_ty_2 = 200;
pub const JS_ATOM_AsyncGeneratorFunction: _bindgen_ty_2 = 201;
pub const JS_ATOM_AsyncGenerator: _bindgen_ty_2 = 202;
pub const JS_ATOM_EvalError: _bindgen_ty_2 = 203;
pub const JS_ATOM_RangeError: _bindgen_ty_2 = 204;
pub const JS_ATOM_ReferenceError: _bindgen_ty_2 = 205;
pub const JS_ATOM_SyntaxError: _bindgen_ty_2 = 206;
pub const JS_ATOM_TypeError: _bindgen_ty_2 = 207;
pub const JS_ATOM_URIError: _bindgen_ty_2 = 208;
pub const JS_ATOM_InternalError: _bindgen_ty_2 = 209;
pub const JS_ATOM_Private_brand: _bindgen_ty_2 = 210;
pub const JS_ATOM_Symbol_toPrimitive: _bindgen_ty_2 = 211;
pub const JS_ATOM_Symbol_iterator: _bindgen_ty_2 = 212;
pub const JS_ATOM_Symbol_match: _bindgen_ty_2 = 213;
pub const JS_ATOM_Symbol_matchAll: _bindgen_ty_2 = 214;
pub const JS_ATOM_Symbol_replace: _bindgen_ty_2 = 215;
pub const JS_ATOM_Symbol_search: _bindgen_ty_2 = 216;
pub const JS_ATOM_Symbol_split: _bindgen_ty_2 = 217;
pub const JS_ATOM_Symbol_toStringTag: _bindgen_ty_2 = 218;
pub const JS_ATOM_Symbol_isConcatSpreadable: _bindgen_ty_2 = 219;
pub const JS_ATOM_Symbol_hasInstance: _bindgen_ty_2 = 220;
pub const JS_ATOM_Symbol_species: _bindgen_ty_2 = 221;
pub const JS_ATOM_Symbol_unscopables: _bindgen_ty_2 = 222;
pub const JS_ATOM_Symbol_asyncIterator: _bindgen_ty_2 = 223;
pub const JS_ATOM_Symbol_operatorSet: _bindgen_ty_2 = 224;
pub const JS_ATOM_END: _bindgen_ty_2 = 225;
pub type _bindgen_ty_2 = ::std::os::raw::c_uint;
