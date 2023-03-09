use fantoccini::{ClientBuilder, Locator};

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let c = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect("failed to connect to WebDriver");

    c.goto("https://teste350.booksy.com").await?;

    let service_rows = c
        .find_all(Locator::Css("li[data-testid='service']"))
        .await?;
    for row in service_rows {
        let name = row.find(Locator::Css("h3")).await?;
        let price = row
            .find(Locator::Css("[data-testid='service-price']"))
            .await?;
        let duration = row
            .find(Locator::Css("[data-testid='service-duration']"))
            .await?;
        let button = row
            .find(Locator::Css("[data-testid='service-button']"))
            .await?;

        println!(
            "{} - {} - {} - {}",
            name.text().await?,
            price.text().await?,
            duration.text().await?,
            button.text().await?
        );
    }

    c.close().await
}
