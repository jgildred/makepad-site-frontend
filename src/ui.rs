#![allow(unused)]

use std::{any::Any, fmt::Debug};
use crate::state::State;
use crate::app::App;
use makepad_widgets::*;
use tab_bar::TabBarSetWidgetRefExt;

// convenience for hashmap literals
macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$(($k, $v),)*]))
    }};
}

// UI design
live_design!(
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;
    
    // reference resources here that will be pulled into the app binary if needed
    //ICO_USER = dep("crate://self/resources/icon_user.svg")

    FONT_SIZE_SUB = 9.5
    FONT_SIZE_P = 11.5
    
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
    Logo = <View> {
        flow: Overlay,
        padding: 0,
        height: Fill, width: Fill,
        logo_image = <Image> {
            margin: 0,
            width: Fill,
            height: Fill,
        }
        button = <ButtonFlat> {
            height: Fill, width: Fill,
        }
    }
    TextButton = <Button> {
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
        text: "unknown"
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
            //svg_file: (ICO_FAV),
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
        text: "unknown"
    }
    Header = <View> {
        width: Fill,
        height: 70,
        flow: Right,
        padding: 10.0,
        spacing: 20.0,
        align: {x: 0.0, y: 0.5},
        show_bg: true,
        draw_bg: {color: (COLOR_OVERLAY_BG)}
        logo = <Logo> {
            height: 40, width: 40,
            margin: {top: 0.0, left: 50.0}
        }
        <FillerY>{}
        nav = <View> {
            width: Fill,
            height: 40,
            //align: {x: 0, y: 0.5},
            nav_menu = <PortalList> {
                //auto_tail: false,
                width: Fit,
                height: Fill,
                flow: Right,
                nav_menu_button = <TextButton> {}
            }
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
        label = <Label> {
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
        spacing: 0.0,
        image = <Image> {
            margin: 0,
            width: Fill,
            height: 200
        }
    }
    TextImageSection = <View> {
        width: Fill,
        height: Fit,
        flow: Right,
        padding: 10.0,
        spacing: 0.0,
        align: {x: 0.5, y: 0.0},
        label = <Label> {
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
            margin: 0,
            width: Fill,
            height: 100,
        },
    }
    ImageTextSection = <View> {
        width: Fill,
        height: Fit,
        flow: Right,
        padding: 0.0,
        spacing: 0.0,
        align: {x: 0.5, y: 0.0},
        image = <Image> {
            margin: 0,
            width: Fill,
            height: 100
        },
        label = <Label> {
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
    ImageGridSection = <View> {}
    AlertMessage = <View> {
        width: Fill,
        height: 30 // 30
        flow: Right,
        padding: 10.0,
        align: {x: 0.5, y: 0.5},
        show_bg: true,
        draw_bg: {color: (COLOR_OVERLAY_BG)},
        label = <Label> {
            width: Fill,
            height: Fit,
            draw_text: {
                wrap: Word,
                text_style: <TEXT_SUB> {},
                color: (COLOR_P),
            },
            text: "no text"
        }
    }
    Pages = <View> {
        height: Fill, width: Fill
        pages_dock = <Dock> {
            width: Fill, height: Fill,
            tab_bar: {
                height: 0,
                InvisoTab = <Tab> {
                    closeable: false
                }
            }
            root = Tabs {
                tabs: [first_tab],
                selected: 0
            }
            first_tab = Tab {
                name: ""
                template: InvisoTab,
                kind: Page
            }
            Page = <RectView> {
                <View> {
                    height: Fill, width: Fill
                    padding: 0,
                    show_bg: true,
                    draw_bg: {color: (COLOR_BG)}
                    sections = <PortalList> {
                        max_pull_down: 0.0,
                        text_section = <TextSection> {}
                        image_section = <ImageSection> {}
                        text_image_section = <TextImageSection> {}
                        image_text_section = <ImageTextSection> {}
                        title_section = <TitleSection> {}
                        space_section = <SpaceSection> {}
                        image_grid_section = <ImageGridSection> {}
                    }
                }
            }
        }
    }
    Body = {{Body}} {
        width: Fill, height: Fill,
        flow: Overlay,
        padding: 0.0,
        spacing: 0,
        pages = <Pages> {}
        top = <View> {
            width: Fill, height: Fill, 
            flow: Down,
            header = <Header> {}
            <FillerY> {}
            alert_message = <AlertMessage> {}
        }
    }
    Ui = <Window> {
        window: {inner_size: vec2(800, 800)},
        body = <Body> {}
    }
);

#[derive(Live, LiveHook, Widget)]
pub struct Body {
    #[deref]
    view: View,
}

impl Widget for Body {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let state = scope.data.get::<State>().unwrap();
        let config = state.config.clone();
        let pages = state.pages.clone();
        let logo_image_loaded = state.logo_image_loaded;
        let host = state.host.clone();
        let menu_range_end = state.pages.len();
        while let Some(widget) = self.view.draw_walk(cx, scope, walk).step() {
            log!("BOOOP");
            // draw the site logo
            // update the site name based on the config (FIXME)
            if let Some(logo) = self.widget(id!(logo)).as_view().borrow_mut() {
                if !logo_image_loaded {
                    let id = LiveId::from_str_with_lut("image_logo").unwrap();
                    let url = config.logo_image_url.clone();
                    if url.is_some() && !url.clone().unwrap().is_empty() {
                        State::fetch_image(cx, id, &url.unwrap(), &host);
                    }
                }
            }
            // draw the nav menu
            if let Some(mut list) = self.widget(id!(nav_menu)).as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, menu_range_end);
                while let Some(index) = list.next_visible_item(cx) {
                    if index < menu_range_end {
                        let item = list.item(cx, index, live_id!(nav_menu_button)).unwrap();
                        // add the menu item in page order, but only if the page exists
                        for (i,p) in pages.iter().enumerate() {
                            if !p.name.is_empty() && (p.name == config.page_order[index]) {
                                item.set_text(p.name.as_str());
                            }
                        }
                        item.draw_all(cx, scope);
                    }
                }
            }
            // draw the pages (not working)
            if self.widget(id!(pages_dock)).as_dock().borrow_mut().is_some() {
                let dock = self.widget(id!(pages_dock)).as_dock();
                for page in pages.clone() {
                    // add the tab to the dock widget
                    let tab_id = LiveId::from_str_with_lut(format!("tab_{}", page.name).as_str()).unwrap();
                    let (tab_bar, pos) = dock.find_tab_bar_of_tab(live_id!(first_tab)).unwrap();
                    let page_tab = dock.create_tab(cx, tab_bar, tab_id, live_id!(Page), "".to_string(), live_id!(InvisoTab), None).unwrap();
                    // fill the sections in the page widget
                    if let Some(mut list) = page_tab.widget(id!(sections)).as_portal_list().borrow_mut() {
                        let range_end = page.sections.len();
                        list.set_item_range(cx, 0, range_end);
                        while let Some(index) = list.next_visible_item(cx) {
                            if index < range_end {
                                let template = match App::get_layout_template(page.sections[index].layout.as_str()) {
                                    Some(t) => {t}
                                    None => {live_id!(space_section)}
                                };
                                let item = list.item(cx, index, template).unwrap();
                                if page.sections[index]
                                    .text.as_ref()
                                    .is_some_and(|s| !s.is_empty()) {
                                        item.label(id!(label))
                                        .set_text(page.sections[index].text.as_ref().unwrap());
                                }
                                if [live_id!(image_section),live_id!(text_image_section),live_id!(image_text_section)]
                                .contains(&template) {
                                    // check if the image has been cached to avoid repetitive fetching
                                    if page.sections[index].image_cache.is_none() 
                                    || page.sections[index].image_cache.as_ref().unwrap().is_empty() {
                                        log!("{} s{:?} NO image cache, so fetching", page.name, index);
                                        let id_string = format!("image_{}_s{}", page.name, index.to_string());
                                        let id = LiveId::from_str_with_lut(id_string.as_str()).unwrap();
                                        let url = page.sections[index].image_url.clone();
                                        if url.is_some() && !url.clone().unwrap().is_empty() {
                                            State::fetch_image(cx, id, &url.unwrap(), &host);
                                        }
                                    }
                                    else {
                                        let cache = page.sections[index].image_cache.clone().unwrap();
                                        log!("{} s{:?} YES image cache, loading it", page.name, index);
                                        let image = item.image(id!(image));
                                        App::load_image(cx, image, &cache);
                                    }
                                }
                                item.draw_all(cx, scope);
                            }
                        }
                    }
                    page_tab.draw_all(cx, scope);
                }
            }
        }
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }
}

impl WidgetMatchEvent for Body {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let state = scope.data.get_mut::<State>().unwrap();
        let sections_with_actions = self
            .view
            .portal_list(id!(sections))
            .items_with_actions(actions);
        let menu_items_with_actions = self
            .view
            .portal_list(id!(nav_menu))
            .items_with_actions(actions);
        let nav_menu_clicked = menu_items_with_actions.iter().find(|(_index, widget)| {
            match widget.as_button().borrow_mut() {
                Some(btn) => btn.clicked(actions),
                None => false,
            }
        });
        if let Some((_index, _widget)) = nav_menu_clicked {
            // this is handled in app
        }
    }
}