use crate::models::BitImage;
use crate::schema::*;
use diesel::dsl::*;
use diesel::expression::SqlLiteral;
use diesel::pg::Pg;
use diesel::prelude::{Queryable, Selectable};
use diesel::*;
use diesel::{sql_types, QueryDsl};
#[derive(Queryable)]
struct BitImageListItem {
    pub bit_image: BitImage,
    pub user_name: String,
    pub my_score: Option<i16>,
    pub like_count: i64,
    pub dislike_count: i64,
}

impl<DB> Selectable<DB> for BitImageListItem
where
    DB: diesel::backend::Backend,
{
    type SelectExpression = (
        (
            bit_image::id,
            bit_image::title,
            bit_image::description,
            bit_image::create_at,
            bit_image::update_at,
            bit_image::user_id,
        ),
        user_::name,
        //SqlLiteral<sql_types::Nullable<SmallInt>>,
        SqlLiteral<sql_types::BigInt>,
        SqlLiteral<sql_types::BigInt>,
    );

    fn construct_selection() -> Self::SelectExpression {
        let bil_c = diesel::alias!(bit_image_like as c);

        //let a:bit_image_like::BoxedQuery<_,_> = bil_c.into_boxed();

        let a = bil_c
            .select(bil_c.field(bit_image_like::score))
            .filter(
                bil_c
                    .field(bit_image_like::bit_image_id)
                    .eq(bit_image::id)
                    .and(bil_c.field(bit_image_like::user_id).eq(1)),
            )
            //.into_boxed()//<<===== here is the err
            .single_value();
        (
            bit_image::all_columns,
            user_::name,
            //a,
            sql::<sql_types::BigInt>(
                "count (case when bit_image_like.score = 1 then 1 else null end)",
            ),
            sql::<sql_types::BigInt>(
                "count (case when bit_image_like.score = -1 then 1 else null end)",
            ),
        )
    }
}

fn what_into_boxed_type_after_join(user_id: Option<i32>) -> bit_image::BoxedQuery<'static, Pg> //what type here ?
{
    let a = bit_image::table.left_join(bit_image_like::table);
    let b = a.into_boxed();
    b
}
