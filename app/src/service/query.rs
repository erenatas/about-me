use ::entity::{post, post::Entity as Post};
use sea_orm::prelude::Expr;
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_post_by_id(db: &DbConn, id: i32) -> Result<Option<post::Model>, DbErr> {
        Post::find_by_id(id).one(db).await
    }

    pub async fn list_posts(db: &DbConn) -> Result<Vec<post::Model>, DbErr> {
        // let posts = Post::find().all(db).await;
        let posts = Post::find()
            .select_only()
            .column(post::Column::Id)
            .column(post::Column::Title)
            .column(post::Column::PublicationDate)
            .column(post::Column::Views)
            .into_model::<post::Model>()
            .all(db)
            .await;
        posts
    }

    pub async fn increment_post_view(db: &DbConn, id: i32) -> Result<post::Model, DbErr> {
        // First update the views
        Post::update_many()
            .col_expr(post::Column::Views, Expr::col(post::Column::Views).add(1))
            .filter(post::Column::Id.eq(id))
            .exec(db)
            .await?;

        // Then fetch the updated post
        Post::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Post not found".to_string()))
    }
}
