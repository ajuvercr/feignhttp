use feignhttp::{feign, Feign};

const GITHUB_URL: &str = "https://api.github.com";

#[derive(Feign)]
#[feign(headers = "Accept: {accept}")]
struct Github {
    #[feign_path("owner")]
    user: &'static str,
    #[feign_path]
    repo: &'static str,
    #[param]
    accept: &'static str,
}

#[feign(url = GITHUB_URL)]
impl Github {
    #[get]
    async fn home(&self) -> feignhttp::Result<String> {}

    #[get("/repos/{owner}/{repo}", headers = "accept: application/json")]
    async fn repository(&self) -> feignhttp::Result<String> {}

    #[get(path = "/repos/{owner}/{repo}/contributors")]
    async fn contributors(&self, #[query] page: u32) -> feignhttp::Result<String> {}

    #[get("/repos/{owner}/{repo}/commits")]
    async fn commits(
        &self,
        #[header] accept: &str,
        #[query] page: u32,
        #[query] per_page: u32,
    ) -> feignhttp::Result<String> {
    }

    // Structure method still send request.
    #[get(path = "/repos/{owner}/{repo}/languages")]
    async fn languages(&self) -> feignhttp::Result<String> {}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dxx_github = Github {
        user: "dxx",
        repo: "feignhttp",
        accept: "*/*",
    };

    let r = dxx_github.home().await?;
    println!("github result: {}\n", r);

    let r = dxx_github.repository().await?;
    println!("repository result: {}\n", r);

    let r = dxx_github.contributors(1).await?;
    println!("contributors result: {}\n", r);

    let r = dxx_github
        .commits("application/vnd.github.v3+json", 1, 1)
        .await?;
    println!("commits result: {}\n", r);

    let r = dxx_github.languages().await?;
    println!("languages result: {}\n", r);

    Ok(())
}
