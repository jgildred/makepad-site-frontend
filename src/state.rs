use std::{
    fs, path::{Path, PathBuf}
};
use makepad_widgets::*;
use makepad_micro_serde::*;

const SERVER_BASE_URL: &str = "http://127.0.0.1:3000";

#[derive(Default)]
pub struct State {
    pub config: SiteConfig,
    pub pages: Vec<Page>,
    pub current_page: String,
}

impl State {
    pub fn set_image_as_loaded(&mut self, page_name: String, section_number: usize) {
        if let Some(page) = self.pages.iter_mut().find(|p| p.name == page_name) {
            page.sections[section_number].image_loaded = Some(true);
        }
    }
    pub fn update_image(&self, cx: &mut Cx, live_id: LiveId, image_url: String) {
        let completion_url = format!("{}{}", SERVER_BASE_URL.to_string(), image_url.to_ascii_lowercase());
        let request_id = live_id!(LoadImage);
        let mut request = HttpRequest::new(completion_url, HttpMethod::GET);
        request.set_metadata_id(live_id);
        log!("sent: {}", &request.url);
        cx.http_request(request_id, request);
    }
    pub fn load_config(&mut self, cx: &mut Cx) {
        let completion_url = format!("{}/makepad_site_frontend/resources/page_data/config.json", SERVER_BASE_URL.to_string());
        let request_id = live_id!(LoadConfig);
        let request = HttpRequest::new(completion_url, HttpMethod::GET);
        log!("sent: {}", &request.url);
        cx.http_request(request_id, request);
    }
    pub fn load_page(&mut self, cx: &mut Cx, page_name: &String) {
        let completion_url = format!("{}/makepad_site_frontend/resources/page_data/page_{}.json", SERVER_BASE_URL.to_string(), page_name.to_ascii_lowercase());
        let request_id = live_id!(LoadPage);
        let request = HttpRequest::new(completion_url, HttpMethod::GET);
        //log!("sent: {}", &request.url);
        cx.http_request(request_id, request);
    }
}


#[derive(Default, SerJson, DeJson, Debug)]
pub struct SiteConfig {
    pub page_order: Vec<String>,
    pub default_page: String,
}

#[derive(SerJson, DeJson, Clone, Debug)]
pub struct Section {
    pub layout: String,
    pub padding: Option<f32>,
    pub text: Option<String>,
    pub image_url: Option<String>,
    pub image_loaded: Option<bool>,
}

#[derive(SerJson, DeJson, Clone, Debug)]
pub struct Page {
    pub name: String,
    pub sections: Vec<Section>,
}
