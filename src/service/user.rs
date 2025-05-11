use crate::entity::user::Model;
use crate::routes::result::ErrorMessage;
use crate::service::password::encrypt_password;
use crate::DATABASE;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, NotSet, QueryFilter, Set};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub user_id: i32,
    #[serde(skip_serializing)]
    pub password: String,
    pub name: String,
    pub desc: String,
    pub avatar: String,
}

impl User {
    pub async fn register(
        raw_password: &str,
        name: String,
        desc: Option<String>,
        avatar: Option<String>,
    ) -> Result<User, ErrorMessage> {
        if crate::entity::prelude::User::find()
            .filter(crate::entity::user::Column::Name.eq(&name))
            .one(&*DATABASE)
            .await?
            .is_some()
        {
            return Err(ErrorMessage::TooManySubmit);
        }

        crate::entity::user::ActiveModel {
            id: NotSet,
            password: Set(encrypt_password(raw_password)),
            name: Set(name),
            desc: Set(desc.unwrap_or_default()),
            avatar: Set(avatar.unwrap_or_default()),
        }
        .insert(&*DATABASE)
        .await
        .map(|e| e.into())
        .map_err(|e| e.into())
    }

    pub async fn get_by_id(user_id: i32) -> Option<User> {
        crate::entity::user::Entity::find_by_id(user_id)
            .one(&*DATABASE)
            .await
            .unwrap()
            .map(|u| u.into())
    }
}

impl From<Model> for User {
    fn from(value: Model) -> Self {
        Self {
            user_id: value.id,
            password: value.password,
            name: value.name,
            desc: value.desc,
            avatar: value.avatar,
        }
    }
}
