
#include <memory>
#include <vector>
#include <iostream>

class GDALRasterBand
{
    int m_nr = 0;

    friend void gdal_driver_c_api_band_set_number(GDALRasterBand*, int);

public:
    virtual ~GDALRasterBand() = default;

    int get_band_number() const { return m_nr; }
};

class GDALDataset
{
    int m_size = 0;
    std::vector<std::unique_ptr<GDALRasterBand>> m_bands{};

    friend void gdal_driver_c_api_dataset_set_size(GDALDataset*, int);

public:

    GDALRasterBand* get_band(int i) {
        if( i >= 0 && i < static_cast<int>(m_bands.size()) )
            return m_bands[i].get();
        return nullptr;
    }

    int get_band_count() const { return static_cast<int>(m_bands.size()); }

    void add_band(GDALRasterBand* b) { m_bands.emplace_back(b); }

    int get_size() const { return m_size; }

    virtual void say_hi() { std::cout << "GDALDataset::say_hi()" << std::endl; }

    virtual ~GDALDataset() = default;
};
