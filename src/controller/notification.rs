use rocket::response::status::Created;
use rocket::serde::json::Json;

use bambangship::Result;
use crate::model::subscriber::Subscriber;
use crate::service::notification::NotificationService;