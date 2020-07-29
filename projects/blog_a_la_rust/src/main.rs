use blog_a_la_rust::Post;


fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    // we add shadowing as each instance produces a new kind of post
    let mut post = post.request_review();
    post.approve();
    post.approve();
    let post = match post.publish() {
        Some(post) => {
            post
        },
        None => panic!("You need more than 1 approval to publish!")
    };
    assert_eq!(
        "I ate a salad for lunch today",
        post.content()
    );
    
}
