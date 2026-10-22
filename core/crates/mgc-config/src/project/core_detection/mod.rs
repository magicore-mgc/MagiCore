//! Public project core-detection methods and private manifest recognizers.
//! API nhận diện core dự án và bộ nhận diện manifest nội bộ.
mod core_detection;
mod error;
mod manifest_declarations;
mod signatures;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
