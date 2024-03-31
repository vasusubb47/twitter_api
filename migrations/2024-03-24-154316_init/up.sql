CREATE TABLE client(
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    email VARCHAR(35) NOT NULL,
    first_name VARCHAR(35) NOT NULL,
    last_name VARCHAR(35) NOT NULL,
    user_name VARCHAR(35) NOT NULL,
    gender CHAR(1),
    dob date NOT NULL,
    hasspass VARCHAR(255) NOT NULL,
    followers_count INTEGER NOT NULL DEFAULT 0,
    following_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP(0) WITH TIME zone NOT NULL DEFAULT now(),
    updated_at TIMESTAMP(0) WITH TIME zone NULL
);

CREATE TABLE tweet(
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    cid UUID NOT NULL,
    tweet_body VARCHAR(255) NOT NULL,
    reply_tid UUID NULL,
    re_tid UUID NULL,
    hash_tags JSONB NULL,
    likes INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP(0) WITH TIME zone NOT NULL DEFAULT now(),
    updated_at TIMESTAMP(0) WITH TIME zone NULL,
    
    FOREIGN KEY(cid) REFERENCES client(id),
    FOREIGN KEY(reply_tid) REFERENCES tweet(id),
    FOREIGN KEY(re_tid) REFERENCES tweet(id)
);

CREATE TABLE followers(
    cid UUID NOT NULL,
    fid UUID NOT NULL,
    
    PRIMARY KEY(cid, fid),
    
    FOREIGN KEY(cid) REFERENCES client(id),
    FOREIGN KEY(fid) REFERENCES client(id)
);

CREATE TABLE mentioned_tweets(
    tid UUID NOT NULL,
    cid UUID NOT NULL,
    
    PRIMARY KEY(cid, tid),
    
    FOREIGN KEY(cid) REFERENCES client(id),
    FOREIGN KEY(tid) REFERENCES tweet(id)
);

CREATE TABLE hash_tags(
    tag VARCHAR(255) NOT NULL,
    tid uuid NOT NULL,
    
    PRIMARY KEY(tag, tid),
    
    FOREIGN KEY(tid) REFERENCES tweet(id)
);