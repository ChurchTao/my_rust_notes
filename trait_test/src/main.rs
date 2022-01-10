use trait_test::NewsArticle;
use trait_test::Summary;
use trait_test::Tweet;

fn main() {
    let tweet = Tweet {
        username: "tao jiacheng".to_string(),
        content: "my name is niupi".to_string(),
        reply: "reppppppply".to_string(),
        retweet: "retwwoor".to_string(),
    };

    let news_article = NewsArticle {
        headline: "tao jiacheng".to_string(),
        content: "my name is niupi".to_string(),
        author: "reppppppply".to_string(),
        location: "retwwoor".to_string(),
    };

    let summary = tweet.summarize();
    println!("summary ={}", summary);
    let summary = news_article.summarize();
    println!("summary ={}", summary);
}
