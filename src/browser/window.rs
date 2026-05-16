use tao::{
    dpi::{LogicalPosition, LogicalSize, PhysicalSize},
    event_loop::EventLoopProxy,
    window::Window,
};
use wry::{PageLoadEvent, Rect, WebView, WebViewBuilder};

use crate::app::UserEvent;
use crate::browser::ipc::{BrowserCommand, IpcMessage};
use crate::utils::url::{resolve, HOME};

const TOOLBAR_H: u32 = 52;

pub struct BrowserWindow {
    toolbar: WebView,
    content: WebView,
    history: Vec<String>,
    pos: usize,
    in_history_nav: bool,
}

impl BrowserWindow {
    pub fn new(
        window: &Window,
        proxy: EventLoopProxy<UserEvent>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let size = window.inner_size().to_logical::<u32>(window.scale_factor());
        let content_h = size.height.saturating_sub(TOOLBAR_H);

        let proxy_toolbar = proxy.clone();
        let toolbar_html = include_str!("../../ui/toolbar/index.html");

        let toolbar = WebViewBuilder::new_as_child(window)
            .with_bounds(Rect {
                position: LogicalPosition::new(0, 0).into(),
                size: LogicalSize::new(size.width, TOOLBAR_H).into(),
            })
            .with_html(toolbar_html)
            .with_navigation_handler(move |url: String| {
                if let Some(query) = url.strip_prefix("astra-ipc://x?") {
                    let params: std::collections::HashMap<String, String> =
                        url::form_urlencoded::parse(query.as_bytes())
                            .into_owned()
                            .collect();
                    if let Some(json) = params.get("msg") {
                        if let Ok(msg) = serde_json::from_str::<IpcMessage>(json) {
                            let cmd = msg.into_command(|u| resolve(u));
                            let _ = proxy_toolbar.send_event(UserEvent::Command(cmd));
                        }
                    }
                    false
                } else {
                    true
                }
            })
            .build()?;

        let proxy_load = proxy.clone();
        let content = WebViewBuilder::new_as_child(window)
            .with_bounds(Rect {
                position: LogicalPosition::new(0, TOOLBAR_H).into(),
                size: LogicalSize::new(size.width, content_h).into(),
            })
            .with_url(HOME)
            .with_custom_protocol("astra".to_string(), |_req| {
                let html: &'static [u8] = include_bytes!("../../ui/newtab/index.html");
                wry::http::Response::builder()
                    .header("content-type", "text/html; charset=utf-8")
                    .body(std::borrow::Cow::Borrowed(html))
                    .unwrap()
            })
            .with_on_page_load_handler(move |event, url| {
                let cmd = match event {
                    PageLoadEvent::Started  => BrowserCommand::SetLoading(true),
                    PageLoadEvent::Finished => BrowserCommand::PageLoaded(url),
                };
                let _ = proxy_load.send_event(UserEvent::Command(cmd));
            })
            .build()?;

        Ok(Self {
            toolbar,
            content,
            history: vec![HOME.to_string()],
            pos: 0,
            in_history_nav: false,
        })
    }

    pub fn handle_command(&mut self, cmd: BrowserCommand) {
        match cmd {
            BrowserCommand::Navigate(url) => {
                self.in_history_nav = false;
                let _ = self.content.load_url(&url);
            }
            BrowserCommand::Back => {
                if self.pos > 0 {
                    self.pos -= 1;
                    self.in_history_nav = true;
                    let url = self.history[self.pos].clone();
                    let _ = self.content.load_url(&url);
                    self.sync_url_bar(&url);
                }
            }
            BrowserCommand::Forward => {
                if self.pos + 1 < self.history.len() {
                    self.pos += 1;
                    self.in_history_nav = true;
                    let url = self.history[self.pos].clone();
                    let _ = self.content.load_url(&url);
                    self.sync_url_bar(&url);
                }
            }
            BrowserCommand::Reload => {
                let _ = self.content.evaluate_script("location.reload()");
            }
            BrowserCommand::PageLoaded(url) => {
                self.sync_url_bar(&url);
                if !self.in_history_nav {
                    self.history.truncate(self.pos + 1);
                    if self.history.last().map(String::as_str) != Some(&url) {
                        self.history.push(url);
                        self.pos = self.history.len() - 1;
                    }
                }
                self.in_history_nav = false;
                self.update_nav_buttons();
            }
            BrowserCommand::SetLoading(loading) => {
                let _ = self.toolbar.evaluate_script(&format!("setLoading({})", loading));
            }
            BrowserCommand::NewTab(_) => {}
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>, scale: f64) {
        let size = size.to_logical::<u32>(scale);
        let content_h = size.height.saturating_sub(TOOLBAR_H);

        let _ = self.toolbar.set_bounds(Rect {
            position: LogicalPosition::new(0, 0).into(),
            size: LogicalSize::new(size.width, TOOLBAR_H).into(),
        });
        let _ = self.content.set_bounds(Rect {
            position: LogicalPosition::new(0, TOOLBAR_H).into(),
            size: LogicalSize::new(size.width, content_h).into(),
        });
    }

    fn sync_url_bar(&self, url: &str) {
        let escaped = url.replace('\\', "\\\\").replace('\'', "\\'");
        let _ = self.toolbar.evaluate_script(&format!("setUrl('{}')", escaped));
    }

    fn update_nav_buttons(&self) {
        let can_back    = self.pos > 0;
        let can_forward = self.pos + 1 < self.history.len();
        let _ = self.toolbar.evaluate_script(
            &format!("setNavState({}, {})", can_back, can_forward)
        );
    }
}
