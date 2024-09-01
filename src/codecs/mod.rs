//! Encoding and decoding for various image file formats.
//!
//! # Supported formats
//!
//! <!--- NOTE: Make sure to keep this table in sync with the README -->
//!
//! | Format   | Decoding                                  | Encoding                                |
//! | -------- | ----------------------------------------- | --------------------------------------- |
//! | AVIF     | Yes (8-bit only) \*                       | Yes (lossy only)                        |
//! | BMP      | Yes                                       | Yes                                     |
//! | DDS      | Yes                                       | ---                                     |
//! | Farbfeld | Yes                                       | Yes                                     |
//! | GIF      | Yes                                       | Yes                                     |
//! | HDR      | Yes                                       | Yes                                     |
//! | ICO      | Yes                                       | Yes                                     |
//! | JPEG     | Yes                                       | Yes                                     |
//! | EXR      | Yes                                       | Yes                                     |
//! | PNG      | Yes                                       | Yes                                     |
//! | PNM      | Yes                                       | Yes                                     |
//! | QOI      | Yes                                       | Yes                                     |
//! | TGA      | Yes                                       | Yes                                     |
//! | TIFF     | Yes                                       | Yes                                     |
//! | WebP     | Yes                                       | Yes (lossless only)                     |
//!
//! - \* Requires the `avif-native` feature, uses the libdav1d C library.
//!
//! ## A note on format specific features
//!
//! One of the main goals of `image` is stability, in runtime but also for programmers. This
//! ensures that performance as well as safety fixes reach a majority of its user base with little
//! effort. Re-exporting all details of its dependencies would run counter to this goal as it
//! linked _all_ major version bumps between them and `image`. As such, we are wary of exposing too
//! many details, or configuration options, that are not shared between different image formats.
//!
//! Nevertheless, the advantage of precise control is hard to ignore. We will thus consider
//! _wrappers_, not direct re-exports, in either of the following cases:
//!
//! 1. A standard specifies that configuration _x_ is required for decoders/encoders and there
//!    exists an essentially canonical way to control it.
//! 2. At least two different implementations agree on some (sub-)set of features in practice.
//! 3. A technical argument including measurements of the performance, space benefits, or otherwise
//!    objectively quantified benefits can be made, and the added interface is unlikely to require
//!    breaking changes.
//!
//! Features that fulfill two or more criteria are preferred.
//!
//! Re-exports of dependencies that reach version `1` will be discussed when it happens.
    #[cfg(any(feature = "avif", feature = "avif-native"))]
    pub mod avif;
    #[cfg(feature = "bmp")]
    pub mod bmp;
    #[cfg(feature = "dds")]
    pub mod dds;
    #[cfg(feature = "ff")]
    pub mod farbfeld;
    #[cfg(feature = "gif")]
    pub mod gif;
    #[cfg(feature = "hdr")]
    pub mod hdr;
    #[cfg(feature = "ico")]
    pub mod ico;
    #[cfg(feature = "jpeg")]
    pub mod jpeg;
    #[cfg(feature = "exr")]
    pub mod openexr;
    #[cfg(feature = "png")]
    pub mod png;
    #[cfg(feature = "pnm")]
    pub mod pnm;
    #[cfg(feature = "qoi")]
    pub mod qoi;
    #[cfg(feature = "tga")]
    pub mod tga;
    #[cfg(feature = "tiff")]
    pub mod tiff;
    #[cfg(feature = "webp")]
    pub mod webp;

    #[cfg(feature = "dds")]
    mod dxt;
