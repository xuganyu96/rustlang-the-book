/** A type's behavior is the collection of the methods that can be called on
 * that type.
 * Trait definitions are a way to group method signature together to define a
 * set of behaviors necessary to accomplish a task
 */
pub trait Summary {
    /** Each type that implements the trait "Summary" needs to provide custom
     * implementation of the behavior "summarize". The compiler will enforce
     * the function signature to be the same
     */
    fn summarize_author(&self) -> String;

    /** One can provide default implementation in the trait definition.
     * Default implementation can call other methods in the same trait, even
     * if the other method doesn't have default implementation
     */
    fn summarize(&self) -> String {
        format!("{}: Read more...", &self.summarize_author())
    }
}

/* Recall that fields of a struct are private by default; variants of an enum
 * are public by default
 */
pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub location: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {} ({})", &self.headline, &self.author, &self.location)
    }

    fn summarize_author(&self) -> String {
        format!("{}", &self.author)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", &self.username)
    }
}

