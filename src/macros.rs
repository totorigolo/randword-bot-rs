macro_rules! get {
    ($data:expr, $what:ty) => {
        $data
            .get::<$what>()
            .expect(concat!("Failed to get ", stringify!($what)))
    };
}

macro_rules! get_mut {
    ($data:expr, $what:ty) => {
        $data
            .get_mut::<$what>()
            .expect(concat!("Failed to get ", stringify!($what)))
    };
}
