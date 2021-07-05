use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;

mod submodule;
use submodule::*;

extern crate image;

use image::GenericImageView;

#[pyfunction]
fn grayscale(img: &str) -> PyResult<bool> {

    let img = image::open(str).unwrap();

    let grey = img.grayscale();
    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    // Write the contents of this image to the Writer in PNG format.
    grey.save("test.png").unwrap();
    Ok(true)
}

#[pymodule]
fn filters(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(grayscale, m)?)?;

    Ok(())
}
