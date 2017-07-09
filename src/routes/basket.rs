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

#[get("/<username>/<basket>/<facade>", rank = 10)]
pub fn facade(
    username: &str,
    basket: &str,
    auth_user: Option<AuthUser>,
    db: State<Db>,
    facade: &str,
) -> Option<Template> {
    handler(username, basket, auth_user, db, Some(facade))
}

fn handler(
    username: &str,
    basket: &str,
    auth_user: Option<AuthUser>,
    db: State<Db>,
    facade: Option<&str>,
) -> Option<Template> {
    Basket::load(basket, username, auth_user.as_ref(), &db)
        .and_then(|basket| {
            // TODO: load facade

            let repo = basket.open_repo();
            repo.debug();

            let active_facade = facade.unwrap_or("settings");

            let facade = match active_facade {
                "settings" => json!({
                    "name": "settings",
                    "title": "Settings",
                }),
                "file-browser" => json!({
                    "name": "file-browser",
                    "title": "File browser",
                }),
                _ => return None,
            };

            let context = Context {
                auth_user,
                content: Some(json!({
                    "owner": &basket.owner,
                    "name": basket.name(),
                    "description": basket.description(),
                    "basket_url": basket.url(),
                    "facade_bar": facade_bar(&basket, active_facade, &db),
                    "facade": facade,
                })),
                .. Context::default()
            };

            let template = match active_facade {
                "settings" => "basket/settings",
                _ => "basket/facade",
            };
            Some(Template::render(template, &context))
        })
}

fn facade_bar(basket: &Basket, active_facade: &str, _db: &Db) -> String {
    use std::fmt::Write;

    let mut s = String::new();

    let facades = [
        ("file-browser", "Files", ""),
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
