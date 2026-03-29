// Copyright (c) 2026 Lucio Di Capua <ldcproductions@proton.me>
// Licensed under the Prosperity Public License 3.0.0
// Commercial use requires a license. See LICENSE.md for details.

//! Tipi database per password salvate.

use crate::{PasswordScore, SecretBox, SecretString};
use sqlx::FromRow;
use sqlx_template::SqlxTemplate;
use uuid::Uuid;

use crate::{DbSecretString, DbSecretVec};

/// Struct per l'autenticazione utente.
#[derive(FromRow, Debug)]
pub struct UserAuth {
    pub id: i64,
    pub password: DbSecretString,
}

/// Struct per una password salvata nel database.
#[derive(FromRow, Debug, Clone, SqlxTemplate)]
#[table("passwords")]
#[db("sqlite")]
#[tp_upsert(by = "id")]
#[tp_select_builder]
pub struct StoredPassword {
    pub id: Option<i64>,
    pub user_id: i64,
    pub name: String,
    pub username: DbSecretVec,
    pub username_nonce: Vec<u8>,
    pub url: DbSecretVec,
    pub url_nonce: Vec<u8>,
    pub password: DbSecretVec,
    pub password_nonce: Vec<u8>,
    pub notes: Option<DbSecretVec>,
    pub notes_nonce: Option<Vec<u8>>,
    pub score: PasswordScore,
    pub created_at: Option<String>,
}

impl StoredPassword {
    /// Crea una nuova struct [`StoredPassword`].
    pub fn new(
        id: Option<i64>,
        user_id: i64,
        name: String,
        username: SecretBox<[u8]>,
        username_nonce: Vec<u8>,
        url: SecretBox<[u8]>,
        url_nonce: Vec<u8>,
        password: SecretBox<[u8]>,
        notes: Option<SecretBox<[u8]>>,
        notes_nonce: Option<Vec<u8>>,
        score: PasswordScore,
        created_at: Option<String>,
        password_nonce: Vec<u8>,
    ) -> Self {
        let username: DbSecretVec = username.into();
        let url: DbSecretVec = url.into();
        let password: DbSecretVec = password.into();
        let notes: Option<DbSecretVec> = notes.map(|n| n.into());

        StoredPassword {
            id,
            user_id,
            name,
            username,
            username_nonce,
            url,
            url_nonce,
            password,
            password_nonce,
            notes,
            notes_nonce,
            score,
            created_at,
        }
    }
}

/// Password non criptata per uso interno.
/// Include un UUID in memoria per identificazione univoca nella UI.
#[derive(Clone)]
pub struct StoredRawPassword {
    /// UUID generato in memoria (non salvato nel DB)
    pub uuid: Uuid,
    pub id: Option<i64>,
    #[allow(unused)]
    pub user_id: i64,
    pub name: String,
    pub username: SecretString,
    pub url: SecretString,
    pub password: SecretString,
    pub notes: Option<SecretString>,
    pub score: Option<PasswordScore>,
    pub created_at: Option<String>,
}

impl std::fmt::Debug for StoredRawPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StoredRawPassword")
            .field("uuid", &self.uuid)
            .field("id", &self.id)
            .field("user_id", &self.user_id)
            .field("name", &self.name)
            .field("username", &"***SECRET***")
            .field("url", &"***SECRET***")
            .field("password", &"***SECRET***")
            .field("notes", &self.notes.as_ref().map(|_| "***SECRET***"))
            .field("score", &self.score)
            .field("created_at", &self.created_at)
            .finish()
    }
}

impl StoredRawPassword {
    pub fn new() -> Self {
        StoredRawPassword {
            uuid: Uuid::new_v4(),
            id: None,
            user_id: 0,
            name: String::new(),
            username: SecretString::new("".into()),
            url: SecretString::new("".into()),
            password: "".to_string().into(),
            notes: None,
            score: None,
            created_at: None,
        }
    }

}

impl PartialEq for StoredRawPassword {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}
