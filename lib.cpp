#include "c_header.h"
#include "cpp_header.h"

#include <stdio.h>

int gdal_driver_c_api_dataset_get_band_count(GDALDataset* d)
{
    return d->get_band_count();
}

GDALRasterBand* gdal_driver_c_api_dataset_get_band(GDALDataset* d, int i)
{
    return d->get_band(i);
}
 
void gdal_driver_c_api_dataset_set_size(GDALDataset* d, int s)
{
    d->m_size = s;
}

int gdal_driver_c_api_dataset_get_size(GDALDataset* d)
{
    return d->get_size();
}

void gdal_driver_c_api_dataset_add_band(GDALDataset* d, GDALRasterBand* b)
{
    d->add_band(b);
}

void gdal_driver_c_api_dataset_say_hi_base(GDALDataset* d)
{
    d->GDALDataset::say_hi();
}

int gdal_driver_c_api_band_get_number(GDALRasterBand* b)
{
    return b->get_band_number();
}

void gdal_driver_c_api_band_set_number(GDALRasterBand* b, int nr)
{
    b->m_nr = nr;
}

// Rust specific below


struct RustWrapperDataset: public GDALDataset
{
    RustDataset* m_rustDs = nullptr;
    bool m_bInDestructor = false;

    RustWrapperDataset(RustDataset* rustDs): m_rustDs(rustDs) {
        std::cout << "RustWrapperDataset()" << std::endl;
    }

    void say_hi() override {
        gdal_driver_rust_dataset_say_hi(m_rustDs);
    }

    void detach_from_rust() {
        m_rustDs = nullptr;
        if( !m_bInDestructor )
            delete this;
    }

    ~RustWrapperDataset() override {
        std::cout << "~RustWrapperDataset()" << std::endl;
        if( m_rustDs ) {
            m_bInDestructor = true;
            gdal_driver_rust_dataset_delete(m_rustDs);
        }
    }
};

GDALDataset* gdal_driver_c_api_dataset_create_from_rust(RustDataset* d)
{
    return new RustWrapperDataset(d);
}

void gdal_driver_c_api_dataset_detach_from_rust(GDALDataset* d)
{
    static_cast<RustWrapperDataset*>(d)->detach_from_rust();
}

struct RustWrapperBand: public GDALRasterBand
{
    RustBand* m_rustBand = nullptr;
    bool m_bInDestructor = false;

    RustWrapperBand(RustBand* rustBand): m_rustBand(rustBand) {
        std::cout << "RustWrapperBand()" << std::endl;
    }
    void detach_from_rust() {
        m_rustBand = nullptr;
        if( !m_bInDestructor )
            delete this;
    }

    ~RustWrapperBand() override {
        std::cout << "~RustWrapperBand()" << std::endl;
        if( m_rustBand ) {
            m_bInDestructor = true;
            gdal_driver_rust_band_delete(m_rustBand);
        }
    }
};

extern GDALRasterBand* gdal_driver_c_api_band_create_from_rust(RustBand* b)
{
    return new RustWrapperBand(b);
}

void gdal_driver_c_api_band_detach_from_rust(GDALRasterBand* b)
{
    static_cast<RustWrapperBand*>(b)->detach_from_rust();
}

RustBand* gdal_driver_c_api_band_get_rust_band(GDALRasterBand* b)
{
    return static_cast<RustWrapperBand*>(b)->m_rustBand;
}
