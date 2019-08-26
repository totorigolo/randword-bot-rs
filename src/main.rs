fn main() {
    env_logger::init();

    // Loads env variables in `.env`
    kankyo::load().expect("Failed to load .env file");

    randword_bot_rs::run();
}
