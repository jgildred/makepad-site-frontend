use makepad_micro_serde::*;
use makepad_widgets::*;
   
live_design!(
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import crate::ui::*;
    
    App = {{App}} {
        ui: <Ui> {}
    }
);

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
    #[rust] state: State,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crate::ui::live_design(cx);
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
                                self.state.config = response.get_json_body::<SiteConfig>().unwrap();
                                // Load the default page data (eventually need to add page routes)
                                for page in self.state.config.page_order.clone() {
                                    self.state.load_page(cx, page);
                                }
                            } else {
                                self.flash_alert(cx, "Failed to get site config.".to_string());
                            }
                        },
                        live_id!(LoadPage) => {
                            if response.status_code == 200 {
                                self.state.pages.push(response.get_json_body::<Page>().unwrap());
                            } else {
                                self.flash_alert(cx, "Failed to get page data.".to_string());
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
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

impl App {
    fn flash_alert(&mut self, cx: &mut Cx, alert_text: String) {
        let label = self.ui.label(id!(alert_message));
        label.set_text_and_redraw(cx, &alert_text);
    } 
}

#[derive(Default, SerJson, DeJson)]
pub struct SiteConfig {
    pub page_order: Vec<String>,
    pub default_page: String,
}

#[derive(SerJson, DeJson)]
pub enum SectionLayout {
    Text, Image, Space, Title, TextImageL, TextImageR, ImageGrid
}

#[derive(SerJson, DeJson)]
pub struct Section {
    pub layout: SectionLayout,
    pub padding: i8,
    pub text: String,
    pub image_url: String,
}

#[derive(SerJson, DeJson)]
pub struct Page {
    pub name: String,
    pub sections: Vec<Section>,
}

#[derive(Default)]
pub struct State {
    pub config: SiteConfig,
    pub pages: Vec<Page>
}

impl State {
    fn load_config(&mut self, cx: &mut Cx) {
        let completion_url ="/resources/page_data/config.json".to_string();
        let request_id = live_id!(LoadConfig);
        let request = HttpRequest::new(completion_url, HttpMethod::GET);
        cx.http_request(request_id, request);
    }
    fn load_page(&mut self, cx: &mut Cx, page_name: String) {
        let completion_url = format!("/resources/page_data/page_{}.json", page_name);
        let request_id = live_id!(LoadPage);
        let request = HttpRequest::new(completion_url, HttpMethod::GET);
        cx.http_request(request_id, request);
    }
}

app_main!(App);