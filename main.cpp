#include <stdlib.h>
#include <stdio.h>

#include "c_header.h"
#include "cpp_header.h"

int main(void) {

    gdal_driver_rust_set_c_functions(
        gdal_driver_c_api_dataset_create_from_rust,
        gdal_driver_c_api_dataset_detach_from_rust,
        gdal_driver_c_api_dataset_set_size,
        gdal_driver_c_api_dataset_get_size,
        gdal_driver_c_api_dataset_get_band_count,
        gdal_driver_c_api_dataset_get_band,
        gdal_driver_c_api_dataset_add_band,
        gdal_driver_c_api_dataset_say_hi_base,
        gdal_driver_c_api_band_create_from_rust,
        gdal_driver_c_api_band_get_rust_band,
        gdal_driver_c_api_band_detach_from_rust,
        gdal_driver_c_api_band_get_number,
        gdal_driver_c_api_band_set_number
    );

    auto rust_ds = gdal_driver_rust_dummy_driver_create_dataset();
    if( rust_ds )
    {
        auto ds = std::unique_ptr<GDALDataset>(gdal_driver_rust_dataset_get_gdal_dataset(rust_ds));
        if( ds )
            ds->say_hi();
    }

    return 0;
}
