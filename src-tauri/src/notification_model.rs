pub mod model {
    use serde::{Deserialize, Serialize};
    
    #[derive(Serialize, Deserialize)]
    pub struct Notification {
        pub id: String,
        pub updated_at: String,
        pub subject: Subject,
        pub repository: Repository,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Subject {
        pub title: String,
        pub url: String,
        #[serde(rename = "type")]
        pub subject_type: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Repository {
        pub full_name: String,
        pub owner: Owner,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Owner {
        pub avatar_url: String,
    }
}
