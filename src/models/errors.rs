use poem_openapi::Object;

#[derive(Object)]
pub struct ResponseError {
    pub message: String,
}   