use anyhow::Result;
use zelto_ir::node::ZeltoNode;
use zelto_win32::window::ZeltoWindow;
use crate::reconciler::Reconciler;
use crate::event_loop::run_message_loop;

pub struct AppConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            title: "Zelto App".into(),
            width: 800,
            height: 600,
            resizable: true,
        }
    }
}

/// Entry point for a Zelto application.
pub struct App {
    config: AppConfig,
    root_fn: Box<dyn Fn() -> ZeltoNode>,
}

impl App {
    pub fn new<F>(config: AppConfig, root_fn: F) -> Self
    where
        F: Fn() -> ZeltoNode + 'static,
    {
        Self {
            config,
            root_fn: Box::new(root_fn),
        }
    }

    pub fn run(self) -> Result<i32> {
        env_logger::init();

        ZeltoWindow::register_class()?;

        let window = ZeltoWindow::new(
            &self.config.title,
            self.config.width,
            self.config.height,
            self.config.resizable,
        )?;

        let root = (self.root_fn)();
        let (w, h) = window.client_rect();

        let mut reconciler = Reconciler::new();
        reconciler.mount(&root, window.hwnd, (w as f32, h as f32))?;

        run_message_loop()
    }
}
