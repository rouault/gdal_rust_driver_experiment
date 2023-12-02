#ifndef C_HEADER
#define C_HEADER

extern "C" {

// Generic C API
typedef struct GDALDataset GDALDataset;
typedef struct GDALRasterBand GDALRasterBand;

void gdal_driver_c_api_dataset_set_size(GDALDataset*, int);
int gdal_driver_c_api_dataset_get_size(GDALDataset*);
int gdal_driver_c_api_dataset_get_band_count(GDALDataset*);
GDALRasterBand* gdal_driver_c_api_dataset_get_band(GDALDataset*, int);
void gdal_driver_c_api_dataset_add_band(GDALDataset*, GDALRasterBand*);
void gdal_driver_c_api_dataset_say_hi_base(GDALDataset*);

int gdal_driver_c_api_band_get_number(GDALRasterBand*);
void gdal_driver_c_api_band_set_number(GDALRasterBand*, int);


// C <--> Rust API

typedef struct RustDataset RustDataset;
typedef struct RustBand RustBand;

// Defined in GDAL bridge
GDALDataset* gdal_driver_c_api_dataset_create_from_rust(RustDataset*);
void gdal_driver_c_api_dataset_detach_from_rust(GDALDataset* d);

// Defined in GDAL bridge
GDALRasterBand* gdal_driver_c_api_band_create_from_rust(RustBand*);
void gdal_driver_c_api_band_detach_from_rust(GDALRasterBand* d);

// Defined in GDAL bridge
RustBand* gdal_driver_c_api_band_get_rust_band(GDALRasterBand* b);

// Defined in Rust bridge
GDALDataset* gdal_driver_rust_dataset_get_gdal_dataset(RustDataset* d);

// Defined in Rust bridge
// Delete the Rust counterpart of a RustWrapperDataset
void gdal_driver_rust_dataset_delete(RustDataset* d);

// Defined in Rust bridge
void gdal_driver_rust_dataset_say_hi(RustDataset* d);

// Defined in Rust bridge
// Delete the Rust counterpart of a RustWrapperBand
void gdal_driver_rust_band_delete(RustBand* b);

// Defined in Rust bridge
void gdal_driver_rust_set_c_functions(
    GDALDataset* (*p_gdal_driver_c_api_dataset_create_from_rust)(RustDataset*),
    void (*p_gdal_driver_c_api_dataset_detach_from_rust)(GDALDataset*),
    void (*p_gdal_driver_c_api_dataset_set_size)(GDALDataset*, int),
    int (*p_gdal_driver_c_api_dataset_get_size)(GDALDataset*),
    int (*p_gdal_driver_c_api_dataset_get_band_count)(GDALDataset*),
    GDALRasterBand* (*p_gdal_driver_c_api_dataset_get_band)(GDALDataset*, int),
    void (*p_gdal_driver_c_api_dataset_add_band)(GDALDataset*, GDALRasterBand*),
    void (*p_gdal_driver_c_api_dataset_say_hi_base)(GDALDataset*),
    GDALRasterBand* (*p_gdal_driver_c_api_band_create_from_rust)(RustBand*),
    RustBand* (*p_gdal_driver_c_api_band_get_rust_band)(GDALRasterBand* b),
    void (*p_gdal_driver_c_api_band_detach_from_rust)(GDALRasterBand*),
    int (*p_gdal_driver_c_api_band_get_number)(GDALRasterBand*),
    void (*p_gdal_driver_c_api_band_set_number)(GDALRasterBand*, int)
);

// Specific to a given driver
RustDataset* gdal_driver_rust_dummy_driver_create_dataset();

}

#endif
