use tide::prelude::*;
use tide::Request;

use api_tide::get_users;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/users")
        .get(|_| async { Ok(get_users().await.unwrap()) });
    app.listen("127.0.0.1:8084").await?;
    Ok(())
}
