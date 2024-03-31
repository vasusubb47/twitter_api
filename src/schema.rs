// @generated automatically by Diesel CLI.

diesel::table! {
    client (id) {
        id -> Uuid,
        #[max_length = 35]
        email -> Varchar,
        #[max_length = 35]
        first_name -> Varchar,
        #[max_length = 35]
        last_name -> Varchar,
        #[max_length = 35]
        user_name -> Varchar,
        #[max_length = 1]
        gender -> Nullable<Bpchar>,
        dob -> Date,
        #[max_length = 255]
        hasspass -> Varchar,
        followers_count -> Int4,
        following_count -> Int4,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    followers (cid, fid) {
        cid -> Uuid,
        fid -> Uuid,
    }
}

diesel::table! {
    hash_tags (tag, tid) {
        #[max_length = 255]
        tag -> Varchar,
        tid -> Uuid,
    }
}

diesel::table! {
    mentioned_tweets (cid, tid) {
        tid -> Uuid,
        cid -> Uuid,
    }
}

diesel::table! {
    tweet (id) {
        id -> Uuid,
        cid -> Uuid,
        #[max_length = 255]
        tweet_body -> Varchar,
        reply_tid -> Nullable<Uuid>,
        re_tid -> Nullable<Uuid>,
        hash_tags -> Nullable<Jsonb>,
        likes -> Int4,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(hash_tags -> tweet (tid));
diesel::joinable!(mentioned_tweets -> client (cid));
diesel::joinable!(mentioned_tweets -> tweet (tid));
diesel::joinable!(tweet -> client (cid));

diesel::allow_tables_to_appear_in_same_query!(
    client,
    followers,
    hash_tags,
    mentioned_tweets,
    tweet,
);
