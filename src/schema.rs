table! {
    members (id) {
        id -> Varchar,
        employee_num -> Nullable<Int4>,
        name -> Varchar,
        leader -> Bool,
        mail -> Varchar,
        phone -> Varchar,
        business_connection -> Varchar,
        memo -> Varchar,
        joined -> Timestamptz,
    }
}

