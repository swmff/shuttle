use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use crate::db::{self, AppData, Log, UserMetadata, UserState};

use super::base;
use askama::Template;

#[derive(Default, PartialEq, serde::Deserialize)]
pub struct CallbackQueryProps {
    pub callback: String, // redirect here after finish
}

#[derive(Template)]
#[template(path = "auth/login.html")]
struct LoginTemplate {
    callback: String,
    // required fields (super::base)
    auth_state: bool,
    site_name: String,
    body_embed: String,
}

#[derive(Template)]
#[template(path = "auth/register.html")]
struct RegisterTemplate {
    callback: String,
    invite_code_required: bool,
    // required fields (super::base)
    auth_state: bool,
    site_name: String,
    body_embed: String,
}

#[derive(Template)]
#[template(path = "auth/login_secondary_token.html")]
struct LoginSecondaryTokenTemplate {
    callback: String,
    // required fields (super::base)
    auth_state: bool,
    site_name: String,
    body_embed: String,
}

#[derive(Default, PartialEq, serde::Deserialize)]
pub struct QueryProps {
    pub edit: Option<bool>,
    pub offset: Option<i32>,
}

#[derive(Template)]
#[template(path = "auth/followers.html")]
struct FollowersTemplate {
    followers: Vec<Log>,
    user: UserState<UserMetadata>,
    offset: i32,
    // required fields (super::base)
    auth_state: bool,
    site_name: String,
    body_embed: String,
}

#[derive(Template)]
#[template(path = "auth/following.html")]
struct FollowingTemplate {
    following: Vec<Log>,
    user: UserState<UserMetadata>,
    offset: i32,
    // required fields (super::base)
    auth_state: bool,
    site_name: String,
    body_embed: String,
}

#[derive(Default, PartialEq, serde::Deserialize)]
pub struct FollowersQueryProps {
    pub offset: Option<i32>,
}

#[derive(Template)]
#[template(path = "auth/user_settings.html")]
struct SettingsTemplate {
    profile: UserState<UserMetadata>,
    metadata: String,
    // required fields (super::base)
    auth_state: bool,
    site_name: String,
    body_embed: String,
}

#[get("/flow/auth/register")]
/// Available at "/flow/auth/register"
/// Still renders even if `REGISTRATION_DISABLED` is present
pub async fn register_request(
    req: HttpRequest,
    info: web::Query<CallbackQueryProps>,
) -> impl Responder {
    let invite_codes = crate::config::get_var("INVITE_CODES");

    // ...
    let base = base::get_base_values(req.cookie("__Secure-Token").is_some());
    return HttpResponse::Ok()
        .append_header(("Content-Type", "text/html"))
        .body(
            RegisterTemplate {
                callback: info.callback.clone(),
                invite_code_required: invite_codes.is_some(),
                // required fields
                auth_state: base.auth_state,
                site_name: base.site_name,
                body_embed: base.body_embed,
            }
            .render()
            .unwrap(),
        );
}

#[get("/flow/auth/login")]
/// Available at "/flow/auth/login"
pub async fn login_request(
    req: HttpRequest,
    info: web::Query<CallbackQueryProps>,
) -> impl Responder {
    // ...
    let base = base::get_base_values(req.cookie("__Secure-Token").is_some());
    return HttpResponse::Ok()
        .append_header(("Content-Type", "text/html"))
        .body(
            LoginTemplate {
                callback: info.callback.clone(),
                // required fields
                auth_state: base.auth_state,
                site_name: base.site_name,
                body_embed: base.body_embed,
            }
            .render()
            .unwrap(),
        );
}

#[get("/flow/auth/login-st")]
/// Available at "/flow/auth/login-st"
pub async fn login_secondary_token_request(
    req: HttpRequest,
    info: web::Query<CallbackQueryProps>,
) -> impl Responder {
    // ...
    let base = base::get_base_values(req.cookie("__Secure-Token").is_some());
    return HttpResponse::Ok()
        .append_header(("Content-Type", "text/html"))
        .body(
            LoginSecondaryTokenTemplate {
                callback: info.callback.clone(),
                // required fields
                auth_state: base.auth_state,
                site_name: base.site_name,
                body_embed: base.body_embed,
            }
            .render()
            .unwrap(),
        );
}

#[get("/{username:.*}/followers")]
/// Available at "/{username}/followers"
pub async fn followers_request(
    req: HttpRequest,
    data: web::Data<AppData>,
    info: web::Query<FollowersQueryProps>,
) -> impl Responder {
    // get user
    let username: String = req.match_info().get("username").unwrap().to_string();
    let username_c = username.clone();

    let user = data.db.get_user_by_username(username).await;

    if user.is_ok() == false {
        return HttpResponse::NotFound()
            .append_header(("Content-Type", "text/plain"))
            .body("404: Not Found");
    }

    let unwrap = user.ok().unwrap();

    // verify auth status
    let (set_cookie, _, _) = base::check_auth_status(req.clone(), data.clone()).await;

    // ...
    let followers_res: db::DefaultReturn<Option<Vec<db::Log>>> = data
        .db
        .get_user_followers(username_c.clone(), info.offset)
        .await;

    let base = base::get_base_values(req.cookie("__Secure-Token").is_some());
    let props = FollowersTemplate {
        user: unwrap.clone().user,
        followers: followers_res.payload.unwrap(),
        offset: if info.offset.is_some() {
            info.offset.unwrap()
        } else {
            0
        },
        auth_state: base.auth_state,
        site_name: base.site_name,
        body_embed: base.body_embed,
    };

    return HttpResponse::Ok()
        .append_header(("Set-Cookie", set_cookie))
        .append_header(("Content-Type", "text/html"))
        .body(props.render().unwrap());
}

#[get("/{username:.*}/following")]
/// Available at "/{username}/following"
pub async fn following_request(
    req: HttpRequest,
    data: web::Data<AppData>,
    info: web::Query<FollowersQueryProps>,
) -> impl Responder {
    // get user
    let username: String = req.match_info().get("username").unwrap().to_string();
    let username_c = username.clone();

    let user = data.db.get_user_by_username(username).await;

    if user.is_ok() == false {
        return HttpResponse::NotFound()
            .append_header(("Content-Type", "text/plain"))
            .body("404: Not Found");
    }

    let unwrap = user.ok().unwrap();

    // verify auth status
    let (set_cookie, _, _) = base::check_auth_status(req.clone(), data.clone()).await;

    // ...
    let following_res: db::DefaultReturn<Option<Vec<db::Log>>> = data
        .db
        .get_user_following(username_c.clone(), info.offset)
        .await;

    let base = base::get_base_values(req.cookie("__Secure-Token").is_some());
    let props = FollowingTemplate {
        user: unwrap.clone().user,
        following: following_res.payload.unwrap(),
        offset: if info.offset.is_some() {
            info.offset.unwrap()
        } else {
            0
        },
        auth_state: base.auth_state,
        site_name: base.site_name,
        body_embed: base.body_embed,
    };

    return HttpResponse::Ok()
        .append_header(("Set-Cookie", set_cookie))
        .append_header(("Content-Type", "text/html"))
        .body(props.render().unwrap());
}

#[get("/{name:.*}/settings")]
/// Available at "/{name}/settings"
pub async fn user_settings_request(
    req: HttpRequest,
    data: web::Data<db::AppData>,
) -> impl Responder {
    // get user
    let name: String = req.match_info().get("name").unwrap().to_string();
    let profile = match data.db.get_user_by_username(name).await {
        Ok(p) => p,
        Err(e) => {
            return HttpResponse::NotFound().body(e.to_string());
        }
    };

    // verify auth status
    let (set_cookie, token_cookie, token_user) =
        base::check_auth_status(req.clone(), data.clone()).await;

    if token_user.is_none() {
        return HttpResponse::NotAcceptable().body("An account is required to do this");
    }

    // ...
    let user = token_user.unwrap().ok().unwrap();
    let can_view: bool = (user.user.username == profile.user.username)
        | (user
            .level
            .permissions
            .contains(&String::from("ManageUsers")));

    if can_view == false {
        return HttpResponse::NotFound()
            .append_header(("Content-Type", "text/plain"))
            .body("You do not have permission to manage this user's contents.");
    }

    // ...
    let base = base::get_base_values(token_cookie.is_some());
    let props = SettingsTemplate {
        profile: profile.clone().user,
        metadata: serde_json::to_string(&profile.user.metadata)
            .unwrap()
            .replace("/", "\\/"),
        auth_state: base.auth_state,
        site_name: base.site_name,
        body_embed: base.body_embed,
    };

    return HttpResponse::Ok()
        .append_header(("Set-Cookie", set_cookie))
        .append_header(("Content-Type", "text/html"))
        .body(props.render().unwrap());
}
