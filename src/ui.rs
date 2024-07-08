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

        text: "alert"

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
        text: ""
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
        text: ""
    }

    TextImageLSection = <View> {
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
        text: ""
    }

    TextImageRSection = <View> {
        width: Fill,
        height: Fit
        flow: Right,
        padding: 0.0,
        spacing: 0.0

        text: ""
        image = <Image> {
            source: (IMG_A),
            //image_scale: 1.0,
            margin: 0,
            width: Fill,
            height: 200
        }
    }

    ImageGridSection = <View> {}

    // This will be populated when page data is loaded    
    Page = {{Page}} {}
    
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
                                                            
            <View> {
                flow: Down
                <Header> {}
                <Alert> {}
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
        while let Some(page) =  self.view.draw_walk(cx, scope, walk).step(){
            if let Some(mut list) = page.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, 1000);
                while let Some(item_id) = list.next_visible_item(cx) {
                    let template = match item_id {
                        0 => live_id!(TopSpace),
                        x if x % 5 == 0 => live_id!(PostImage),
                        _ => live_id!(Post)
                    };
                    let item = list.item(cx, item_id, template).unwrap();
                    let text = match item_id % 4 {
                        1 => format!("Message id: {}", item_id),
                        2 => format!("How are you\nItem id: {}", item_id),
                        3 => format!("Item id: {}", item_id),
                        _ => format!("Message 4 id {}", item_id),
                    };
                    item.label(id!(content.text)).set_text(&text);
                    item.draw_all(cx, &mut Scope::empty());
                }
            }
        }
        DrawStep::done()
    }
    fn handle_event(&mut self, cx:&mut Cx, event:&Event, scope:&mut Scope){
        self.view.handle_event(cx, event, scope)
    }
}
