// Because of the nature of the program, testing should be done on a single thread (rust default is multi-threaded)
// Use  "cargo test -- --test-threads=1" to run the test

#[cfg(test)]
mod test {
    use std::ffi::{ CString};
    use std::os::raw::{c_ulonglong, c_longlong, c_int, c_char};
    use crate::general::*;


    // Scenario 1|2 include:
    // setting a path
    // sharing an array
    // retrieving array's flatten length, rank, dimension
    // comparing the shared memory_data with the original data

    // scenario 3 is like the 1|2 but include some repetition to discover hiccups

    // This scenario should be run first to set the path for the future uses.
    #[test]
    fn scenario_1_length_3_with_setup() {
        internal_set_shared_memory_path(r"D:\projects\Rust\10. shared memory\rust\data".to_string());
        complete_test_scenario::<1, 3, f32>(&[11., 22., 33.], &[3]);
        complete_test_scenario::<1, 3, f64>(&[11., 22., 33.], &[3]);
        complete_test_scenario::<1, 3, u8>(&[11, 22, 33], &[3]);
        complete_test_scenario::<1, 3, u16>(&[11, 22, 33], &[3]);
        complete_test_scenario::<1, 3, u32>(&[11, 22, 33], &[3]);
        complete_test_scenario::<1, 3, u64>(&[11, 22, 33], &[3]);
        complete_test_scenario::<1, 3, i8>(&[11, 22, 33], &[3]);
        complete_test_scenario::<1, 3, i16>(&[11, 22, 33], &[3]);
        complete_test_scenario::<1, 3, i32>(&[11, 22, 33], &[3]);
        complete_test_scenario::<1, 3, i64>(&[11, 22, 33], &[3]);
    }

    #[test]
    fn scenario_2_length_11() {
        complete_test_scenario::<1, 11, f32>(&[11., 22., 33., 44., 55., 66., 77., 88., 99., 100., 110.], &[11]);
        complete_test_scenario::<1, 11, f64>(&[11., 22., 33., 44., 55., 66., 77., 88., 99., 100., 110.], &[11]);
        complete_test_scenario::<1, 11, u8>(&[11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 110], &[11]);
        complete_test_scenario::<1, 11, u16>(&[11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 110], &[11]);
        complete_test_scenario::<1, 11, u32>(&[11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 110], &[11]);
        complete_test_scenario::<1, 11, u64>(&[11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 110], &[11]);
        complete_test_scenario::<1, 11, i8>(&[11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 110], &[11]);
        complete_test_scenario::<1, 11, i16>(&[11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 110], &[11]);
        complete_test_scenario::<1, 11, i32>(&[11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 110], &[11]);
        complete_test_scenario::<1, 11, i64>(&[11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 110], &[11]);
    }

    #[test]
    fn scenario_3_empty() {
        complete_test_scenario::<1, 0, f32>(&[], &[0]);
        complete_test_scenario::<1, 0, f64>(&[], &[0]);
        complete_test_scenario::<1, 0, u8>(&[], &[0]);
        complete_test_scenario::<1, 0, u16>(&[], &[0]);
        complete_test_scenario::<1, 0, u32>(&[], &[0]);
        complete_test_scenario::<1, 0, u64>(&[], &[0]);
        complete_test_scenario::<1, 0, i8>(&[], &[0]);
        complete_test_scenario::<1, 0, i16>(&[], &[0]);
        complete_test_scenario::<1, 0, i32>(&[], &[0]);
        complete_test_scenario::<1, 0, i64>(&[], &[0]);
    }


    #[test]
    fn scenario_3_length_4_repeated() {
        const DATA: [f64; 6] = [11., 22., 33., 44., 55., 66.];
        const DATA_LENGTH: usize = DATA.len();
        const DIMS: [u64; 2] = [2, 3];

        let error_code1_1 = set_shared_memory_data_float64(DATA.as_ptr(), DIMS.as_ptr(), 2);
        let error_code1_2 = set_shared_memory_data_float64(DATA.as_ptr(), DIMS.as_ptr(), 2);
        let error_code1_3 = set_shared_memory_data_float64(DATA.as_ptr(), DIMS.as_ptr(), 2);
        if error_code1_1 == 0 && error_code1_2 == 0 && error_code1_3 == 0 {
            assert_eq!(get_shared_memory_rank(), 2);
            assert_eq!(get_shared_memory_rank(), 2);
            assert_eq!(get_shared_memory_rank(), 2);

            assert_eq!(get_shared_memory_flatten_length(), 6);
            assert_eq!(get_shared_memory_flatten_length(), 6);
            assert_eq!(get_shared_memory_flatten_length(), 6);

            {
                let temp_dim = [0; 2];
                get_shared_memory_dimensions(temp_dim.as_ptr() as *mut u64);
                assert_eq!(temp_dim, DIMS);
            }

            {
                let temp_dim = [0; 2];
                get_shared_memory_dimensions(temp_dim.as_ptr() as *mut u64);
                assert_eq!(temp_dim, DIMS);
            }

            {
                let temp_dim = [0; 2];
                get_shared_memory_dimensions(temp_dim.as_ptr() as *mut u64);
                assert_eq!(temp_dim, DIMS);
            }

            {// change dimension
                let new_dim = [3, 2];
                let error_code3 = set_shared_memory_dimensions(new_dim.as_ptr());
                if error_code3 != 0 {
                    panic!("ERROR in changin dimensions");
                }
            }

            {
                let temp_dim: [u64; 2] = [0; 2];
                get_shared_memory_dimensions(temp_dim.as_ptr() as *mut u64);
                assert_eq!(temp_dim, [3u64, 2u64]);
            }

            assert_eq!(get_shared_memory_rank(), 2);

            assert_eq!(get_shared_memory_flatten_length(), 6);

            let mut temp = [0.0; DATA_LENGTH];
            let error_code2_1 = get_shared_memory_flatten_data_float64(temp.as_mut_ptr());
            let error_code2_2 = get_shared_memory_flatten_data_float64(temp.as_mut_ptr());
            let error_code2_3 = get_shared_memory_flatten_data_float64(temp.as_mut_ptr());
            if error_code2_1 == 0 && error_code2_2 == 0 && error_code2_3 == 0 {
                assert_eq!(temp, DATA);
            } else {
                panic!("ERROR in retrieving the data, {:?} ", (error_code2_1, error_code2_2, error_code2_3));
            }
        } else {
            panic!("ERROR in sharing data, {:?}", (error_code1_1, error_code1_2, error_code1_3));
        }
    }

    #[test]
    fn test_string() {

        let my_string = CString::new("This is a sample text").unwrap();

        set_shared_memory_string(my_string.as_ptr());
        assert_eq!(get_shared_memory_flatten_length(),21);

        // assert_eq!(get_shared_memory_string_utf16_length(),21);
        assert_eq!(get_shared_memory_rank(),1);
        assert_eq!(get_shared_memory_data_type(),12);

        let temp_dim: [u64; 1] = [0; 1];
        get_shared_memory_dimensions(temp_dim.as_ptr() as *mut u64);
        assert_eq!(temp_dim,[21]);

        let mut output_string:[i8;21] = [0;21];
        get_shared_memory_string(output_string.as_ptr() as *mut c_char);

        let output_string_u8 = unsafe{ std::slice::from_raw_parts(output_string.as_ptr() as *const u8, output_string.len()) };
        assert_eq!(output_string_u8,my_string.as_bytes());

        // match internal_read_shared_memory_string(){
        //     Ok(v)=> { assert_eq!(v.1,my_string.to_str().unwrap()) },
        //     Err(e) => {panic!("Error {}",e)}
        // };

    }

    #[test]
    fn final_scenario() {
        delete_shared_memory();
    }

    fn complete_test_scenario<const RANK: usize, const FLATTEN_LENGTH: usize, T>(data: &[T], dims: &[c_ulonglong])
        where T: Default + std::marker::Copy + std::cmp::PartialEq + std::fmt::Debug {
        let error_code_1 = set_shared_memory_data(data.as_ptr(), [FLATTEN_LENGTH.try_into().unwrap()].as_ptr(), RANK as c_ulonglong, 9);
        if error_code_1 == 0 {
            assert_eq!(get_shared_memory_rank(), RANK as c_int);

            assert_eq!(get_shared_memory_flatten_length(), FLATTEN_LENGTH as c_longlong);

            let temp_dim = [0; RANK];
            get_shared_memory_dimensions(temp_dim.as_ptr() as *mut u64);
            assert_eq!(temp_dim, dims);

            let mut temp = [T::default(); FLATTEN_LENGTH];
            let error_code_2 = get_shared_memory_flatten_data(temp.as_mut_ptr());
            if error_code_2 == 0 {
                assert_eq!(temp, data);
            } else {
                panic!("ERROR in retrieving the data, {} ", error_code_2);
            }
        } else {
            panic!("ERROR in sharing data, {}", error_code_1);
        }
    }
}


// This module is the core of the library and is used
//  by Julia, Matlab and Python
mod general {

    use std::ffi::{CStr};
    use std::os::raw::*;
    use shared_memory::*;
    use std::ptr::copy_nonoverlapping;
    use retry::retry;
    use retry::delay::Fixed;

    // Possible errors - all are negative
    const EMPTY_LIBRARY_PATH_ERROR_CODE: c_int = -1;
    const ACCESS_ERROR_CODE: c_int = -2;
    const PARSING_PATH_STRING_ERROR: c_int = -3;
    const CAN_NOT_CREATE_SHARED_MEMORY_ERROR: c_int = -4;
    const NEW_RANK_DOES_NOT_MATCH_PREVIOUS_RANK_ERROR: c_int = -5;
    pub const INVALID_DATA_TYPE: c_int = -6;
    const INVALID_UTF8_STRING:c_int = -7;




    const NUMBER_OF_ITEMS_WITHOUT_RANK: usize = 9;

    // GLOBAL VARIABLE
    // Used to store the shared memory path
    static mut SHARED_MEMORY_FILE_PATH: Option<String> = None;

    pub enum AddressesOffset {
        Type = 0,
        Rank = 1,
        Dimensions = 9
    }

    #[derive(PartialEq, Clone)]
    pub enum SharedMemoryType {
        Unsigned8 = 0,
        Unsigned16 = 1,
        Unsigned32 = 2,
        Unsigned64 = 3,
        Signed8 = 4,
        Signed16 = 5,
        Signed32 = 6,
        Signed64 = 7,
        Float32 = 8,
        Float64 = 9,
        ComplexFloat32 = 10,
        ComplexFloat64 = 11,
        UTF8String =12
    }

    impl From<u8> for SharedMemoryType {
        fn from(id: u8) -> Self {
            match id {
                0 => SharedMemoryType::Unsigned8,
                1 => SharedMemoryType::Unsigned16,
                2 => SharedMemoryType::Unsigned32,
                3 => SharedMemoryType::Unsigned64,
                4 => SharedMemoryType::Signed8,
                5 => SharedMemoryType::Signed16,
                6 => SharedMemoryType::Signed32,
                7 => SharedMemoryType::Signed64,
                8 => SharedMemoryType::Float32,
                9 => SharedMemoryType::Float64,
                10 => SharedMemoryType::ComplexFloat32,
                11 => SharedMemoryType::ComplexFloat64,
                12 => SharedMemoryType::UTF8String,
                _ => panic!("Invalid type id to convert to SharedMemoryType."),
            }
        }
    }

    // used for mathematica
    impl SharedMemoryType {
        pub fn to_library_link(self) -> u8 {
            match self {
                SharedMemoryType::Unsigned8 => 2,
                SharedMemoryType::Unsigned16 => 4,
                SharedMemoryType::Unsigned32 => 6,
                SharedMemoryType::Unsigned64 => 8,
                SharedMemoryType::Signed8 => 1,
                SharedMemoryType::Signed16 => 3,
                SharedMemoryType::Signed32 => 5,
                SharedMemoryType::Signed64 => 7,
                SharedMemoryType::Float32 => 9,
                SharedMemoryType::Float64 => 10,
                SharedMemoryType::ComplexFloat32 => 11,
                SharedMemoryType::ComplexFloat64 => 12,
                _ => unreachable!()
            }
        }
    }


    pub fn internal_get_shared_memory_path<'a>() -> Result<&'a String, c_int> {
        match unsafe { &SHARED_MEMORY_FILE_PATH } {
            Some(ref v) => {
                Ok(v)
            }
            None =>Err(EMPTY_LIBRARY_PATH_ERROR_CODE)
        }
    }

    pub trait SharedMemoryInterface {
        fn get_dimensions(&self) -> &[u64];
        fn get_data_type(&self) -> SharedMemoryType;
        fn get_rank(&self) -> usize;

        fn internal_get_data_type_ptr(&self) -> *const u8;
        fn internal_get_rank_ptr(&self) -> *const u8;
        fn internal_get_dimensions_ptr(&self) -> *const u8;
        fn internal_get_data_ptr(&self) -> *const u8;

        fn get_flatten_length(&self) -> usize;
        fn delete_shared_memory(self);

        fn get_data_as_cstr(&self) ->  &CStr;
        fn get_data_as_str(&self) -> Result<&str,c_int>;

        fn set_dimensions(&self,new_dimensions: &[u64]) -> c_int;
        fn new<T>(data: *const T, dims_raw: *const u64, rank: u64, type_id: u8) -> c_int;
        fn get_flatten_data<T>(&self,array_source: *mut T) -> c_int;
    }

    impl SharedMemoryInterface for Shmem {

        fn get_flatten_length(&self) -> usize {
            self.get_dimensions().iter().product::<u64>() as usize
        }

        fn get_dimensions(&self) -> &[u64] {
            unsafe { std::slice::from_raw_parts(self.internal_get_dimensions_ptr() as *const u64, self.get_rank()) }
        }

        fn get_data_type(&self) -> SharedMemoryType {
            SharedMemoryType::from(unsafe { *(self.internal_get_data_type_ptr() as *const u8) })
        }

        fn get_rank(&self) -> usize {
            unsafe { *(self.internal_get_rank_ptr() as *const usize) }
        }

        fn internal_get_data_type_ptr(&self) -> *const u8 {
            unsafe { self.as_ptr().offset(AddressesOffset::Type as isize) }
        }

        fn internal_get_rank_ptr(&self) -> *const u8 {
            unsafe { self.as_ptr().offset(AddressesOffset::Rank as isize) }
        }

        fn internal_get_dimensions_ptr(&self) -> *const u8 {
            unsafe { self.as_ptr().offset(AddressesOffset::Dimensions as isize) }
        }

        fn internal_get_data_ptr(&self) -> *const u8 {
            unsafe { self.as_ptr().offset(AddressesOffset::Dimensions as isize + self.get_rank() as isize * 8) }
        }

        fn delete_shared_memory(mut self){
            self.set_owner(true);
        }

        fn get_data_as_str(&self) -> Result<&str,c_int> {
            let temp_slice = unsafe { std::slice::from_raw_parts(self.internal_get_data_ptr(), self.get_flatten_length() - 1 ) };
            match std::str::from_utf8(temp_slice) {
                Ok(v) => Ok(v),
                Err(_) => Err(INVALID_UTF8_STRING)
            }
        }

        fn get_data_as_cstr(&self) -> &CStr {
            unsafe { CStr::from_ptr(self.internal_get_data_ptr() as *const c_char) }
        }

        fn set_dimensions(&self,new_dimensions : &[u64]) -> c_int {
            let current_rank = self.get_rank();
            if current_rank != new_dimensions.len() || new_dimensions.iter().product::<u64>() != self.get_flatten_length() as u64 {
                NEW_RANK_DOES_NOT_MATCH_PREVIOUS_RANK_ERROR
            }else {
                unsafe { copy_nonoverlapping(new_dimensions.as_ptr(), self.internal_get_dimensions_ptr() as *mut u64, current_rank) };
                0
            }
        }

        fn new<T>( data: *const T, dims_raw: *const u64, rank: u64, type_id: u8) -> c_int {
            let path = match internal_get_shared_memory_path() {
                Ok(v) => v,
                Err(e) => { return e; }
            };

            let rank_usize = rank as usize;
            let data_type = SharedMemoryType::from(type_id);
            let dims = unsafe { std::slice::from_raw_parts(dims_raw, rank_usize) };
            let data_flatten_length = dims.iter().product::<u64>() as usize * if data_type == SharedMemoryType::ComplexFloat32 || data_type == SharedMemoryType::ComplexFloat64 { 2 } else { 1 };
            let mut shared_memory = match ShmemConf::new().size((data_flatten_length + rank_usize) * 8 + NUMBER_OF_ITEMS_WITHOUT_RANK + if data_type == SharedMemoryType::UTF8String { 1 } else { 0 }).flink(path).create() {
                Ok(m) => m,
                Err(_) => {
                    return CAN_NOT_CREATE_SHARED_MEMORY_ERROR;
                }
            };
            unsafe {
                // Type
                *(shared_memory.internal_get_data_type_ptr() as *mut u8) = type_id;
                // Rank
                *(shared_memory.internal_get_rank_ptr() as *mut u64) = rank as u64;
                // Dimensions
                copy_nonoverlapping::<u64>(dims_raw, shared_memory.internal_get_dimensions_ptr() as *mut u64, rank_usize);
                // Data
                copy_nonoverlapping::<T>(data, shared_memory.internal_get_data_ptr() as *mut T, data_flatten_length);
            }
            shared_memory.set_owner(false);
            0
        }
        fn get_flatten_data<T>(&self,array_source: *mut T) -> c_int{
            let dims = self.get_dimensions();
            let data_type = self.get_data_type();
            let data_flatten_length = dims.iter().product::<u64>() as usize * if data_type == SharedMemoryType::ComplexFloat32 || data_type == SharedMemoryType::ComplexFloat64 { 2 } else { 1 };
            if data_flatten_length > 0 {
                unsafe { copy_nonoverlapping::<T>(self.internal_get_data_ptr() as *const T, array_source, data_flatten_length); }
            }
            0
        }
    }

    // Try to access the shared memory
    pub fn internal_open_shared_memory() -> Result<Shmem, c_int> {
        let path = internal_get_shared_memory_path()?;
        // ShmemConf::new().flink(path).open().map_err(|_| ACCESS_ERROR_CODE)

        retry(Fixed::from_millis(1).take(5), || {
            ShmemConf::new().flink(path).open()
        }).map_err(|_| ACCESS_ERROR_CODE)
    }

    // Set the shared memory path so other functions can work seamlessly
    pub fn internal_set_shared_memory_path(path: String) {
        unsafe { SHARED_MEMORY_FILE_PATH = Some(path) }
    }

    #[no_mangle]
    pub extern "C" fn get_shared_memory_data_type() -> c_int {
        match internal_open_shared_memory() {
            Ok(v) => v.get_data_type() as c_int,
            Err(e) => e
        }
    }

    // new dimension should match with previous rank
    #[no_mangle]
    pub extern "C" fn set_shared_memory_dimensions(new_dimensions_raw: *const c_ulonglong) -> c_int {
        match internal_open_shared_memory() {
            Ok(shared_memory) => {
                let current_rank = shared_memory.get_rank();
                let new_dimensions = unsafe { std::slice::from_raw_parts(new_dimensions_raw, current_rank) };
                shared_memory.set_dimensions(new_dimensions );
                0
            },
            Err(e) => e
        }
    }

    #[no_mangle]
    pub extern "C" fn get_shared_memory_rank() -> c_int {
        match internal_open_shared_memory() {
            Ok(v) => v.get_rank() as c_int,
            Err(e) => e
        }
    }

    #[no_mangle]
    pub extern "C" fn get_shared_memory_dimensions(array: *mut c_ulonglong) -> c_int {
        match internal_open_shared_memory() {
            Ok(v) => {
                let dims=v.get_dimensions();
                unsafe { copy_nonoverlapping::<u64>(dims.as_ptr() as *const u64, array, dims.len()) };
                0
            },
            Err(e) => e
        }
    }

    #[no_mangle]
    pub extern "C" fn set_shared_memory_path(path: *const c_char) -> c_int {
        match unsafe { CStr::from_ptr(path).to_str() } {
            Ok(v) => {
                internal_set_shared_memory_path(v.to_string());
                0
            }
            Err(_) => {
                PARSING_PATH_STRING_ERROR
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn get_shared_memory_flatten_length() -> c_longlong {
        match internal_open_shared_memory() {
            Ok(v) => v.get_flatten_length() as c_longlong,
            Err(e) => {
                e as c_longlong
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn delete_shared_memory() -> c_int {
        let path = match internal_get_shared_memory_path() {
            Ok(v) => v,
            Err(e) => { return e; }
        };
        match internal_open_shared_memory() {
            Ok(v) => {
                v.delete_shared_memory();
            },
            Err(_) => {
                // remove the file shared memory used
                let _ = std::fs::remove_file(path);
            }
        };
        0
    }


    // accept a null-terminated string
    // length of the string without null character will be saved but the shared_memory's data contain null character
    #[no_mangle]
    pub extern "C" fn set_shared_memory_string(string:*const c_char) -> c_int {
        let data = unsafe { CStr::from_ptr(string) };

        set_shared_memory_data(data.as_ptr() as *const u8,[data.to_bytes().len() as u64].as_ptr(),1,12)
    }

    #[no_mangle]
    pub extern "C" fn get_shared_memory_string(string:*mut c_char) -> c_int {

        get_shared_memory_flatten_data(string)

    }

    pub fn get_shared_memory_flatten_data<T>(array_source: *mut T) -> c_int {
        match internal_open_shared_memory() {
            Ok(shared_memory) => {
               shared_memory.get_flatten_data(array_source)
            }
            Err(e) => e
        }
    }

    #[no_mangle]
    pub extern "C" fn get_shared_memory_flatten_data_unsigned_8(array_source: *mut c_uchar) -> c_int {
        get_shared_memory_flatten_data(array_source)
    }

    #[no_mangle]
    pub extern "C" fn get_shared_memory_flatten_data_unsigned_16(array_source: *mut c_ushort) -> c_int {
        get_shared_memory_flatten_data(array_source)
    }

    #[no_mangle]
    pub extern "C" fn get_shared_memory_flatten_data_unsigned_32(array_source: *mut c_ulong) -> c_int {
        get_shared_memory_flatten_data(array_source)
    }

    #[no_mangle]
    pub extern "C" fn get_shared_memory_flatten_data_unsigned_64(array_source: *mut c_ulonglong) -> c_int {
        get_shared_memory_flatten_data(array_source)
    }

    #[no_mangle]
    pub extern "C" fn get_shared_memory_flatten_data_signed_8(array_source: *mut c_char) -> c_int {
        get_shared_memory_flatten_data(array_source)
    }

    #[no_mangle]
    pub extern "C" fn get_shared_memory_flatten_data_signed_16(array_source: *mut c_short) -> c_int {
        get_shared_memory_flatten_data(array_source)
    }

    #[no_mangle]
    pub extern "C" fn get_shared_memory_flatten_data_signed_32(array_source: *mut c_long) -> c_int {
        get_shared_memory_flatten_data(array_source)
    }

    #[no_mangle]
    pub extern "C" fn get_shared_memory_flatten_data_signed_64(array_source: *mut c_longlong) -> c_int {
        get_shared_memory_flatten_data(array_source)
    }

    #[no_mangle]
    pub extern "C" fn get_shared_memory_flatten_data_float32(array_source: *mut c_float) -> c_int {
        get_shared_memory_flatten_data(array_source)
    }

    #[no_mangle]
    pub extern "C" fn get_shared_memory_flatten_data_float64(array_source: *mut c_double) -> c_int {
        get_shared_memory_flatten_data(array_source)
    }


    pub fn set_shared_memory_data<T>(data: *const T, dims_raw: *const u64, rank: u64, type_id: u8) -> c_int {
        match internal_open_shared_memory() {
            Ok(v) => {
                v.delete_shared_memory()
            },
            Err(_) => {}
        }
        Shmem::new(data,dims_raw,rank,type_id)
    }


    #[no_mangle]
    pub extern "C" fn set_shared_memory_data_unsigned_8(
        data: *const c_uchar,
        dims_raw: *const c_ulonglong,
        rank: c_ulonglong,
    ) -> c_int {
        set_shared_memory_data(data, dims_raw, rank, 0)
    }

    #[no_mangle]
    pub extern "C" fn set_shared_memory_data_unsigned_16(
        data: *const c_ushort,
        dims_raw: *const c_ulonglong,
        rank: c_ulonglong,
    ) -> c_int {
        set_shared_memory_data(data, dims_raw, rank, 1)
    }

    #[no_mangle]
    pub extern "C" fn set_shared_memory_data_unsigned_32(
        data: *const c_ulong,
        dims_raw: *const c_ulonglong,
        rank: c_ulonglong,
    ) -> c_int {
        set_shared_memory_data(data, dims_raw, rank, 2)
    }

    #[no_mangle]
    pub extern "C" fn set_shared_memory_data_unsigned_64(
        data: *const c_ulonglong,
        dims_raw: *const c_ulonglong,
        rank: c_ulonglong,
    ) -> c_int {
        set_shared_memory_data(data, dims_raw, rank, 3)
    }

    #[no_mangle]
    pub extern "C" fn set_shared_memory_data_signed_8(
        data: *const c_char,
        dims_raw: *const c_ulonglong,
        rank: c_ulonglong,
    ) -> c_int {
        set_shared_memory_data(data, dims_raw, rank, 4)
    }

    #[no_mangle]
    pub extern "C" fn set_shared_memory_data_signed_16(
        data: *const c_short,
        dims_raw: *const c_ulonglong,
        rank: c_ulonglong,
    ) -> c_int {
        set_shared_memory_data(data, dims_raw, rank, 5)
    }

    #[no_mangle]
    pub extern "C" fn set_shared_memory_data_signed_32(
        data: *const c_long,
        dims_raw: *const c_ulonglong,
        rank: c_ulonglong,
    ) -> c_int {
        set_shared_memory_data(data, dims_raw, rank, 6)
    }

    #[no_mangle]
    pub extern "C" fn set_shared_memory_data_signed_64(
        data: *const c_longlong,
        dims_raw: *const c_ulonglong,
        rank: c_ulonglong,
    ) -> c_int {
        set_shared_memory_data(data, dims_raw, rank, 7)
    }


    #[no_mangle]
    pub extern "C" fn set_shared_memory_data_float32(
        data: *const c_float,
        dims_raw: *const c_ulonglong,
        rank: c_ulonglong,
    ) -> c_int {
        set_shared_memory_data(data, dims_raw, rank, 8)
    }

    #[no_mangle]
    pub extern "C" fn set_shared_memory_data_float64(
        data: *const c_double,
        dims_raw: *const c_ulonglong,
        rank: c_ulonglong,
    ) -> c_int {
        set_shared_memory_data(data, dims_raw, rank, 9)
    }

    #[no_mangle]
    pub extern "C" fn set_shared_memory_data_complex_float32(
        data: *const c_float,
        dims_raw: *const c_ulonglong,
        rank: c_ulonglong,
    ) -> c_int {
        set_shared_memory_data(data, dims_raw, rank, 10)
    }

    #[no_mangle]
    pub extern "C" fn set_shared_memory_data_complex_float64(
        data: *const c_double,
        dims_raw: *const c_ulonglong,
        rank: c_ulonglong,
    ) -> c_int {

        set_shared_memory_data(data, dims_raw, rank, 11)
    }


}


// This module is for Wolfram Language (Mathematica)
// If you don't have or won't use it, comment this section + it's dependency in Cargo.toml (wolfram-library-link)
mod mathematica {

    use wolfram_library_link::sys::{WolframLibraryData, mint, MArgument, MType_Real, MTensor, MType_Integer, MNumericArray};
    use crate::general::*;
    use std::os::raw::{c_char,c_int, c_ulonglong};
    use std::ptr::copy_nonoverlapping;
    use shared_memory::{Shmem};

    // pub const TENSOR_TYPE: u32 = MType_Real;

    use wolfram_library_link as wll;


    #[wll::export]
    fn set_shared_memory_path_mathematica(new_path: String) {
        internal_set_shared_memory_path(new_path)
    }

    #[wll::export]
    fn get_shared_memory_rank_mathematica() -> i64 {
        get_shared_memory_rank() as i64
    }

    #[wll::export]
    fn get_shared_memory_data_type_mathematica() -> i64 {
        get_shared_memory_data_type() as i64
    }

    #[no_mangle]
    pub unsafe extern "C" fn get_shared_memory_flatten_length_mathematica(
        _lib_data: WolframLibraryData,
        _arg_count: mint,
        _args: *mut MArgument,
        res: MArgument,
    ) -> c_int {
        match internal_open_shared_memory() {
            Ok(v) => {
                *res.integer = v.get_flatten_length() as mint;
                0
            },
            Err(e) => e
        }
    }



    #[no_mangle]
    pub unsafe extern "C" fn get_shared_memory_dimensions_mathematica(
        lib_data: WolframLibraryData,
        _arg_count: mint,
        _args: *mut MArgument,
        res: MArgument,
    ) -> c_int {
        let mut output: MTensor = std::ptr::null_mut();

        match internal_open_shared_memory() {
            Ok(v) => {
                let dims = v.get_dimensions();

                let error_code = ((*lib_data).MTensor_new.unwrap())(MType_Integer.into(), 1, [dims.len()].as_ptr() as *const mint, &mut output as *mut MTensor);
                if error_code == 0 {
                    copy_nonoverlapping::<i64>(dims.as_ptr() as *const i64, ((*lib_data).MTensor_getIntegerData.unwrap())(output), dims.len());
                    *res.tensor = output
                }
                error_code
            }
            Err(e) => e
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn get_shared_memory_flatten_data_float64_mathematica(
        lib_data: WolframLibraryData,
        _arg_count: mint,
        _args: *mut MArgument,
        res: MArgument,
    ) -> c_int {
        let mut output: MTensor = std::ptr::null_mut();

        {
            let data_flatten_length = match internal_open_shared_memory() {
                Ok(v) => v.get_flatten_length(),
                Err(e) => { return e; }
            };

            ((*lib_data).MTensor_new.unwrap())(MType_Real.into(), 1, [data_flatten_length as mint].as_ptr(), &mut output as *mut MTensor);
        }
        let error_code = get_shared_memory_flatten_data_float64(((*lib_data).MTensor_getRealData.unwrap())(output));
        if error_code == 0 {
            *res.tensor = output;
        }

        error_code
    }

    #[no_mangle]
    pub unsafe extern "C" fn get_shared_memory_flatten_data_signed_64_mathematica(
        lib_data: WolframLibraryData,
        _arg_count: mint,
        _args: *mut MArgument,
        res: MArgument,
    ) -> c_int {
        let mut output: MTensor = std::ptr::null_mut();

        {
            let data_flatten_length = match internal_open_shared_memory() {
                Ok(v) => v.get_flatten_length(),
                Err(e) => { return e; }
            };

            ((*lib_data).MTensor_new.unwrap())(MType_Integer.into(), 1, [data_flatten_length as mint].as_ptr(), &mut output as *mut MTensor);
        }
        let error_code = get_shared_memory_flatten_data_signed_64(((*lib_data).MTensor_getIntegerData.unwrap())(output));
        if error_code == 0 {
            *res.tensor = output;
        }

        error_code
    }

    #[no_mangle]
    pub unsafe extern "C" fn get_shared_memory_flatten_data_numeric_array_mathematica(
        lib_data: WolframLibraryData,
        _arg_count: mint,
        _args: *mut MArgument,
        res: MArgument,
    ) -> c_int {
        let mut numeric_array: MNumericArray = std::ptr::null_mut();

        let numeric_array_functions = *(*lib_data).numericarrayLibraryFunctions;


        let shared_memory = match internal_open_shared_memory() {
            Ok(v) => v,
            Err(e) => { return e; }
        };

        let shared_memory_data_type = shared_memory.get_data_type();
        let numeric_array_type = shared_memory_data_type.clone().to_library_link();

        // internal_get_shared_memory_dimensions_raw(&shared_memory).as_ptr() as *const i64
        (numeric_array_functions.MNumericArray_new.unwrap())(numeric_array_type.into(), 1, [shared_memory.get_flatten_length()].as_ptr() as *const i64 , &mut numeric_array);
        let numeric_array_data = (numeric_array_functions.MNumericArray_getData.unwrap())(numeric_array);
        let error_code = match shared_memory_data_type {
            SharedMemoryType::Unsigned8 => get_shared_memory_flatten_data_unsigned_8(numeric_array_data as *mut u8),
            SharedMemoryType::Unsigned16 => get_shared_memory_flatten_data_unsigned_16(numeric_array_data as *mut u16),
            SharedMemoryType::Unsigned32 => get_shared_memory_flatten_data_unsigned_32(numeric_array_data as *mut u32),
            SharedMemoryType::Unsigned64 => get_shared_memory_flatten_data_unsigned_64(numeric_array_data as *mut u64),
            SharedMemoryType::Signed8 => get_shared_memory_flatten_data_signed_8(numeric_array_data as *mut i8),
            SharedMemoryType::Signed16 => get_shared_memory_flatten_data_signed_16(numeric_array_data as *mut i16),
            SharedMemoryType::Signed32 => get_shared_memory_flatten_data_signed_32(numeric_array_data as *mut i32),
            SharedMemoryType::Signed64 => get_shared_memory_flatten_data_signed_64(numeric_array_data as *mut i64),
            SharedMemoryType::Float32 => get_shared_memory_flatten_data_float32(numeric_array_data as *mut f32),
            SharedMemoryType::Float64 => get_shared_memory_flatten_data_float64(numeric_array_data as *mut f64),
            SharedMemoryType::ComplexFloat32 => get_shared_memory_flatten_data_float32(numeric_array_data as *mut f32),
            SharedMemoryType::ComplexFloat64 => get_shared_memory_flatten_data_float64(numeric_array_data as *mut f64),
            _ => unreachable!()
        };
        if error_code == 0 {
            *res.numeric = numeric_array;
        }

        0
    }

    static mut MATHEMATICA_STRING_CONFIG:Option<Shmem> = None;

    #[no_mangle]
    pub unsafe extern "C" fn get_shared_memory_string_mathematica(
        _lib_data: WolframLibraryData,
        _arg_count: mint,
        _args: *mut MArgument,
        res: MArgument,
    ) -> c_int {
        match internal_open_shared_memory() {
            Ok(v)=> {
                *res.utf8string = v.get_data_as_cstr().as_ptr() as *mut c_char;
                MATHEMATICA_STRING_CONFIG = Some(v);

                // let temp = CString::new("abc").unwrap();
                // *res.utf8string = temp.as_ptr() as *mut c_char;
                // std::mem::forget(temp);
                0
            },
            Err(e) => e
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn internal_free_string_mathematica(
        _lib_data: WolframLibraryData,
        _arg_count: mint,
        _args: *mut MArgument,
        _res: MArgument,
    ) -> c_int {
        MATHEMATICA_STRING_CONFIG = None;
        0
    }

    #[no_mangle]
    pub unsafe extern "C" fn delete_shared_memory_mathematica(
        _lib_data: WolframLibraryData,
        _arg_count: mint,
        _args: *mut MArgument,
        _res: MArgument,
    ) -> c_int {
        delete_shared_memory()
    }

    #[no_mangle]
    pub unsafe extern "C" fn set_shared_memory_dimensions_mathematica(
        lib_data: WolframLibraryData,
        _arg_count: mint,
        args: *mut MArgument,
        _res: MArgument,
    ) -> c_int {
        let new_dimensions_tensor = *((*args).tensor);

        // tensor data pointer
        let new_dimensions_raw = (*lib_data).MTensor_getIntegerData.unwrap()(new_dimensions_tensor);

        let new_dimensions_rank = (*lib_data).MTensor_getFlattenedLength.unwrap()(new_dimensions_tensor) as usize;
        let new_dimension = std::slice::from_raw_parts(new_dimensions_raw, new_dimensions_rank).iter().map(|x: &mint| *x as u64).collect::<Vec<u64>>();

        set_shared_memory_dimensions(new_dimension.as_ptr() as *const c_ulonglong)
    }

    #[no_mangle]
    pub unsafe extern "C" fn set_shared_memory_data_float64_mathematica(
        lib_data: WolframLibraryData,
        _arg_count: mint,
        args: *mut MArgument,
        _res: MArgument,
    ) -> c_int {

        // get the tensor (assuming f64|Real64)
        let data = *((*args).tensor);

        // tensor data pointer
        let data_raw = (*lib_data).MTensor_getRealData.unwrap()(data);

        let data_dims_raw = (*lib_data).MTensor_getDimensions.unwrap()(data);
        let data_rank = (*lib_data).MTensor_getRank.unwrap()(data) as usize;
        let data_dims = std::slice::from_raw_parts(data_dims_raw, data_rank).iter().map(|x: &mint| *x as u64).collect::<Vec<u64>>();

        set_shared_memory_data_float64(data_raw, data_dims.as_ptr() as *const c_ulonglong, data_rank as c_ulonglong)
    }

    #[no_mangle]
    pub unsafe extern "C" fn set_shared_memory_data_signed_64_mathematica(
        lib_data: WolframLibraryData,
        _arg_count: mint,
        args: *mut MArgument,
        _res: MArgument,
    ) -> c_int {

        // get the tensor (assuming f64|Real64)
        let data = *((*args).tensor);

        // tensor data pointer
        let data_raw = (*lib_data).MTensor_getIntegerData.unwrap()(data);

        let data_dims_raw = (*lib_data).MTensor_getDimensions.unwrap()(data);
        let data_rank = (*lib_data).MTensor_getRank.unwrap()(data) as usize;
        let data_dims = std::slice::from_raw_parts(data_dims_raw, data_rank).iter().map(|x: &mint| *x as u64).collect::<Vec<u64>>();

        set_shared_memory_data_signed_64(data_raw, data_dims.as_ptr() as *const c_ulonglong, data_rank as c_ulonglong)
    }

    #[no_mangle]
    pub unsafe extern "C" fn set_shared_memory_string_mathematica(
        lib_data: WolframLibraryData,
        _arg_count: mint,
        args: *mut MArgument,
        _res: MArgument,
    ) -> c_int {
        let mathematica_string = *((*args).utf8string);
        let error_code = set_shared_memory_string(mathematica_string);
        (*lib_data).UTF8String_disown.unwrap()(mathematica_string);
        error_code
    }

    #[no_mangle]
    pub unsafe extern "C" fn set_shared_memory_data_numeric_array_mathematica(
        lib_data: WolframLibraryData,
        _arg_count: mint,
        args: *mut MArgument,
        _res: MArgument,
    ) -> c_int {

        let numeric_array = *((*args).numeric);

        let numeric_array_functions = *(*lib_data).numericarrayLibraryFunctions;

        let numeric_array_rank = (numeric_array_functions.MNumericArray_getRank.unwrap())(numeric_array) as u64;
        let numeric_array_dimensions = (numeric_array_functions.MNumericArray_getDimensions.unwrap())(numeric_array) as *const u64;

        // let numeric_array_flatten_length = numeric_array_dimensions.iter().product() as usize;
        let numeric_array_type = (numeric_array_functions.MNumericArray_getType.unwrap())(numeric_array);
        let numeric_array_data = (numeric_array_functions.MNumericArray_getData.unwrap())(numeric_array);
        match numeric_array_type {
            // MNumericArray_Type_Bit8
            1 => set_shared_memory_data_signed_8(numeric_array_data as *const i8, numeric_array_dimensions, numeric_array_rank),

            // MNumericArray_Type_UBit8
            2 => set_shared_memory_data_unsigned_8(numeric_array_data as *const u8, numeric_array_dimensions, numeric_array_rank),

            // MNumericArray_Type_Bit16
            3 => set_shared_memory_data_signed_16(numeric_array_data as *const i16, numeric_array_dimensions, numeric_array_rank),

            // MNumericArray_Type_UBit16
            4 => set_shared_memory_data_unsigned_16(numeric_array_data as *const u16, numeric_array_dimensions, numeric_array_rank),

            // MNumericArray_Type_Bit32
            5 => set_shared_memory_data_signed_32(numeric_array_data as *const i32, numeric_array_dimensions, numeric_array_rank),

            // MNumericArray_Type_UBit32
            6 => set_shared_memory_data_unsigned_32(numeric_array_data as *const u32, numeric_array_dimensions, numeric_array_rank),

            // MNumericArray_Type_Bit64
            7 => set_shared_memory_data_signed_64(numeric_array_data as *const i64, numeric_array_dimensions, numeric_array_rank),

            // MNumericArray_Type_UBit64
            8 => set_shared_memory_data_unsigned_64(numeric_array_data as *const u64, numeric_array_dimensions, numeric_array_rank),

            // MNumericArray_Type_Real32
            9 => set_shared_memory_data_float32(numeric_array_data as *const f32, numeric_array_dimensions, numeric_array_rank),

            // MNumericArray_Type_Real64
            10 => set_shared_memory_data_float64(numeric_array_data as *const f64, numeric_array_dimensions, numeric_array_rank),

            // MNumericArray_Type_Complex_Real32
            11 => set_shared_memory_data_complex_float32(numeric_array_data as *const f32, numeric_array_dimensions, numeric_array_rank),

            // MNumericArray_Type_Complex_Real64
            12 => set_shared_memory_data_complex_float64(numeric_array_data as *const f64, numeric_array_dimensions, numeric_array_rank),

            _ => INVALID_DATA_TYPE
        }
    }
}