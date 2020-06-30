use serde_json;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    #[serde(alias = "post")]
    pub inner: PostInner,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostInner {
    pub id: i64,
    pub title: String,
    pub comment: String,
    pub tags: Vec<Tag>,
    pub rating: String,
    pub thumb: Option<Thumb>,
    pub thumb_micro: Option<String>,
    pub show_adult_thumb: bool,
    pub posted_at: String,
    pub likes_count: i64,
    pub liked: bool,
    pub is_contributor: bool,
    pub uri: NextUri,
    pub is_pulish_open: bool,
    pub is_blog: bool,
    pub converted_at: String,
    pub fanclub_brand: i64,
    pub special_reaction: Option<serde_json::Value>,
    pub redirect_url_from_save: String,
    pub fanclub: Fanclub,
    pub status: String,
    pub post_contents: Vec<PostContent>,
    pub deadline: Option<String>,
    pub publish_reserved_at: Option<String>,
    pub comments: Comments,
    pub blog_comment: String,
    pub comments_reactions: Comments,
    pub reactions: Comments,
    pub reaction_types_url: String,
    pub ogp_api_url: String,
    pub links: Links,
    pub is_fanclub_tip_accept: bool,
    pub is_fanclub_joined: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comments {
    pub get_url: String,
    pub post_uri: Option<String>,
    pub delete_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fanclub {
    pub id: i64,
    pub user: User,
    pub category: FanclubCategory,
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
    pub recent_posts: Vec<Post>,
    pub recent_products: Vec<Product>,
    pub plans: Vec<Plan>,
    pub background: Option<String>,
    pub point_top_users: Vec<PointTopUser>,
    pub support_point: i64,
    pub support_point_goals: Vec<SupportPointGoal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FanclubCategory {
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
    pub medium: Option<String>,
    pub main: String,
    pub ogp: String,
    pub original: String,
    pub small: Option<String>,
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
    pub vacant_seat: Option<VacantSeat>,
    pub order: Option<Order>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub status: String,
    pub is_oneclick: bool,
    pub uri: Option<String>,
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
    pub medium: Option<String>,
    pub main: Option<String>,
    pub original: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub toranoana_identify_token: String,
    pub name: String,
    pub image: Image,
    pub profile_text: Option<String>,
    pub has_fanclub: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub small: String,
    pub medium: String,
    pub large: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub product_type: String,
    pub category: RecentProductCategory,
    pub thumb: Cover,
    pub show_adult_thumb: bool,
    pub stock: String,
    pub price: i64,
    pub buyable_lowest_plan: Plan,
    pub likes: Likes,
    pub uri: String,
    pub reactions: Comments,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentProductCategory {
    pub id: i64,
    pub name: String,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Likes {
    pub count: i64,
    pub has_like: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupportPointGoal {
    pub id: i64,
    pub title: String,
    pub point: i64,
    pub description: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FanclubUri {
    pub show: String,
    pub posts: String,
    pub plans: String,
    pub products: String,
    pub users: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    pub previous: Option<Next>,
    pub next: Option<Next>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Next {
    pub id: i64,
    pub title: String,
    pub comment: String,
    pub tags: Vec<Tag>,
    pub rating: String,
    pub thumb: Option<Thumb>,
    pub thumb_micro: Option<String>,
    pub show_adult_thumb: bool,
    pub posted_at: String,
    pub likes_count: i64,
    pub liked: bool,
    pub is_contributor: bool,
    pub uri: NextUri,
    pub is_pulish_open: bool,
    pub is_blog: bool,
    pub converted_at: String,
    pub fanclub_brand: i64,
    pub special_reaction: Option<serde_json::Value>,
    pub redirect_url_from_save: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Thumb {
    pub thumb: String,
    pub medium: String,
    pub large: String,
    pub main: String,
    pub ogp: Option<String>,
    pub micro: String,
    pub original: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NextUri {
    pub show: String,
    pub edit: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentPost {
    pub title: String,
    pub url: String,
    pub date: String,
    pub deadline: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostContentPhoto {
    pub id: i64,
    pub url: Thumb,
    pub comment: Option<String>,
    pub show_original_uri: String,
    pub is_converted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostContent {
    pub id: i64,
    pub title: String,
    pub visible_status: String,
    pub published_state: String,
    pub category: String,
    pub comment: Option<String>,
    pub embed_url: Option<String>,
    pub content_type: Option<String>,
    pub comments: Comments,
    pub comments_reactions: Comments,
    pub embed_api_url: String,
    pub reactions: Comments,
    pub reaction_types_url: String,
    pub post_content_photos: Option<Vec<PostContentPhoto>>,
    pub post_content_photos_micro: Vec<String>,
    pub plan: Option<Plan>,
    pub product: Option<Product>,
    pub onsale_backnumber: Option<String>,
    pub backnumber_link: Option<String>,
    pub join_status: Option<String>,
    pub parent_post: ParentPost,
    pub is_converted: Option<bool>,
    pub filename: Option<String>,
    pub download_uri: Option<String>,
    pub hls_uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VacantSeat {
    Integer(i64),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OnsaleBacknumber {
    Bool(bool),
    String(String),
}
