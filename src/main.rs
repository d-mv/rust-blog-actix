use dotenv::dotenv;
use std::env;
use blog_actix::Blog;

fn main() -> std::io::Result<()> {
    dotenv().ok();

    env::set_var("RUST_LOG","actix_web=info");
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let app = Blog::new(8998);
    app.run(database_url)
}