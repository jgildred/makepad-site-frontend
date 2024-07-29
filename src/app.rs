//use image_cache::ImageError;
use makepad_widgets::*;
use std::path::Path;
use crate::state::*;

live_design!(
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import crate::ui::Ui;
    App = {{App}} {
        ui: <Ui> {}
    }
);

#[derive(Live, LiveHook)]
struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    state: State,
}

impl App {
    fn flash_alert(&mut self, _cx: &mut Cx, _alert_text: String) {
        // let label = self.ui.label(id!(body.top.alert_message.text));
        // label.set_text_and_redraw(_cx, &_alert_text);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, _cx: &mut Cx) {
        // Load the site configuration
        self.state.load_config(_cx);
    }
    fn handle_network_responses(&mut self, cx: &mut Cx, responses:&NetworkResponsesEvent ){
        for event in responses{
            match &event.response {
                NetworkResponse::HttpResponse(response) => {
                    match event.request_id {
                        live_id!(LoadConfig) => {
                            if response.status_code == 200 {
                                if let Some(config) = response.get_json_body::<SiteConfig>().ok() {
                                    self.state.config = config;
                                    self.flash_alert(cx, "Config loaded.".to_string());
                                    // Load data for all pages
                                    for page in self.state.config.page_order.clone(){
                                        self.state.load_page(cx, &page);
                                    }
                                    // Set current page
                                    self.state.current_page = self.state.config.default_page.clone();
                                }
                                else {
                                    log!("Received bad data for site config.");
                                }
                            } else {
                                log!("Failed to get site config.");
                                self.flash_alert(cx, "Failed to get site config.".to_string());
                            }
                        },
                        live_id!(LoadPage) => {
                            if response.status_code == 200 {
                                match response.get_json_body::<Page>() {
                                    Ok(page) => {
                                        dbg!(&page.name);
                                        self.state.pages.push(page);
                                        self.flash_alert(cx, format!("{} pages loaded.", self.state.pages.len()));
                                        self.ui.portal_list(id!(list)).redraw(cx);
                                    }
                                    Err(e) => {
                                        log!("Received bad data for page: {:?}", e);
                                        self.flash_alert(cx, "Received bad data for page.".to_string());
                                    }
                                }
                            } else {
                                self.flash_alert(cx, "Failed to get page data.".to_string());
                            }
                        },
                        live_id!(LoadImage) => if let Some(data) = response.get_body() {
                            //dbg!(data.len());
                            //dbg!(response.metadata_id);
                            let list = self.ui.portal_list(id!(list));
                            //dbg!(list.item(cx, 0, live_id!(image_section)));
                            let image = list.item(cx, response.metadata_id.to_string().parse::<usize>().unwrap(), live_id!(image_section)).unwrap().image(id!(image));
                            image.load_jpg_from_data(cx, data).unwrap();
                            if image.has_texture() {
                                image.redraw(cx);
                                log!("loaded image {:?}",response.metadata_id);
                            }
                            else {
                                log!("couldn't load image");
                            }
                        },
                        _ => (),
                    }
                }
                NetworkResponse::HttpRequestError(error) => {
                    self.flash_alert(cx, format!("Failed to connect with server, {:?}", error));
                }
                _ => ()
            }
        } 
     }
     fn handle_actions(&mut self, _cx:&mut Cx, actions:&Actions){
        let sections = self.ui.portal_list_set(ids!(list));
        for (item_id, item) in sections.items_with_actions(&actions) {
            if item.button(id!(likes)).clicked(&actions) {
                log!("liked {}", item_id);
            }
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui
            .handle_event(cx, event, &mut Scope::with_data(&mut self.state));
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
