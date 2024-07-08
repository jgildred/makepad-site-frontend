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
        // Load the home page data (eventually need to add page routes)
        self.state.load_page(_cx, "home".to_string());
    }
    fn handle_network_responses(&mut self, cx: &mut Cx, responses:&NetworkResponsesEvent ){
        for event in responses{
            match &event.response {
                NetworkResponse::HttpResponse(response) => {
                    match event.request_id {
                        live_id!(LoadPage) => {
                            let label = self.ui.label(id!(message_label));
                            if response.status_code == 200 {
                                let chat_response = response.get_json_body::<ChatResponse>().unwrap();
                                let assistant_message = chat_response.choices[0].message.content.clone();
                                
                                self.conversation_history.push(Message {
                                    content: assistant_message,
                                    role: "assistant".to_string()
                                });
                                
                                self.update_message_label(cx);
                            } else {
                                label.set_text_and_redraw(cx, "Failed to get page data.");
                            }
                            label.redraw(cx);
                        },
                        _ => (),
                    }
                }
                NetworkResponse::HttpRequestError(error) => {
                    let label = self.ui.label(id!(message_label));
                    label.set_text_and_redraw(cx, &format!("Failed to connect with server {:?}", error));
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

pub enum SectionLayout {
    Text, Image, Space, Title, TextImageL, TextImageR
}
pub struct Section {
    pub layout: SectionLayout,
    pub text: String,
    pub image_url: String,
}

pub struct Page {
    pub name: String,
    pub sections: Vec<Section>,
}

#[derive(Default)]
pub struct State {
    pub pages: Vec<Page>
}

impl State {
    fn load_config(&mut self, cx: &mut Cx) {
        let completion_url ="/page_data/config.json".to_string();
        let request_id = live_id!(LoadConfig);
        let request = HttpRequest::new(completion_url, HttpMethod::GET);
        cx.http_request(request_id, request);
    }
    fn load_page(&mut self, cx: &mut Cx, page_name: String) {
        let completion_url = format!("/page_data/{}.json", page_name);
        let request_id = live_id!(LoadPage);
        let request = HttpRequest::new(completion_url, HttpMethod::GET);
        cx.http_request(request_id, request);
    }
    fn update_site_message(&mut self, cx: &mut Cx) {
        let label = self.ui.label(id!(site_message_label));
        let mut conversation_text = String::new();

        for message in &self.conversation_history {
            let role_label = if message.role == "user" { "User:" } else { "Assistant:" };
            conversation_text.push_str(&format!("{}\n{}\n\n", role_label, message.content));
        }

        label.set_text_and_redraw(cx, &conversation_text);
    }
}

app_main!(App);