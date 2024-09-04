pub fn returns_leaked_type() -> secret::ShouldNotLeak {
    secret::ShouldNotLeak
}

mod secret {

    /// This documentation cannot be read by our library.
    ///
    /// In fact, this whole type is not known, so nothing can link to it.
    ///
    /// We can see those through `cargo doc --document-private-items` though.
    ///
    /// We cannot know that it implements `Debug`...
    #[derive(Debug)]
    pub struct ShouldNotLeak;

    impl ShouldNotLeak {
        /// rustdoc doesn´t support this documentation.
        /// rust analyzer can suggest it to me, and I can use it.
        pub fn how_do_i_discover_this(&self) {
            println!("This is undocumented.")
        }
    }
}
