//! Rust bindings to the OpenSlide C library.
//!
//! The official project site can be found at http://openslide.org/.
//!
//! This is an unofficial hobby project, and has no affiliations with the official OpenSlide project.
//!
//! ## Requirements
//!
//! This library has been built against
//!
//! ```text,no_run
//! OpenSlide 3.4.1
//! Rust 1.26
//! ```
//!
//! No downwards compatible guarantees can be made.
//!
//! ## Building OpenSlide
//!
//! Download the latest release version (3.4.1) from `https://openslide.org/download/` to some
//! location (here, `DOWNLOAD_DIR`)
//!
//! Build the project somewhere (here, `BUILD_DIR`):
//!
//! ```shell,no_run
//! cd BUILD_DIR
//! tar xvzf DOWNLOAD_DIR/openslide-3.4.1.tar.gz
//! cd openslide-3.4.1
//! ./configure
//! make
//! sudo make install
//! ```
//!
//! In the build output, you will see something like this
//!
//! ```text,no_run
//! Libraries have been installed in:
//!    /usr/local/lib
//! ```
//!
//! To make the library discoverable, we append it (call it `LIB_DIR`) to the `LD_LIBRARY_PATH`
//! environment variable
//!
//! ```shell,no_run
//! export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/local/lib
//! ```
//!
//! You should now be able to compile, and run a project using this OpenSlide binding.
//!


extern crate libc;
extern crate failure;
extern crate image;

/*
pub use bindings::{OpenSlideT,
                   detect_vendor,
                   open,
                   close,
                   get_level_count,
                   get_level0_dimensions,
                   get_level_dimensions,
                   get_level_downsample,
                   get_best_level_for_downsample,
                   read_region,
                   get_error,
                   get_property_names,
                   get_property_value,
};
*/

pub use convenience::{OpenSlide,
};

pub mod bindings;
pub mod utils;
mod convenience;
