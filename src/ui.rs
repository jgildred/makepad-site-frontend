use crate::app::{State};
use makepad_widgets::*;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;
    
    IMG_A = dep("crate://self/resources/neom-THlO6Mkf5uI-unsplash.jpg")
    IMG_PROFILE_A = dep("crate://self/resources/profile_1.jpg")
    IMG_PROFILE_B = dep("crate://self/resources/profile_2.jpg")
    LOGO = dep("crate://self/resources/scramble_logo.svg")
    ICO_FAV = dep("crate://self/resources/icon_favorite.svg")
    ICO_COMMENT = dep("crate://self/resources/icon_comment.svg")
    ICO_REPLY = dep("crate://self/resources/icon_reply.svg")
    ICO_HOME = dep("crate://self/resources/icon_home.svg")
    ICO_FIND = dep("crate://self/resources/icon_find.svg")
    ICO_LIKES = dep("crate://self/resources/icon_likes.svg")
    ICO_USER = dep("crate://self/resources/icon_user.svg")
    ICO_ADD = dep("crate://self/resources/icon_add.svg")
        
    FONT_SIZE_SUB = 9.5
    FONT_SIZE_P = 12.5
        
    TEXT_SUB = {
        font_size: (FONT_SIZE_SUB),
        font: {path: dep("crate://makepad-widgets/resources/GoNotoKurrent-Regular.ttf")}
    }
        
    TEXT_P = {
        font_size: (FONT_SIZE_P),
        height_factor: 1.65,
        font: {path: dep("crate://makepad-widgets/resources/GoNotoKurrent-Regular.ttf")}
    }
        
    COLOR_BG = #xfff8ee
    COLOR_ALERT = #xf88379
    COLOR_BRAND = #xfff8ee
    COLOR_BRAND_HOVER = #xf66
    COLOR_META_TEXT = #xaaa
    COLOR_META = #xccc
    COLOR_META_INV = #xfffa
    COLOR_OVERLAY_BG = #x000000d8
    COLOR_DIVIDER = #x00000018
    COLOR_DIVIDER_DARK = #x00000044
    COLOR_PROFILE_CIRCLE = #xfff8ee
    COLOR_P = #x999
        
    FillerY = <View> {width: Fill}
        
    FillerX = <View> {height: Fill}
        
    Logo = <Button> {
        draw_icon: {
            svg_file: (LOGO),
            fn get_color(self) -> vec4 {
                return (COLOR_BRAND)
            }
        }
        icon_walk: {width: 7.5, height: Fit}
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                return sdf.result
            }
        }
        padding: 9.0
        text: ""
    }
        
    NavButton = <Button> {
        draw_text: {
            instance hover: 0.0
            instance pressed: 0.0
            text_style: {
                font_size: 11.0
            }
            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        (COLOR_META_TEXT),
                        (COLOR_BRAND),
                        self.hover
                    ),
                    (COLOR_BRAND_HOVER),
                    self.pressed
                )
            }
        }
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                return sdf.result
            }
        }
        padding: 9.0
        text: "page"
    }    

    NavMenu = <View> {
        width: Fill,
        height: Fit,
        margin: 0.0
        flow: Right,
        padding: 0.0,
        spacing: 25.0,
        align: {x: 0.5, y: 0.5}

        // These will be populated when site config is loaded            
        <NavButton> {}
        <NavButton> {}
        <NavButton> {}
        <NavButton> {}
    }
        
    Header = <RoundedYView> {
        width: Fill,
        height: 70
        flow: Right,
        padding: 10.0,
        spacing: 10.0
        draw_bg: {color: (COLOR_OVERLAY_BG), inset: vec4(-0.5, -0.5, -1.0, 0.0), radius: vec2(0.5, 4.5)}
                
        <Logo> {
            height: Fit,
            width: Fill,
            margin: {top: 0.0}
            icon_walk: {width: Fit, height: 27.0}
        }   

        <NavMenu> {}       
    }

    Alert = <RoundedYView> {
        width: Fill,
        height: 40,
        flow: Right,
        padding: 10.0,
        spacing: 10.0

        draw_bg: {color: (COLOR_ALERT), inset: vec4(-0.5, 0.0, -1.0, -1.0), radius: vec2(4.5, 0.5)}
        text = <Label> {
            width: Fill,
            height: Fit
            draw_text: {
                wrap: Word,
                text_style: <TEXT_P> {},
                color: (COLOR_P)
            }
            text: "no alert"
        }
    }
        
    LineH = <RoundedView> {
        width: Fill,
        height: 2,
        margin: 0.0
        padding: 0.0,
        spacing: 0.0
        draw_bg: {color: (COLOR_DIVIDER)}
    }
        
    TextSection = <View> {
        width: Fill,
        height: Fit
        flow: Right,
        padding: 0.0,
        spacing: 0.0,
        text = <Label> {
            width: Fill,
            height: Fit
            draw_text: {
                wrap: Word,
                text_style: <TEXT_P> {},
                color: (COLOR_P)
            }
            text: "no text"
        }
    }

    ImageSection = <View> {
        width: Fill,
        height: Fit
        flow: Down,
        padding: 0.0,
        spacing: 0.0
                
        image = <Image> {
            source: (IMG_A),
            //image_scale: 1.0,
            margin: 0,
            width: Fill,
            height: 200
        }
    }

    SpaceSection = <View> {
        width: Fill,
        height: Fit
        flow: Right,
        padding: 0.0,
        spacing: 0.0
    }

    TitleSection = <View> {
        width: Fill,
        height: Fit
        flow: Right,
        padding: 0.0,
        spacing: 0.0,
        text = <Label> {
            width: Fill,
            height: Fit
            draw_text: {
                wrap: Word,
                text_style: <TEXT_P> {},
                color: (COLOR_P)
            }
            text: "no text"
        }
    }

    ImageTextSection = <View> {
        width: Fill,
        height: Fit
        flow: Right,
        padding: 0.0,
        spacing: 0.0
                
        image = <Image> {
            source: (IMG_A),
            //image_scale: 1.0,
            margin: 0,
            width: Fill,
            height: 200
        }
        text = <Label> {
            width: Fill,
            height: Fit
            draw_text: {
                wrap: Word,
                text_style: <TEXT_P> {},
                color: (COLOR_P)
            }
            text: "no text"
        }
    }

    TextImageSection = <View> {
        width: Fill,
        height: Fit
        flow: Right,
        padding: 0.0,
        spacing: 0.0

        text = <Label> {
            width: Fill,
            height: Fit
            draw_text: {
                wrap: Word,
                text_style: <TEXT_P> {},
                color: (COLOR_P)
            }
            text: "no text"
        }
        image = <Image> {
            source: (IMG_A),
            //image_scale: 1.0,
            margin: 0,
            width: Fill,
            height: 200
        }
    }

    ImageGridSection = <View> {}
       
    Page = {{Page}} {
        // This will be populated when page data is loaded 
        sections = <PortalList> {
        }
    }

    /*NewsFeed ={{NewsFeed}}{
        list = <PortalList>{
            TopSpace = <View> {height: 80}
            Post = <Post> {}
            PostImage = <PostImage> {}
            BottomSpace = <View> {height: 100}
        }
    }*/
    
    // This is the top level layout
    Ui = <Window> {
        window: {inner_size: vec2(428, 926)},
        show_bg: true
        draw_bg: {
            fn pixel(self) -> vec4 {
                return (COLOR_BG);
            }
        }
        body = {
            flow: Overlay,
            padding: 0.0
            spacing: 0,
            align: {
                x: 0.0,
                y: 0.0
            },
            
            page = <Page> {}
                                                            
            top = <View> {
                flow: Down
                <Header> {}
                alert_message = <Alert> {}
                <FillerY> {}
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct Page{ 
    #[deref] view:View
}

// features of the page widget
impl Widget for Page{
    fn draw_walk(&mut self, cx:&mut Cx2d, scope:&mut Scope, walk:Walk)->DrawStep{
        while let Some(page_view) =  self.view.draw_walk(cx, scope, walk).step(){
            if let Some(state) = scope.data.get_mut::<State>() {
                let page = state.pages.iter().find(|&p| p.name == state.config.default_page).unwrap();
                if let Some(mut section_views) = page_view.as_portal_list().borrow_mut() {
                    section_views.set_item_range(cx, 0, page.sections.len() - 1);
                    // Build the sectionViews from default page data
                    for section in &page.sections {
                        if let Some(section_view_id) = section_views.next_visible_item(cx) {
                            let template = match section.layout.as_str() {
                                "text" => live_id!(TextSection),
                                "image" => live_id!(ImageSection),
                                "title" => live_id!(TitleSection),
                                "space" => live_id!(SpaceSection),
                                "text-image" => live_id!(TextImageSection),
                                "image-text" => live_id!(ImageTextSection),
                                "image-grid" => live_id!(ImageGridSection),
                                _ => live_id!(SpaceSection),
                            };
                            let section_view = section_views.item(cx, section_view_id, template).unwrap();
                            if !section.text.is_empty() {
                                section_view.label(id!(text)).set_text(&section.text);
                            }
                            if !section.image_url.is_empty() {
                                section_view.image(id!(image)).load_image_dep_by_path(cx, &section.image_url)
                                        .unwrap();
                            }
                            section_view.draw_all(cx, &mut Scope::empty());
                        }
                    }
                }
            }
        }
        DrawStep::done()
    }
    fn handle_event(&mut self, cx:&mut Cx, event:&Event, scope:&mut Scope){
        self.view.handle_event(cx, event, scope)
    } 
}
