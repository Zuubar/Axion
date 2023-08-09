mod macros;

pub trait AxionJson {
    fn to_json(&self) -> String;
}

impl_axion_json_for_builtins!(
    i8, u8,
    i16, u16,
    i32, u32,
    i64, u64,
    i128, u128,
    f32, f64,
    isize, usize,
    &str, String
);

impl_axion_json_for_collections!(
    [T],
    Vec<T>
);

#[cfg(test)]
mod tests {
    use super::*;
    use axion_json_derive::AxionJson;

    #[derive(AxionJson, Copy, Clone)]
    struct Post<'a> {
        id: u128,
        title: &'a str,
        content: &'a str,
    }

    impl<'a> Post<'a> {
        fn new() -> Post<'a> {
            Post {
                id: 0,
                title: "Lorem Ipsum",
                content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras sed fringilla erat",
            }
        }
    }

    #[derive(AxionJson, Clone)]
    struct User<'a> {
        id: u64,
        name: String,
        credits: f64,
        posts: Vec<Post<'a>>,
    }

    impl<'a> User<'a> {
        fn new() -> User<'a> {
            User {
                id: 0,
                name: "Tom".to_string(),
                credits: 64f64,
                posts: vec![],
            }
        }

        fn new_with_posts() -> User<'a> {
            User {
                id: 0,
                name: "Tom".to_string(),
                credits: 64f64,
                posts: vec![Post::new(), Post::new()],
            }
        }
    }

    #[test]
    fn test_post_to_json() {
        let post = Post::new();
        let post_to_json = r#"{"id":0,"title":"Lorem Ipsum","content":"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras sed fringilla erat"}"#;

        assert_eq!(post.to_json(), post_to_json);
        assert_eq!([post, post].to_json(), format!("[{},{}]", post_to_json, post_to_json));
        assert_eq!(vec![post, post].to_json(), format!("[{},{}]", post_to_json, post_to_json));
        assert_eq!([post, post].to_json(), vec![post, post].to_json());
    }

    #[test]
    fn test_user_to_json() {
        let user = User::new();
        let user_to_json = r#"{"id":0,"name":"Tom","credits":64.0,"posts":[]}"#;

        assert_eq!(user.to_json(), user_to_json);
        assert_eq!([user.clone(), user.clone()].to_json(), format!("[{},{}]", user_to_json, user_to_json));
        assert_eq!(vec![user.clone(), user.clone()].to_json(), format!("[{},{}]", user_to_json, user_to_json));
        assert_eq!([user.clone(), user.clone()].to_json(), vec![user.clone(), user.clone()].to_json());
    }

    #[test]
    fn test_user_with_post_to_json() {
        let user = User::new_with_posts();
        let user_to_json = r#"{"id":0,"name":"Tom","credits":64.0,"posts":[{"id":0,"title":"Lorem Ipsum","content":"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras sed fringilla erat"},{"id":0,"title":"Lorem Ipsum","content":"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras sed fringilla erat"}]}"#;

        assert_eq!(user.to_json(), user_to_json);
        assert_eq!([user.clone(), user.clone()].to_json(), format!("[{},{}]", user_to_json, user_to_json));
        assert_eq!(vec![user.clone(), user.clone()].to_json(), format!("[{},{}]", user_to_json, user_to_json));
        assert_eq!([user.clone(), user.clone()].to_json(), vec![user.clone(), user.clone()].to_json());
    }
}
