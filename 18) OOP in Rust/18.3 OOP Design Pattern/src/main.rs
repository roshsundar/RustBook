/* Summary:
The state pattern is a common OOP design pattern, we define a set of states a value can have internally, and the value's
behavior changes based on its state.
Each state is represented by a trait object. Each one is responsible for its own behavior and for when it should change state.

To demonstate this, we will use the example of a blog post - which will go from "draft" to "review" to "published".
*/

mod alternate_post;

fn main() {
    // One way of impl'ing the state pattern (in lib.rs)
    {
        use blog::Post;

        let mut post = Post::new(); // post starts in the draft state.

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content()); // post can't give content in the draft state.

        post.request_review(); // Set post to review state.
        assert_eq!("", post.content()); // post can't give content in the draft state.

        post.approve(); // Set post to published state.
        assert_eq!("I ate a salad for lunch today", post.content()); // post content can be accessed now.
    }

    // Another way of impl'ing the state pattern (in alternate_post.rs)
    {
        use crate::alternate_post::Post;

        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");

        let post = post.request_review();

        let post = post.approve();

        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
