use serde_json;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FanClub {
    pub fanclub: Fanclub,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fanclub {
    pub id: i64,
    pub user: User,
    pub category: Category,
    pub fanclub_name_with_creator_name: String,
    pub fanclub_name_or_creator_name: String,
    pub title: String,
    pub cover: Cover,
    pub icon: Icon,
    pub is_join: bool,
    pub fan_count: i64,
    pub posts_count: i64,
    pub products_count: i64,
    pub uri: FanclubUri,
    pub user_support_point: i64,
    pub is_blocked: bool,
    pub creator_name: String,
    pub name: String,
    pub fanclub_name: String,
    pub comment: String,
    pub recent_posts: Vec<RecentPost>,
    pub recent_products: Vec<Option<serde_json::Value>>,
    pub plans: Vec<Plan>,
    pub background: Option<serde_json::Value>,
    pub point_top_users: Vec<PointTopUser>,
    pub support_point: i64,
    pub support_point_goals: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub uri: CategoryUri,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryUri {
    pub fanclub: String,
    pub products: String,
    pub posts: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cover {
    pub thumb: String,
    pub medium: String,
    pub main: String,
    pub ogp: String,
    pub original: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Icon {
    pub thumb: String,
    pub main: String,
    pub original: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plan {
    pub id: i64,
    pub price: i64,
    pub name: String,
    pub description: String,
    pub limit: i64,
    pub thumb: String,
    pub vacant_seat: String,
    pub order: Order,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub status: String,
    pub is_oneclick: bool,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointTopUser {
    pub id: i64,
    pub support_comment: String,
    pub support_image: SupportImage,
    pub support_point: i64,
    pub extra_pay_plan: i64,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupportImage {
    pub medium: Option<serde_json::Value>,
    pub main: Option<serde_json::Value>,
    pub original: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub toranoana_identify_token: String,
    pub name: String,
    pub image: Image,
    pub profile_text: Option<String>,
    pub has_fanclub: Option<bool>,
    pub fanclub: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub small: String,
    pub medium: String,
    pub large: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentPost {
    pub id: i64,
    pub title: String,
    pub comment: Option<serde_json::Value>,
    pub tags: Vec<Option<serde_json::Value>>,
    pub rating: String,
    pub thumb: Option<serde_json::Value>,
    pub thumb_micro: String,
    pub show_adult_thumb: bool,
    pub posted_at: String,
    pub likes_count: i64,
    pub liked: Option<serde_json::Value>,
    pub is_contributor: bool,
    pub uri: RecentPostUri,
    pub is_pulish_open: bool,
    pub is_blog: bool,
    pub converted_at: String,
    pub fanclub_brand: i64,
    pub special_reaction: Option<SpecialReaction>,
    pub redirect_url_from_save: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialReaction {
    pub reaction: String,
    pub kind: String,
    pub display_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentPostUri {
    pub show: String,
    pub edit: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FanclubUri {
    pub show: String,
    pub posts: String,
    pub plans: String,
    pub products: String,
    pub users: String,
}
