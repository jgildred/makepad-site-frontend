#![allow(unused)]

use makepad_widgets::*;
use makepad_micro_serde::*;

const ASSETS_PATH: &str = "/assets";
const DATA_PATH: &str = "/data";
const ROUTS_PATH: &str = "/r";

#[derive(Default, Clone, Debug)]
pub struct State {
    pub config: SiteConfig,
    pub pages: Vec<Page>,
    pub current_page: String,
    pub host: String,
    pub logo_image_loaded: bool,
}

impl State {
    pub fn fetch_image(cx: &mut Cx, live_id: LiveId, image_url: &String, host: &String) {
        let mut url = image_url.clone().to_ascii_lowercase();
        if url.is_empty() {
            log!("An image url is missing in page section {}.", live_id.to_string());
            return;
        }
        // make sure the url starts with '/'
        if url.chars().nth(0).unwrap() != '/' {
            url = format!("/{}", url);
        }
        let completion_url = format!("{}{}{}", host, ASSETS_PATH.to_string(), url);
        let request_id = live_id!(LoadImage);
        let mut request = HttpRequest::new(completion_url, HttpMethod::GET);
        request.set_metadata_id(live_id);
        log!("fetching: {}", &request.url);
        cx.http_request(request_id, request);
    }
    pub fn load_config(&mut self, cx: &mut Cx) {
        let completion_url = format!("{}{}/config.json", self.host, DATA_PATH.to_string());
        let request_id = live_id!(LoadConfig);
        let request = HttpRequest::new(completion_url, HttpMethod::GET);
        log!("fetching: {}", &request.url);
        cx.http_request(request_id, request);
    }
    pub fn load_page(&mut self, cx: &mut Cx, page_name: &String) {
        let completion_url = format!("{}{}/page_{}.json", self.host, DATA_PATH.to_string(), page_name.to_ascii_lowercase());
        let request_id = live_id!(LoadPage);
        let request = HttpRequest::new(completion_url, HttpMethod::GET);
        log!("fetching: {}", &request.url);
        cx.http_request(request_id, request);
    }
}

#[derive(Default, SerJson, DeJson, Clone, Debug)]
pub struct SiteConfig {
    pub site_name: String,
    pub logo_image_url: Option<String>,
    pub page_order: Vec<String>,
    pub default_page: Option<String>,
}

#[derive(Default, SerJson, DeJson, Clone, Debug)]
pub struct Section {
    pub layout: String,
    pub padding: Option<f32>,
    pub text: Option<String>,
    pub image_url: Option<String>,
    pub image_cache: Option<Vec<u8>>,
}

#[derive(Default, SerJson, DeJson, Clone, Debug)]
pub struct Page {
    pub name: String,
    pub sections: Vec<Section>,
}