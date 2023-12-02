#[macro_export]
macro_rules! create_dataset_struct {

    // Base case for the recursion
    (@expand_fields $struct_name:ident<$lifetime:tt> { $($field_name:ident : $field_type:ty,)* }) => {
        pub struct $struct_name<$lifetime> {
            cpp_dataset: GdalDataset,
            phantom: std::marker::PhantomData<&$lifetime GdalDataset>,
            $($field_name: $field_type,)*
        }

        // Drop implementation
        impl Drop for $struct_name<'_> {
            fn drop(&mut self) {
                self.cleanup();
                unsafe {
                    self.detach();
                }
            }
        }

        impl RustDatasetBase for $struct_name<'_> {
            unsafe fn get_base(&self) -> GdalDataset {
                self.cpp_dataset
            }

            unsafe fn set_base(&mut self, base: GdalDataset) {
                self.cpp_dataset = base;
            }
        }
    };

    // Define a macro pattern for generating fields with a given type
    ($struct_name:ident<$lifetime:tt> { $($field_name:ident : $field_type:ty),* }) => {
        create_dataset_struct!(@expand_fields $struct_name<$lifetime> { $($field_name : $field_type,)* });
    };
}

#[macro_export]
macro_rules! create_band_struct {
    // Base case for the recursion
    (@expand_fields $struct_name:ident<$lifetime:tt> { $($field_name:ident : $field_type:ty,)* }) => {
        pub struct $struct_name<$lifetime> {
            cpp_band: GdalRasterBand,
            phantom: std::marker::PhantomData<&$lifetime GdalRasterBand>,
            $($field_name: $field_type,)*
        }

        // Drop implementation
        impl Drop for $struct_name<'_> {
            fn drop(&mut self) {
                self.cleanup();
                unsafe {
                    self.detach();
                }
            }
        }

        impl RustBandBase for $struct_name<'_> {
            unsafe fn get_base(&self) -> GdalRasterBand {
                self.cpp_band
            }

            unsafe fn set_base(&mut self, band: GdalRasterBand) {
                self.cpp_band = band;
            }
        }
    };

    // Define a macro pattern for generating fields with a given type
    ($struct_name:ident<$lifetime:tt> { $($field_name:ident : $field_type:ty),* }) => {
        create_band_struct!(@expand_fields $struct_name<$lifetime> { $($field_name : $field_type,)* });
    };
}
