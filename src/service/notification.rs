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

    pub fn unsubscribe(product_type: &str, url: &str) -> Result<Subscriber> {
        let product_type_upper = product_type.to_uppercase();

        let result = SubscriberRepository::delete(product_type_upper.as_str(), url);

        if let Some(subscriber) = result {
            Ok(subscriber)
        } else {
            Err(compose_error_response(
                Status::NotFound,
                String::from("Subscriber not found."),
            ))
        }
    }

    pub fn notify(&self, product_type: &str, status: &str, product: Product) {
        let mut payload: Notification = Notification {
            product_title: product.clone().title,
            product_type: String::from(product_type),
            product_url: product.clone().get_url(),
            subscriber_name: String::from(""),
            status: String::from(status),
        };

        let subscribers: Vec<Subscriber> = SubscriberRepository::list_all(product_type);

        for subscriber in subscribers {
            payload.subscriber_name = subscriber.clone().name;

            let subscriber_clone = subscriber.clone();
            let payload_clone = payload.clone();

            thread::spawn(move || subscriber_clone.update(payload_clone));
        }
    }
}