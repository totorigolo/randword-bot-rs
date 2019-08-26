use log::*;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub trait ToUsername {
    fn to_username(&self, user_id: UserId) -> String;
}

impl ToUsername for &mut Context {
    fn to_username(&self, user_id: UserId) -> String {
        user_id
            .to_user(&*self)
            .map(|user| user.name)
            .unwrap_or_else(|err| {
                warn!("Failed to fetch username: {:?}", err);
                "?".to_string()
            })
    }
}
