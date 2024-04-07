use dashmap::Dashmap;
use lazy_static::lazy_static;
use crate::model::subscriber::Subscriber;

// Singleton of Databsasee
lazy_static! {
    static ref SUBSCRIBERS: Dashmap<String, Dashmap<String, Subscriber>>=Dashmap::new();
}

pub struct SubscriberRepository;

impl SubscriberRepository{
    
}