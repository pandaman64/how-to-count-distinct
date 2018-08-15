table! {
    t (id) {
        id -> Int4,
        labels -> Array<Text>,
    }
}

#[derive(Insertable)]
#[table_name = "t"]
pub struct NewRow<'a> {
    pub labels: &'a [String],
}
