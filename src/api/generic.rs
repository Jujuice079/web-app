use serde::de::DeserializeOwned;
use async_trait::async_trait;
use crate::prelude::*;
use super::*;

#[async_trait]
pub trait CachedData: DeserializeOwned + Serialize {
    fn storage_key() -> &'static str;
    fn endpoint() -> &'static str;
    fn cache_duration() -> u64;
    fn force_reload(&self) -> bool { false }
    fn on_load(result: Result<Self, ApiError>, app_link: Scope<App>);

    fn save(&self) {
        let now = (js_sys::Date::new_0().get_time() / 1000.0) as i64;
        let storage_key = Self::storage_key();
        let local_storage = window().local_storage().unwrap().unwrap();
        let _ = local_storage.set(&format!("last_updated_{storage_key}"), &now.to_string());
        let _ = local_storage.set(&format!("cached_{storage_key}"), &serde_json::to_string(self).unwrap());    
    }

    fn init(app_link: Scope<App>) -> Option<Self> {
        let now = (js_sys::Date::new_0().get_time() / 1000.0) as i64;

        // Get cached
        let mut default = None;
        if let Some((last_updated, cached)) = load_cached::<Self>() {
            if last_updated > now - Self::cache_duration() as i64 && !cached.force_reload() {
                return Some(cached);
            }
            default = Some(cached);
        }
    
        // Update from server
        wasm_bindgen_futures::spawn_local(async move {
            let result = load::<Self>(now).await;
            Self::on_load(result, app_link);
        });
    
        default
    }

    fn refresh(app_link: Scope<App>) {
        let now = (js_sys::Date::new_0().get_time() / 1000.0) as i64;
        wasm_bindgen_futures::spawn_local(async move {
            let result = load::<Self>(now).await;
            Self::on_load(result, app_link);
        });
    }
}

fn load_cached<T: CachedData>() -> Option<(i64, T)> {
    let local_storage = window().local_storage().unwrap().unwrap();
    let storage_key = T::storage_key();

    let Ok(Some(Ok(last_updated))) = local_storage.get(&format!("last_updated_{storage_key}")).map(|v| v.map(|v| v.parse())) else { return None };
    let Ok(Some(cached_str)) = local_storage.get(&format!("cached_{storage_key}")) else { return None };
    let Ok(cached) = serde_json::from_str::<T>(&cached_str) else { return None };

    Some((last_updated, cached))
}

async fn load<T: CachedData>(now: i64) -> Result<T, ApiError> {
    let (api_key, counter) = get_login_info();
    let storage_key = T::storage_key();

    let request = Request::new_with_str(T::endpoint())?;
    request.headers().set(
        "Api-Key",
        &format!("{}-{}-{}", api_key, counter, gen_code(api_key, counter)),
    )?;

    let resp = JsFuture::from(window().fetch_with_request(&request)).await?;
    let resp: web_sys::Response = resp.dyn_into()?;
    let json = JsFuture::from(resp.json()?).await?;

    if resp.status() == 400 || resp.status() == 500 {
        let error: KnownApiError = match serde_wasm_bindgen::from_value(json) {
            Ok(error) => error,
            _ => return Err(ApiError::Unknown(JsValue::from("Invalid JSON of error"))),
        };
        return Err(error.into());
    }

    let value: T = match serde_wasm_bindgen::from_value(json) {
        Ok(value) => value,
        _ => return Err(ApiError::Unknown(JsValue::from("Invalid JSON"))),
    };
    value.save();

    Ok(value)
}

impl CachedData for Vec<RawEvent> {
    fn storage_key() ->  &'static str { "events" }
    fn endpoint() ->  &'static str { "/api/schedule" }
    fn cache_duration() -> u64 { 3600*2 }
    fn force_reload(&self) -> bool { self.is_empty() }
    fn on_load(result: Result<Self, ApiError>, app_link: Scope<App>) {
        match result {
            Ok(mut events) => {
                events.sort_by_key(|e| e.start_unixtime);
                app_link.send_message(AppMsg::ScheduleSuccess(events));
            },
            Err(e) => app_link.send_message(AppMsg::ScheduleFailure(e)),
        }
    }
}

impl CachedData for Vec<AnnouncementDesc> {
    fn storage_key() ->  &'static str { "announcements" }
    fn endpoint() ->  &'static str { "/api/announcements" }
    fn cache_duration() -> u64 { 3600 }
    fn on_load(result: Result<Self, ApiError>, app_link: Scope<App>) {
        match result {
            Ok(announcements) => app_link.send_message(AppMsg::AnnouncementsSuccess(announcements)),
            Err(e) => app_link.send_message(AppMsg::ApiFailure(e)),
        }
    }
}

impl CachedData for Vec<GroupDesc> {
    fn storage_key() ->  &'static str { "groups" }
    fn endpoint() ->  &'static str { "/config/groups.json" }
    fn cache_duration() -> u64 { 3600*2 }
    fn force_reload(&self) -> bool { self.is_empty() }
    fn on_load(result: Result<Self, ApiError>, app_link: Scope<App>) {
        match result {
            Ok(groups) => app_link.send_message(AppMsg::GroupsSuccess(groups)),
            Err(e) => sentry_report(&e),
        }
    }
}

impl CachedData for UserInfo {
    fn storage_key() ->  &'static str { "groups" }
    fn endpoint() ->  &'static str { "/config/groups.json" }
    fn cache_duration() -> u64 { 3600*6 }
    fn on_load(result: Result<Self, ApiError>, app_link: Scope<App>) {
        match result {
            Ok(user_info) => app_link.send_message(AppMsg::UserInfoSuccess(user_info)),
            Err(e) => app_link.send_message(AppMsg::ApiFailure(e)),
        }
    }
}
