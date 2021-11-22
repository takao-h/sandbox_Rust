#[derive(Debug, StructOpt)]
#[structopt(about = "Imgur API client.")]
pub enum Opt {
    #[structopt(about = "Get information about an image.")]
    GetImage(GetImageIput),
}


#[derive(Debug, StructOpt)]
pub struct GetImageIput {
    #[structopt(long)]
    pub access_token: Option<String>,
    #[structopt(long)]
    pub client_id: Option<String>,
    pub hash: String
}