// NewsArticle이나 Tweet 구조체의 인스턴스에 저장된 데이터를 요약해서 보여주는 미디어 수집 라이브러리를 개발하자.
// NewsArticle 구조체는 story 필드에 특정 지역의 뉴스 콘텐츠를 저장하고, Tweet 구조체는 최대 280 글자의
// 텍스트와 해당 트윗이 새 트윗인지, 리트윗된 트윗인지, 아니면 다른 트윗의 댓글인지를 가리키는 메타데이터를 포함한다.

// 각 타입으로부터 요약 데이터를 추출해야 하므로 각 인스턴스로부터 summarize 메서드를 호출한다.
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}, ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("러스트 언어 공부를 시작했습니다."),
        reply: false,
        retweet: false,
    };

    println!("새 트윗 1개: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("대한민국, 러시아 월드컵 예선에서 독일을 이겼다."),
        location: String::from("카잔 아레나, 러시아"),
        author: String::from("위키백과"),
        content: String::from("2018년 6월 27일 러시아 카잔의 카잔 아레나에서 열린 2018년 월드컵 F조 3차전 경기에서 대한민국이 독일에 2:0 승리를 거뒀다."),
    };

    println!("새로운 기사: {}", article.summarize());
}
