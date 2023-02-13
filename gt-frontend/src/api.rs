use lazy_static::lazy_static;

fn api_url(endpoint: &str) -> String {
    let base = web_sys::window().unwrap().origin();
    base + "/api" + endpoint
}

lazy_static! {
    pub static ref EXERCISE_NAME: String = api_url("/exercise/name");
    pub static ref EXERCISE_SET: String = api_url("/exercise/set");
    pub static ref EXERCISE_PR: String = api_url("/exercise/pr");
    pub static ref USER_LOGIN: String = api_url("/user/login");
    pub static ref USER_REGISTER: String = api_url("/user/register");
    pub static ref USER_INFO: String = api_url("/user/info");
    pub static ref USER_INFO_TS: String = api_url("/user/info-ts");
    pub static ref AUTH_CHECK: String = api_url("/auth/check");
    pub static ref MERGE_NAMES: String = api_url("/admin/merge-names");
}
