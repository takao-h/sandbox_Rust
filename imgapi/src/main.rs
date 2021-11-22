// use imgurian::commands::{
//     delete_image, favorite_image, generate_access_token, get_account, get_account_image,
//     get_account_images_count, get_image, list_account_images, update_image, upload_image,
// };
use imgapi::opt::Opt;
use imgapi::result::Result;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    match Opt::from_args() {
        Opt::GetImage(input) => get_image(input).await,
    }
}
