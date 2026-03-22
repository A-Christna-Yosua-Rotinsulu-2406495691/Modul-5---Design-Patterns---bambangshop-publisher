use std::thread;

use bambangshop::{compose_error_response, Result};
use rocket::http::Status;

use crate::model::notification::Notification;
use crate::model::product::Product;
use crate::model::subscriber::Subscriber;
use crate::repository::subscriber::SubscriberRepository;

pub struct NotificationService;

impl NotificationService {
    pub fn subscribe(product_type: &str, subscriber: Subscriber) -> Result<Subscriber> {
        let product_type_upper = product_type.to_uppercase();
        let product_type_str = product_type_upper.as_str();

        let subscriber_result =
            SubscriberRepository::add(product_type_str, subscriber);

        Ok(subscriber_result)
    }
}