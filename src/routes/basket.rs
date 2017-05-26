use rocket_contrib::Template;
use rocket::State;

use model::{AuthUser, Basket};
use context::Context;
use db::Db;


#[get("/<username>/<basket>", rank = 10)]
pub fn index(
    username: &str,
    basket: &str,
    auth_user: Option<AuthUser>,
    db: State<Db>,
) -> Option<Template> {
    handler(username, basket, auth_user, db, None)
}

fn handler(
    username: &str,
    basket: &str,
    auth_user: Option<AuthUser>,
    db: State<Db>,
    facade: Option<&str>,
) -> Option<Template> {
    Basket::load(basket, username, auth_user.as_ref(), &db)
        .map(|basket| {
            // TODO: load facade

            // #[derive(Debug, Clone, Serialize)]
            // struct BasketHeaderContext<'a> {
            //     owner: &'a str,
            //     name: &'a str,
            //     description: Option<&'a str>,
            // }

            // let context = Context {
            //     auth_user,
            //     content: Some(BasketHeaderContext {
            //         owner: basket.owner(),
            //         name: basket.name(),
            //         description: basket.description(),
            //     }),
            //     .. Context::default()
            // };

            let active_facade = facade.unwrap_or("settings");
            let context = Context {
                auth_user,
                content: Some(adhoc! {
                    owner: &'a str = basket.owner(),
                    name: &'a str = basket.name(),
                    description: Option<&'a str> = basket.description(),
                    basket_url: String = basket.url(),
                    facade_bar: String = facade_bar(&basket, active_facade, &db),
                }),
                .. Context::default()
            };

            let template = "basket/settings";
            Template::render(template, &context)
        })
}

fn facade_bar(basket: &Basket, active_facade: &str, _db: &Db) -> String {
    use std::fmt::Write;

    let mut s = String::new();

    let facades = [
        ("foo", "Foo", ""),
        ("bar", "Bar", ""),
        ("baz", "Baz", ""),
        ("settings", "Settings", "float-right"),
    ];

    for &(id, name, classes) in &facades {
        write!(
            s,
            r#"<li class="{} {}" ><a href="{}/{}">{}</a></li>"#,
            if active_facade == id { "active" } else { "" },
            classes,
            basket.url(),
            id,
            name,
        ).unwrap();
    }

    s
}
