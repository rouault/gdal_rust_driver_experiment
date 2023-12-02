use crate::bridge::*;
use crate::*;

// Macro definining struct MyRustDataset
create_dataset_struct!(MyRustDataset<'a> {});

impl<'a> MyRustDataset<'a> {
    fn new_box() -> BoxedRustDataset<'a> {
        BoxedRustDataset::new_box(MyRustDataset {
            cpp_dataset: std::ptr::null_mut(),
            phantom: std::marker::PhantomData,
        })
    }

    fn open() -> Option<BoxedRustDataset<'a>> {
        let mut d = MyRustDataset::new_box();
        d.set_size(512);
        d.add_band(MyRustBand::new_box(1));
        d.add_band(MyRustBand::new_box(2));
        MyRustDataset::new_box();
        MyRustBand::new_box(1);
        Some(d)
    }
}

impl RustDataset for MyRustDataset<'_> {
    /// Optional
    fn say_hi(&self, boxed: &BoxedRustDataset) {
        println!("hi there!");
        for band in boxed.band_iterator() {
            println!("band number = {}", band.number());
        }
        let band1 = boxed.band(0).expect("null_band");
        println!("band1 number = {}", band1.number());
        let band2 = boxed.band(1);
        println!("band2 number = {}", band2.expect("null_band").number());
        let ds2 = band1.dataset();
        ds2.band(0);
        ds2.base_say_hi();
    }

    /// Optional
    fn cleanup(&mut self) {
        println!("RustDataset struct being dropped.");
    }
}

// Macro definining struct MyRustBand
create_band_struct!(MyRustBand<'a> {});

impl<'a> MyRustBand<'a> {
    fn new_box(nr: i32) -> BoxedRustBand<'a> {
        let mut b = BoxedRustBand::new_box(MyRustBand {
            cpp_band: std::ptr::null_mut(),
            phantom: std::marker::PhantomData,
        });
        b.set_number(nr);
        b
    }
}

impl RustBand for MyRustBand<'_> {
    /// Optional
    fn cleanup(&mut self) {
        println!("RustBand struct being dropped.");
    }
}

#[no_mangle]
pub extern "C" fn gdal_driver_rust_dummy_driver_create_dataset() -> RustDatasetPtr {
    match MyRustDataset::open() {
        Some(d) => BoxedRustDataset::into_raw(d) as RustDatasetPtr,
        None => std::ptr::null_mut(),
    }
}
