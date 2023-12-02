use thin_trait_object::*;

pub type GdalDataset = *mut libc::c_void;
pub type GdalRasterBand = *mut libc::c_void;
pub type RustDatasetPtr = *mut libc::c_void;
pub type RustBandPtr = *mut libc::c_void;

#[doc(hidden)]
mod hidden_module {
    use crate::bridge::*;

    pub type GdalDatasetCreateFromRust = extern "C" fn(arg1: RustDatasetPtr) -> GdalDataset;
    pub type GdalDatasetDetachType = extern "C" fn(arg1: GdalDataset);
    pub type GdalDatasetSetSizeType = extern "C" fn(arg1: GdalDataset, arg2: i32);
    pub type GdalDatasetGetSizeType = extern "C" fn(arg1: GdalDataset) -> i32;
    pub type GdalDatasetGetBandCountType = extern "C" fn(arg1: GdalDataset) -> i32;
    pub type GdalDatasetGetBandType = extern "C" fn(arg1: GdalDataset, arg2: i32) -> GdalRasterBand;
    pub type GdalDatasetAddBandType = extern "C" fn(arg1: GdalDataset, GdalRasterBand);
    pub type GdalDatasetSayHiBaseType = extern "C" fn(arg1: GdalDataset);
    pub type GdalBandCreateFromRust = extern "C" fn(arg1: RustBandPtr) -> GdalRasterBand;
    pub type GdalBandGetRustBand = extern "C" fn(arg1: GdalRasterBand) -> RustBandPtr;
    pub type GdalBandDetachType = extern "C" fn(arg1: GdalRasterBand);
    pub type GdalBandGetNumberType = extern "C" fn(arg1: GdalRasterBand) -> i32;
    pub type GdalBandSetNumberType = extern "C" fn(arg1: GdalRasterBand, arg2: i32);
}

use crate::bridge::hidden_module::*;

static mut PFN_GDAL_DATASET_CREATE_FROM_RUST: Option<GdalDatasetCreateFromRust> = None;
static mut PFN_GDAL_DATASET_DETACH: Option<GdalDatasetDetachType> = None;
static mut PFN_GDAL_DATASET_SET_SIZE: Option<GdalDatasetSetSizeType> = None;
static mut PFN_GDAL_DATASET_GET_SIZE: Option<GdalDatasetGetSizeType> = None;
static mut PFN_GDAL_DATASET_GET_BAND_COUNT: Option<GdalDatasetGetBandCountType> = None;
static mut PFN_GDAL_DATASET_GET_BAND: Option<GdalDatasetGetBandType> = None;
static mut PFN_GDAL_DATASET_ADD_BAND: Option<GdalDatasetAddBandType> = None;
static mut PFN_GDAL_DATASET_SAY_HI_BASE: Option<GdalDatasetSayHiBaseType> = None;
static mut PFN_GDAL_BAND_CREATE_FROM_RUST: Option<GdalBandCreateFromRust> = None;
static mut PFN_GDAL_BAND_GET_RUST_BAND: Option<GdalBandGetRustBand> = None;
static mut PFN_GDAL_BAND_DETACH: Option<GdalBandDetachType> = None;
static mut PFN_GDAL_BAND_GET_NUMBER: Option<GdalBandGetNumberType> = None;
static mut PFN_GDAL_BAND_SET_NUMBER: Option<GdalBandSetNumberType> = None;

#[no_mangle]
/// Install global callbacks. This function should be called once, to
/// initialize the GDAL C++ <--> Rust bridge.
///
/// # Safety
/// called from C, hence unsafe
pub unsafe extern "C" fn gdal_driver_rust_set_c_functions(
    ds_create: GdalDatasetCreateFromRust,
    ds_detach: GdalDatasetDetachType,
    ds_set_size: GdalDatasetSetSizeType,
    ds_get_size: GdalDatasetGetSizeType,
    ds_band_count: GdalDatasetGetBandCountType,
    ds_band: GdalDatasetGetBandType,
    ds_add_band: GdalDatasetAddBandType,
    ds_base_say_hi: GdalDatasetSayHiBaseType,
    band_create: GdalBandCreateFromRust,
    band_get_rust_band: GdalBandGetRustBand,
    band_detach: GdalBandDetachType,
    band_get_number: GdalBandGetNumberType,
    band_set_number: GdalBandSetNumberType,
) {
    PFN_GDAL_DATASET_CREATE_FROM_RUST = Some(ds_create);
    PFN_GDAL_DATASET_DETACH = Some(ds_detach);
    PFN_GDAL_DATASET_SET_SIZE = Some(ds_set_size);
    PFN_GDAL_DATASET_GET_SIZE = Some(ds_get_size);
    PFN_GDAL_DATASET_GET_BAND_COUNT = Some(ds_band_count);
    PFN_GDAL_DATASET_GET_BAND = Some(ds_band);
    PFN_GDAL_DATASET_ADD_BAND = Some(ds_add_band);
    PFN_GDAL_DATASET_SAY_HI_BASE = Some(ds_base_say_hi);
    PFN_GDAL_BAND_CREATE_FROM_RUST = Some(band_create);
    PFN_GDAL_BAND_GET_RUST_BAND = Some(band_get_rust_band);
    PFN_GDAL_BAND_DETACH = Some(band_detach);
    PFN_GDAL_BAND_GET_NUMBER = Some(band_get_number);
    PFN_GDAL_BAND_SET_NUMBER = Some(band_set_number);
}

pub trait RustBandBase {
    // The method is not dangerous by itself, but dealing with its return
    // might be, hence we don't want it to be used lightly.
    #[doc(hidden)]
    unsafe fn get_base(&self) -> GdalRasterBand;

    // The method is not dangerous by itself, but shoud only be called at
    // construction time or by detach
    #[doc(hidden)]
    unsafe fn set_base(&mut self, band: GdalRasterBand);
}

/// Core band methods
#[thin_trait_object]
pub trait RustBand: RustBandBase {
    #[doc(hidden)]
    unsafe fn thunk_get_base(&self) -> GdalRasterBand {
        unsafe { self.get_base() } // Redirect to the method from the RustDatasetBase trait implementation
    }

    #[doc(hidden)]
    unsafe fn thunk_set_base(&mut self, band: GdalRasterBand) {
        unsafe {
            self.set_base(band);
        } // Redirect to the method from the RustDatasetBase trait implementation
    }

    // Should only be used by drop()
    #[doc(hidden)]
    unsafe fn detach(&mut self) {
        let base = unsafe { self.get_base() };
        assert!(!base.is_null());
        PFN_GDAL_BAND_DETACH.unwrap()(base);
        self.set_base(std::ptr::null_mut());
    }

    /// Custom cleanup method called from drop() method of implementations
    fn cleanup(&mut self) {}

    /// Return band number
    fn number(&self) -> i32 {
        let base = unsafe { self.get_base() };
        assert!(!base.is_null());
        unsafe { PFN_GDAL_BAND_GET_NUMBER.unwrap()(base) }
    }

    /// Set band number
    fn set_number(&mut self, nr: i32) {
        let base = unsafe { self.get_base() };
        assert!(!base.is_null());
        unsafe { PFN_GDAL_BAND_SET_NUMBER.unwrap()(base, nr) }
    }
}

impl RustBandBase for BoxedRustBand<'_> {
    #[doc(hidden)]
    unsafe fn get_base(&self) -> GdalRasterBand {
        self.thunk_get_base()
    }

    #[doc(hidden)]
    unsafe fn set_base(&mut self, base: GdalRasterBand) {
        self.thunk_set_base(base)
    }
}

pub struct RustBandAssociatedWithDataset<'a> {
    cpp_band: GdalRasterBand,
    dataset: &'a BoxedRustDataset<'a>,
}

impl RustBandBase for RustBandAssociatedWithDataset<'_> {
    unsafe fn get_base(&self) -> GdalRasterBand {
        self.cpp_band
    }

    unsafe fn set_base(&mut self, band: GdalRasterBand) {
        self.cpp_band = band;
    }
}

impl RustBand for RustBandAssociatedWithDataset<'_> {}

impl<'a> RustBandAssociatedWithDataset<'a> {
    unsafe fn new(
        dataset: &'a BoxedRustDataset<'a>,
        cpp_band: GdalRasterBand,
    ) -> RustBandAssociatedWithDataset<'a> {
        RustBandAssociatedWithDataset { cpp_band, dataset }
    }

    pub fn dataset(&self) -> &BoxedRustDataset<'a> {
        self.dataset
    }
}

pub trait RustDatasetBase {
    #[doc(hidden)]
    unsafe fn get_base(&self) -> GdalDataset;

    #[doc(hidden)]
    unsafe fn set_base(&mut self, band: GdalDataset);
}

#[thin_trait_object]
pub trait RustDataset: RustDatasetBase {
    #[doc(hidden)]
    unsafe fn thunk_get_base(&self) -> GdalDataset {
        unsafe { self.get_base() } // Redirect to the method from the RustDatasetBase trait implementation
    }

    #[doc(hidden)]
    unsafe fn thunk_set_base(&mut self, base: GdalDataset) {
        unsafe {
            self.set_base(base);
        } // Redirect to the method from the RustDatasetBase trait implementation
    }

    fn size(&self) -> i32 {
        unsafe { PFN_GDAL_DATASET_GET_SIZE.unwrap()(self.get_base()) }
    }
    fn set_size(&mut self, s: i32) {
        unsafe {
            PFN_GDAL_DATASET_SET_SIZE.unwrap()(self.get_base(), s);
        }
    }

    fn cleanup(&mut self) {}

    fn base_say_hi(&self) {
        unsafe {
            PFN_GDAL_DATASET_SAY_HI_BASE.unwrap()(self.get_base());
        }
    }

    fn say_hi(&self, _: &BoxedRustDataset) {
        self.base_say_hi();
    }

    #[doc(hidden)]
    unsafe fn detach(&mut self) {
        PFN_GDAL_DATASET_DETACH.unwrap()(self.get_base());
        self.set_base(std::ptr::null_mut());
    }

    fn band_count(&self) -> i32 {
        unsafe { PFN_GDAL_DATASET_GET_BAND_COUNT.unwrap()(self.get_base()) }
    }

    #[doc(hidden)]
    unsafe fn band_raw(&self, idx: i32) -> GdalRasterBand {
        unsafe { PFN_GDAL_DATASET_GET_BAND.unwrap()(self.get_base(), idx) }
    }

    // borrow passed band
    fn add_band(&self, band: BoxedRustBand) {
        unsafe {
            PFN_GDAL_DATASET_ADD_BAND.unwrap()(self.get_base(), band.get_base());
            std::mem::forget(band);
        }
    }
}

impl RustDatasetBase for BoxedRustDataset<'_> {
    #[doc(hidden)]
    unsafe fn get_base(&self) -> GdalDataset {
        self.thunk_get_base()
    }

    #[doc(hidden)]
    unsafe fn set_base(&mut self, base: GdalDataset) {
        self.thunk_set_base(base)
    }
}

pub struct RustDatasetBandIterator<'a> {
    current: i32,
    ds: &'a BoxedRustDataset<'a>,
}

impl<'a> Iterator for RustDatasetBandIterator<'a> {
    type Item = RustBandAssociatedWithDataset<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.ds.band_count() {
            let result = self.ds.band(self.current);
            self.current += 1;
            result
        } else {
            None
        }
    }
}

impl<'a> BoxedRustDataset<'a> {
    /*fn band(&'a self, idx: i32) -> Box<dyn RustBand + 'a> {
        Box::new(RustBandAssociatedWithDataset::new(self, self.band_raw(idx)))
    }*/

    pub fn band_iterator(&self) -> RustDatasetBandIterator {
        RustDatasetBandIterator {
            current: 0,
            ds: self,
        }
    }

    pub fn band(&self, idx: i32) -> Option<RustBandAssociatedWithDataset> {
        let raw_band = unsafe { self.band_raw(idx) };
        if raw_band.is_null() {
            None
        } else {
            Some(unsafe { RustBandAssociatedWithDataset::new(self, raw_band) })
        }
    }
}

impl<'a> BoxedRustDataset<'a> {
    pub fn new_box<T: RustDataset + 'a>(rust_ds: T) -> BoxedRustDataset<'a> {
        let mut ds = BoxedRustDataset::new(rust_ds);
        let raw_ds = BoxedRustDataset::as_raw(&ds) as RustDatasetPtr;
        let base_ds = unsafe { PFN_GDAL_DATASET_CREATE_FROM_RUST.unwrap()(raw_ds) };
        unsafe {
            ds.set_base(base_ds);
        }
        ds
    }

    // borrow self
    // This method should only be used when returning a new dataset to GDAL core
    // otherwise it will leak memory
    /*
    pub fn as_gdal_dataset(self) -> GdalDataset {
        unsafe {
            let base_ds = self.get_base();
            std::mem::forget(self);
            base_ds
        }
    }*/
}

impl<'a> BoxedRustBand<'a> {
    pub fn new_box<T: RustBand + 'a>(rust_band: T) -> BoxedRustBand<'a> {
        let mut b = BoxedRustBand::new(rust_band);
        let raw_band = BoxedRustBand::as_raw(&b) as RustBandPtr;
        let base_band = unsafe { PFN_GDAL_BAND_CREATE_FROM_RUST.unwrap()(raw_band) };
        unsafe {
            b.set_base(base_band);
        }
        b
    }
    /*
    // borrow self
    fn as_gdal_band(self) -> GdalRasterBand {
        unsafe {
            let base = self.get_base();
            std::mem::forget(self);
            base
        }
    }
    */
}

#[no_mangle]
#[doc(hidden)]
pub unsafe extern "C" fn gdal_driver_rust_dataset_delete(d: RustDatasetPtr) {
    if !d.is_null() {
        drop(BoxedRustDataset::from_raw(d as *mut ()));
    }
}

#[no_mangle]
#[doc(hidden)]
pub extern "C" fn gdal_driver_rust_dataset_get_gdal_dataset(d: RustDatasetPtr) -> GdalDataset {
    assert!(!d.is_null());
    let boxed_d = unsafe { BoxedRustDataset::from_raw(d as *mut ()) };
    let ret = unsafe { boxed_d.get_base() };
    BoxedRustDataset::into_raw(boxed_d);
    ret
}

#[no_mangle]
#[doc(hidden)]
pub extern "C" fn gdal_driver_rust_band_delete(band: RustBandPtr) {
    if !band.is_null() {
        drop(unsafe { BoxedRustBand::from_raw(band as *mut ()) });
    }
}

#[no_mangle]
#[doc(hidden)]
pub extern "C" fn gdal_driver_rust_dataset_say_hi(d: RustDatasetPtr) {
    assert!(!d.is_null());
    let boxed_d = unsafe { BoxedRustDataset::from_raw(d as *mut ()) };
    boxed_d.say_hi(&boxed_d);
    BoxedRustDataset::into_raw(boxed_d);
}
