#![allow(unused)]
use std::{any::Any, fmt::Debug};

use crate::state::*;
use makepad_widgets::*;

// UI design

live_design!(
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;
    
    // reference resources here that will be pulled into the app binary
    //ICO_USER = dep("crate://self/resources/icon_user.svg")

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
    COLOR_ALERT_TEXT = #xfff8ee

    BG_COLOR = #3

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
    COLOR_BRAND = #xf88
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
    
    IconButton = <Button> {
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
        draw_icon: {
            svg_file: (ICO_FAV),
            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        (COLOR_META),
                        (COLOR_BRAND),
                        self.hover
                    ),
                    (COLOR_BRAND_HOVER),
                    self.pressed
                )
            }
        }
        icon_walk: {width: 7.5, height: Fit, margin: {left: 5.0}}
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                return sdf.result
            }
        }
        padding: 9.0
        text: "1"
    }
    
    Header = <RoundedYView> {
        width: Fill,
        height: 70,
        flow: Right,
        padding: 10.0,
        spacing: 20.0,
        align: {x: 0.5, y: 0.5},
        draw_bg: {color: (COLOR_OVERLAY_BG), inset: vec4(-0.5, -0.5, -1.0, 0.0), radius: vec2(0.5, 4.5)}
        
        <Logo> {
            height: Fit,
            margin: {top: 0.0}
            icon_walk: {width: Fit, height: 27.0}
        }
        <FillerX>{}
        // need to dynamicallly generate these based on pages
        homebtn = <IconButton> {draw_icon: {svg_file: (ICO_HOME)} icon_walk: {width: 30.0, height: Fit}, text: ""}
        findbtn = <IconButton> {draw_icon: {svg_file: (ICO_FIND)} icon_walk: {width: 18.0, height: Fit}, text: ""}
        profilebtn = <IconButton> {draw_icon: {svg_file: (ICO_USER)} icon_walk: {width: 15.0, height: Fit}, text: ""}
    }

    TextSection = <View> {
        width: Fill
        height: Fit
        flow: Right
        padding: 0.0
        spacing: 0.0
        label = <Label> {
            width: Fill
            height: Fit
            draw_text: {
                wrap: Word
                text_style: <TEXT_P> {}
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
            //source: (),
            //image_scale: 1.0,
            margin: 0,
            width: Fill,
            height: 200
        }
    }

    SpaceSection = <View> {
        width: Fill,
        height: Fit,
        flow: Right,
        padding: 0.0,
        spacing: 0.0
    }

    TitleSection = <View> {
        width: Fill,
        height: Fit,
        flow: Right,
        padding: 0.0,
        spacing: 0.0,
        text = <Label> {
            width: Fill,
            height: Fit,
            draw_text: {
                wrap: Word
                text_style: <TEXT_P> {}
                color: (COLOR_P)
            }
            text: "no text"
        }
    }

    ImageTextSection = <View> {
        width: Fill,
        height: Fit,
        flow: Right,
        padding: 0.0,
        spacing: 0.0,
        image = <Image> {
            source: (IMG_A),
            //image_scale: 1.0,
            margin: 0,
            width: Fill,
            height: 200
        }
        text = <Label> {
            width: Fill,
            height: Fit,
            draw_text: {
                wrap: Word,
                text_style: <TEXT_P> {},
                color: (COLOR_P),
            },
            text: "no text"
        }
    }

    TextImageSection = <View> {
        width: Fill,
        height: Fit,
        flow: Right,
        padding: 0.0,
        spacing: 0.0,
        text = <Label> {
            width: Fill,
            height: Fit,
            draw_text: {
                wrap: Word,
                text_style: <TEXT_P> {},
                color: (COLOR_P),
            },
            text: "no text"
        },
        image = <Image> {
            source: (IMG_A),
            //image_scale: 1.0,
            margin: 0,
            width: Fill,
            height: 200,
        },
    }

    ImageGridSection = <View> {}

    AlertMessage = <RoundedYView> {
        width: Fill,
        height: 40
        flow: Right,
        padding: 10.0,
        align: {x: 0.5, y: 0.5},
        draw_bg: {color: (COLOR_OVERLAY_BG), inset: vec4(-0.5, -0.5, -1.0, 0.0), radius: vec2(0.5, 4.5)},
        text = <Label> {
            width: Fill,
            height: Fit,
            draw_text: {
                wrap: Word,
                text_style: <TEXT_P> {},
                color: (COLOR_P),
            },
            text: "no text"
        }
    }

    Ui = {{Ui}} {
        window: {inner_size: vec2(428, 926)},
        show_bg: true
        draw_bg: {
            fn pixel(self) -> vec4 {
                return (COLOR_BG);
            }
        }
        body = <View> {
            flow: Overlay,
            padding: 0.0
            spacing: 0,
            align: {
                x: 0.0,
                y: 0.0
            },
            
            list = <PortalList> {
                text_section = <TextSection> {}
                image_section = <ImageSection> {}
                text_image_section = <TextImageSection> {}
                image_text_section = <ImageTextSection> {}
                title_section = <TitleSection> {}
                space_section = <SpaceSection> {}
                image_grid_section = <ImageGridSection> {}
            }
            
            top = <View> {
                flow: Down
                <Header> {}
                <FillerY> {}
                alert_message = <AlertMessage> {}
            }
        }
    }
);

#[derive(Live, LiveHook, Widget)]
pub struct Ui {
    #[deref]
    deref: Window,
}

// widget logic

impl Widget for Ui {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.deref.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(widget) = self.deref.draw_walk(cx, scope, walk).step() {
            let pages = scope.data.get::<State>().unwrap().pages.clone();
            if let Some(page) = pages.iter().find(|&p| p.name ==  scope.data.get::<State>().unwrap().current_page.as_str()) {
                let range_end = page.sections.len();
                if let Some(mut list) = widget.as_portal_list().borrow_mut() {
                    list.set_item_range(cx, 0, range_end);
                    while let Some(index) = list.next_visible_item(cx) {
                        if index < range_end {
                            let template = match page.sections[index].layout.as_str() {
                                "text" => live_id!(text_section),
                                "image" => live_id!(image_section),
                                "title" => live_id!(title_section),
                                "text-image" => live_id!(text_image_section),
                                "image-text" => live_id!(image_text_section),
                                "image-grid" => live_id!(image_grid_section),
                                _ => live_id!(space_section),
                            };
                            let item = list.item(cx, index, template).unwrap();
                            if page.sections[index].text.as_ref().is_some_and(|s| !s.is_empty()) {
                                item.label(id!(label)).set_text(page.sections[index].text.as_ref().unwrap());
                            }
                            if [live_id!(image_section),live_id!(text_image_section),live_id!(image_text_section)].contains(&template) {
                                if page.sections[index].image_loaded.is_none() || !page.sections[index].image_loaded.unwrap() {
                                    let id = LiveId::from_str_with_lut(index.to_string().as_str()).unwrap();
                                    scope.data.get::<State>().unwrap().update_image(cx, id, page.sections[index].image_url.as_ref().unwrap().to_string());
                                    scope.data.get_mut::<State>().unwrap().set_image_as_loaded(page.name.clone(), index);
                                }
                                else {
                                    //dbg!(page.sections[index].image_loaded.unwrap());
                                }
                            }
                            item.draw_all(cx, scope);
                        }
                    }
                }
            }
        }
        let label = self.label(id!(body.top.alert_message.text));
        if let OsType::Web(params) = cx.os_type(){
            let msg = format!("{}", params.pathname);
            label.set_text_and_redraw(cx, &msg);
        } 
        DrawStep::done()
    }
}

impl WidgetMatchEvent for Ui {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let state = scope.data.get_mut::<State>().unwrap();
        let items_with_actions = self
            .deref
            .portal_list(id!(list))
            .items_with_actions(actions);

        let btn_clicked = items_with_actions.iter().find(|(_index, widget)| {
            match widget.as_button().borrow_mut() {
                Some(btn) => btn.clicked(actions),
                None => false,
            }
        });

        if let Some((_index, _widget)) = btn_clicked {
            // do something
        }
    }
}
