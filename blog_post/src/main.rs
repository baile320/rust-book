use blog_post::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.add_text("And some other junk too"); // post is in review status so this shouldnt get added

    post.approve();
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
