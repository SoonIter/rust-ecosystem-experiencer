use futures::executor::block_on;
use tracing::{info, instrument};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[instrument]
fn foo(ans: i32) {
    // with args
    // INFO foo{ans=42}: tracing_span: in foo
    info!("in foo");
}


async fn inner_function_block() -> i32 {
    println!("test executing...");
    info!("info from test");
    1
}
#[instrument]
async fn function_block() -> i32{
    let result = inner_function_block().await;
    info!(result, "this is a word");
    result
}

fn main() {
    tracing_subscriber::registry().with(fmt::layer()).init();
    foo(42);
    block_on(function_block());
}
