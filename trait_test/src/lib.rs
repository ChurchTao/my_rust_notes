pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline:String,
    pub location:String,
    pub author:String,
    pub content:String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("head line {}, author {}, content {}",self.headline,self.author,self.content)
    }
}

pub struct Tweet {
    pub username:String,
    pub content:String,
    pub reply:String,
    pub retweet:String
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("retweet {}, author {}, content {}",self.retweet,self.username,self.content)
    }
}