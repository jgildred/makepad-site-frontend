#![allow(unused)]

use std::collections::HashMap;
use std::{any::Any, path::Path};
use crate::state::*;
use makepad_widgets::*;

const DEFAULT_HOST: &str = "http://127.0.0.1:3000";

// convenience for hashmap literals
macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$(($k, $v),)*]))
    }};
}

// allowed section types in page data
static SECTION_LAYOUTS: &'static [&str] = &[
    "text",
    "image",
    "title",
    "text-image",
    "image-text",
    "image-grid",
    "space-section"
];

live_design!(
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import crate::ui::Ui;
    App = {{App}} {
        ui: <Ui> {}
    }
);

#[derive(Live, LiveHook, Default)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    state: State,
}

impl App {
    // convert section layout from page data to live id template
    pub fn get_layout_template(type_name: &str) -> Option<LiveId> {
        if SECTION_LAYOUTS.contains(&type_name) {
            let type_name = format!("{}_section",type_name.replace("-", "_"));
            log!("Getting template for {}", type_name);
            Some(LiveId::from_str_with_lut(type_name.as_str()).unwrap())
        }
        else {
            None
        }
    }
    pub fn flash_alert(&mut self, _cx: &mut Cx, _alert_text: String) {
        log!("Alert: {}", _alert_text);
        let label = self.ui.label(id!(body.top.alert_message.label));
        label.set_text(&_alert_text);
        self.ui.view(id!(body.top.alert_message)).set_visible_and_redraw(_cx, true);
    }
    pub fn select_page(&mut self, cx: &mut Cx, page_name: String) {
        self.state.current_page = page_name;
        log!("Page changed to {}", self.state.current_page);
        let tab_id = LiveId::from_str_with_lut(format!("tab_{}", self.state.current_page).as_str()).unwrap();
        self.ui.dock(id!(pages_dock)).select_tab(cx, tab_id);
    }
    pub fn load_image(cx: &mut Cx, image: ImageRef, data: &Vec<u8>) {
        match image.load_png_from_data(cx, data) {
            Ok(r) => {
                if image.has_texture() {
                    image.redraw(cx);
                    log!("Loaded image: {:?}", image);
                }
                else {
                    log!("Couldn't load image.");
                }
            }
            Err(e) => {
                log!("Image couldn't load: {}", e);
            }
        }
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        log!("{:?}",cx.os_type());
        // set the host dynamically if using wasm, will be overwritten by server config
        self.state.host = if let OsType::Web(params) = cx.os_type(){
             format!("{}", params.host)
        }
        else {
            DEFAULT_HOST.to_string()
        };
        // define allowed section types for pages
        let section_types: HashMap<_,_> = collection!{ 
            "text".to_string() => live_id!(text_section),
            "image".to_string() => live_id!(image_section),
            "title".to_string() => live_id!(title_section),
            "text-image".to_string() => live_id!(text_image_section),
            "image-text".to_string() => live_id!(image_text_section),
            "image-grid".to_string() => live_id!(image_grid_section),
            "space-section".to_string() => live_id!(space_section),
        };
        // hide alert view
        self.ui.view(id!(body.top.alert_message)).set_visible(false);
        // Load the site configuration
        self.state.load_config(cx);
    }
    fn handle_network_responses(&mut self, cx: &mut Cx, responses:&NetworkResponsesEvent ){
        for event in responses{
            match &event.response {
                NetworkResponse::HttpResponse(response) => {
                    match event.request_id {
                        live_id!(LoadConfig) => {
                            if response.status_code == 200 {
                                if let Some(config) = response.get_json_body::<SiteConfig>().ok() {
                                    // set config from server
                                    self.state.config = config.clone();
                                    // check page data
                                    if !config.page_order.is_empty() {
                                        if config.default_page.is_none() 
                                        || config.default_page.unwrap().is_empty() {
                                            self.state.config.default_page = Some(self.state.config.page_order[0].to_string());
                                        }
                                    }
                                    // Set current page
                                    if self.state.config.default_page.is_some() {
                                        self.state.current_page = self.state.config.default_page.clone().unwrap();
                                    }
                                    self.flash_alert(cx, "Config loaded.".to_string());
                                    // Load data for all pages
                                    for page in self.state.config.page_order.clone(){
                                        self.state.load_page(cx, &page);
                                    }
                                }
                                else {
                                    self.flash_alert(cx, "Received bad data for site config.".to_string());
                                }
                            } else {
                                self.flash_alert(cx, "Failed to get site config.".to_string());
                            }
                        },
                        live_id!(LoadPage) => {
                            if response.status_code == 200 {
                                match response.get_json_body::<Page>() {
                                    Ok(page) => {
                                        log!("Loaded page: {}", &page.name);
                                        self.state.pages.push(page);
                                        // if all pages loaded then redraw ui
                                        if self.state.pages.len() == self.state.config.page_order.len() {
                                            log!("Done loading pages.");
                                            self.ui.redraw(cx);
                                            //cx.debug_draw_tree(false, );
                                            self.select_page(cx, self.state.current_page.clone());
                                        }
                                    }
                                    Err(e) => {
                                        self.flash_alert(cx, format!("Received bad data for page: {:?}", e));
                                    }
                                }
                            } else {
                                self.flash_alert(cx, "Failed to get page data.".to_string());
                            }
                        },
                        live_id!(LoadImage) => if let Some(data) = response.get_body() {
                            let image_id = response.metadata_id;
                            log!("received {:?} ({:?} bytes)",image_id.clone(),data.len());
                            if image_id == live_id!(image_logo) {
                                self.state.logo_image_loaded = true;
                                let image = self.ui.image(id!(logo_image));
                                App::load_image(cx, image, &data);
                            }
                            else {
                                // extract the section number from the metadata_id
                                let id_str = image_id.to_string();
                                let parts = id_str.split("_").collect::<Vec<&str>>();
                                if parts.len() == 3 {
                                    if parts[2].len() > 1 && parts[2].chars().nth(0).unwrap() == 's' {
                                        let mut chars = parts[2].chars();
                                        chars.next();
                                        let section_str = chars.as_str();
                                        //log!("section extracted from image response is {}", section);
                                        if let Result::Ok(entry_id) = section_str.parse::<usize>() {
                                            let page_name = parts[1];
                                            //log!("extracted page is {}", page);
                                            let tab_id = LiveId::from_str_with_lut(format!("tab_{}", page_name).as_str()).unwrap();
                                            let tab = self.ui.dock(id!(pages_dock)).item(tab_id);
                                            let list = tab.portal_list(id!(sections));
                                            let mut page_index: usize = 0;
                                            for (i, page) in self.state.pages.iter().enumerate() {
                                                if page.name == page_name {
                                                   page_index = i;
                                                }
                                            }
                                            // don't know why list.item() needs a template to find a list item
                                            // so trying all possible templates
                                            let layout = self.state.pages[page_index].sections[entry_id].layout.clone();
                                            let layout_template = App::get_layout_template(layout.as_str()).unwrap();
                                            if let Some((item, exists)) = list.item_with_existed(cx, entry_id, layout_template) {
                                                if exists {
                                                    let some_data = &Some(data.clone());
                                                    self.state.pages[page_index].sections[entry_id].image_cache = Some(some_data.clone().unwrap());
                                                    let image = item.image(id!(image));
                                                    App::load_image(cx, image, &data);
                                                }
                                                else {
                                                    log!("Received an image for a section with an incorrect template.");
                                                }
                                            }
                                            else {
                                                log!("Received an image for a section with an unused template.");
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        _ => (),
                    }
                }
                NetworkResponse::HttpRequestError(error) => {
                    dbg!(self.state.host.clone());
                    self.flash_alert(cx, format!("Failed to connect with server, {:?}", error));
                }
                _ => ()
            }
        }
     }
     fn handle_actions(&mut self, cx:&mut Cx, actions:&Actions){
        let menu_items_with_actions = self.ui
            .portal_list(id!(nav_menu))
            .items_with_actions(actions);
        let nav_menu_clicked = menu_items_with_actions.iter().find(|(_index, widget)| {
            match widget.as_button().borrow_mut() {
                Some(btn) => btn.clicked(actions),
                None => false,
            }
        });
        // go to page when menu item is clicked
        if let Some((_index, _widget)) = nav_menu_clicked {
            self.select_page(cx, _widget.text());
        }
        // go to default page when logo is clicked
        if self.ui.button(id!(logo.button)).clicked(actions) {
            self.select_page(cx, self.state.config.default_page.clone().unwrap());
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::with_data(&mut self.state));
        self.match_event(cx, event);
    }
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crate::ui::live_design(cx);
    }
}

app_main!(App);
