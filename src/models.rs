use crate::pagination::*;
use crate::schema::*;
use diesel::*;
use diesel::{
    dsl::{count, exists, sql, sum},
};
use std::time::SystemTime;

pub type DJError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Queryable, Insertable, AsChangeset, Debug, PartialEq, Clone)]
#[table_name = "bit_image_like"]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(BitImage))]
pub struct BitImageLike {
    pub bit_image_id: i32,
    pub user_id: i32,
    pub score: i16,
    pub create_at: SystemTime,
}

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Debug, PartialEq, Clone)]
#[table_name = "user_"]
pub struct User {
    pub id: i32,
    pub name: String,
}
#[derive(Queryable, Identifiable, Insertable, AsChangeset, Debug, PartialEq)]
#[table_name = "bit_image"]
#[diesel(belongs_to(User))]
pub struct BitImage {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub create_at: SystemTime,
    pub update_at: SystemTime,
    pub user_id: i32,
}

diesel::allow_columns_to_appear_in_same_group_by_clause!(
    user_::id,
    user_::name,
    bit_image::id,
    bit_image::title,
    bit_image::description,
    bit_image::create_at,
    bit_image::update_at,
    bit_image::user_id,
);
use std::cmp::min;
pub fn this_fun_is_what_i_use_in_my_porduct(
    conn: &mut PgConnection,
    user_id: Option<i32>,
    page: i64,
    per_page: i64,
    order_by_date: bool,
) -> Result<ListWithTotleCount<BitImageListItem>, DJError> {
    let bil_c = diesel::alias!(bit_image_like as c);

    let subquery = bil_c
        .select(bil_c.field(bit_image_like::score))
        .filter(
            bil_c
                .field(bit_image_like::bit_image_id)
                .eq(bit_image::id)
                .and(
                    bil_c
                        .field(bit_image_like::user_id)
                        .eq(user_id.unwrap_or(0)),
                        /*
                        here i really want to do is
                        match user_id {
                            Some(_)=> subquery,
                            None => sql::<sql_types::Nullable<SmallInt>>("null")
                        }
                        but i dont know how to achive something like this
                        */
                ),
        )
        //.into_boxed()  <<===== if i add into_boxed() here also err
        .single_value();

    let query = bit_image::table
        .inner_join(user_::table.on(user_::id.eq(bit_image::user_id)))
        .left_join(bit_image_like::table.on(bit_image_like::bit_image_id.eq(bit_image::id)))
        .group_by((bit_image::id, user_::id))
        .select((
            bit_image::all_columns,
            user_::name,
            subquery,
            sql::<sql_types::BigInt>(
                "count (case when bit_image_like.score = 1 then 1 else null end)",
            ),
            sql::<sql_types::BigInt>(
                "count (case when bit_image_like.score = -1 then 1 else null end)",
            ),
        ))
        .into_boxed();
    let (query, total_pages) = match order_by_date {
        true => {
            let query = query.order_by(bit_image::create_at.desc()).paginate(page);
            let query = query.per_page(min(per_page, 20));
            query.load_and_count_pages::<BitImageListItem>(conn)?
        }
        false => {
            let query = query
                .order_by(sql::<sql_types::BigInt>(
                    "count (case when bit_image_like.score = 1 then 1 else null end) - count (case when bit_image_like.score = -1 then 1 else null end)",
                ).desc())
                .paginate(page);
            let query = query.per_page(min(per_page, 20));
            query.load_and_count_pages::<BitImageListItem>(conn)?
        }
    };

    Ok(ListWithTotleCount {
        data: query,
        totle_count: total_pages,
    })
}

#[derive(Queryable, Debug)]
pub struct BitImageListItem {
    pub bit_image: BitImage,
    pub user_name: String,
    pub my_score: Option<i16>,
    pub like_count: i64,
    pub dislike_count: i64,
}

#[derive(Debug)]
pub struct ListWithTotleCount<T> {
    pub data: Vec<T>,
    pub totle_count: i64,
}
