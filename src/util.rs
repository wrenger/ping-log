use std::time::Duration;

pub async fn sleep(duration: Duration) {
    #[cfg(feature = "server")]
    tokio::time::sleep(duration).await;
    #[cfg(not(feature = "server"))]
    gloo_timers::future::TimeoutFuture::new(duration.as_millis() as u32).await;
}
