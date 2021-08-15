# 3ways api

arrow to the right `->` means json is returned
arrow to the left `<-` means that the body of the request should be `x-www-form-urlencoded`
parameters between `<param>` are dynamic path based

# endpoints

```
PUT  /user_new <- User
PUT  /user_edit <- User
GET  /user_login <- UserLogin
GET  /user_logout
GET  /user/<user_name> -> User
POST /commitment_new <- commitment
GET  /commitment/<commitment_name>
GET  /commitment_search <- Search
POST /initiative_new <- Initiative
GET  /initiative -> Initiative
GET  /initiative_search <- Search
PUT  /support_add/<initiative_name>
PUT  /support_adopt/<initiative_name>
PUT  /support_remove/<initiative_name>
GET  /support -> DateTime
GET  /support_search <- Search
```

# objects

```rust
pub struct User {
    #[field(validate = len(..=45))]
    pub name: String,
    pub password: String,
    pub email: String,
    pub phone: String,
}

pub struct UserLogin {
    pub name: String,
    pub password: String,
}

pub struct Commitment {
    #[field(validate = len(..=40))]
    pub name: String,
    pub description: String,
}

pub struct Initiative {
    pub commitment_name: String,
    #[field(validate = len(..=40))]
    pub name: String,
    pub description: String,
}

pub struct Search {
    pub support: Option<String>,
    pub is_adopter: Option<bool>,
    pub commitment: Option<String>,
    pub initiative: Option<String>,
}
```