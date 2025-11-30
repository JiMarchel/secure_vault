use std::sync::Once;

use tracing::{Subscriber, subscriber::set_global_default};
use tracing_subscriber::{
    EnvFilter, Registry,
    fmt::{self, MakeWriter},
    layer::SubscriberExt,
};

pub fn get_subscriber<Sink>(env_filter: String, sink: Sink) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let console_layer = fmt::layer()
        .with_target(true)
        .with_level(true)
        .json()
        .with_writer(sink);
    Registry::default().with(filter).with(console_layer)
}

static INIT: Once = Once::new();

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    INIT.call_once(|| {
        set_global_default(subscriber)
            .expect("Failed to set subscriber");
    });
}
