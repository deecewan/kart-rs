use image::DynamicImage;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref HASHER: image_hasher::Hasher = image_hasher::HasherConfig::new().to_hasher();
}

#[macro_export]
macro_rules! load_reference_hash {
    ($file:expr $(,)?) => {{
        // it's probably possible to make these constant by pre-computing the
        // image hash, dumping it to base64, then loading it here using
        // ImageHash::from_base64(&str), but we're not there just yet
        // this _might_ require a procedural macro to do properly, or maybe
        // creating const functions to load all these things
        let f = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/references/", $file));
        let x = image::load_from_memory(f).unwrap();
        crate::hasher::hash_image(x)
    }};
}

pub fn hash_image(image: DynamicImage) -> image_hasher::ImageHash {
    HASHER.hash_image(&image)
}
