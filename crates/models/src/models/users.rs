use crate::{models::Id, ConnectionPool, DatabaseResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
  pub id: Id,
  pub name: String,
  pub age: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewUser {
  pub name: String,
  pub age: i32,
}

impl User {
  pub async fn create(pool: &ConnectionPool, new_user: NewUser) -> DatabaseResult<User> {
    let res = sqlx::query_as(
      r#"
      INSERT INTO 
        users (name, age)
      VALUES 
        ($1, $2)
      RETURNING 
        id, name, age
      "#,
    )
    .bind(new_user.name)
    .bind(new_user.age)
    .fetch_one(&pool.0)
    .await?;

    Ok(res)
  }

  pub async fn find_by_id(pool: &ConnectionPool, user_id: Id) -> DatabaseResult<Option<User>> {
    let user = sqlx::query_as(
      r#"
      SELECT 
        id, name, age
      FROM
        users
      WHERE
        id = $1
      "#,
    )
    .bind(user_id)
    .fetch_optional(&pool.0)
    .await?;

    Ok(user)
  }

  pub async fn delete_by_id(pool: &ConnectionPool, user_id: Id) -> DatabaseResult<u64> {
    let res = sqlx::query(
      r#"
      DELETE FROM 
        users
      WHERE 
        id = $1
      RETURNING 
        id
      "#,
    )
    .bind(user_id)
    .execute(&pool.0)
    .await?;

    Ok(res.rows_affected())
  }
}
