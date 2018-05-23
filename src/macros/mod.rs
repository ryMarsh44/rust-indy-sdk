macro_rules! c_str {
    ($x:ident) => {
        CString::new($x).unwrap()
    }
}

macro_rules! opt_c_str {
    ($x:ident) => {
        $x.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap())
    }
}

macro_rules! rust_str {
    ($x:ident) => {
        unsafe { CStr::from_ptr($x).to_str().unwrap().to_string() }
    }
}

macro_rules! rust_slice {
    ($x:ident, $y:ident) => {
        unsafe { slice::from_raw_parts($x, $y as usize) }
    }
}
