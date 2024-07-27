#[allow(unused)]
trait BrowserEngine {
    fn new(width: u32, height: u32) -> Self;
    fn update(&self);
    fn render(&self);
    fn pixel_buffer(&mut self) -> Option<Vec<u8>>;
    fn goto_url(&self, url: &str);
}

#[cfg(feature = "webkit")]
#[allow(dead_code)]
mod ultralight {
    use ul_next::{
        config::Config,
        platform::{self, LogLevel, Logger},
        renderer::Renderer,
        surface::Surface,
        view::{View, ViewConfig},
    };

    struct MyLogger;

    impl Logger for MyLogger {
        fn log_message(&mut self, log_level: LogLevel, message: String) {
            println!("{:?}: {}", log_level, message);
        }
    }

    struct Ultralight {
        renderer: Renderer,
        view: View,
        surface: Surface,
        width: u32,
        height: u32,
        image: Option<Vec<u8>>,
    }

    impl super::BrowserEngine for Ultralight {
        fn new(width: u32, height: u32) -> Self {
            let config = Config::start().build().unwrap();
            platform::enable_platform_fontloader();
            // TODO: this should change to ~/.rust-browser
            platform::enable_platform_filesystem(".").unwrap();
            platform::set_logger(MyLogger);
            // TODO: this should change to ~/.rust-browser
            platform::enable_default_logger("./log.txt").unwrap();
            let renderer = Renderer::create(config).unwrap();
            let view_config = ViewConfig::start()
                .initial_device_scale(2.0)
                .font_family_standard("Arial")
                .is_accelerated(false)
                .build()
                .unwrap();

            let view = renderer
                .create_view(width, height, &view_config, None)
                .unwrap();

            let surface = view.surface().unwrap();

            let bytes_per_pixel = surface.row_bytes() / width;
            // RGBA
            assert!(bytes_per_pixel == 4);

            Self {
                renderer,
                view,
                surface,
                width,
                height,
                image: None,
            }
        }

        fn update(&self) {
            self.renderer.update()
        }

        fn render(&self) {
            self.renderer.render()
        }

        fn pixel_buffer(&mut self) -> Option<Vec<u8>> {
            // Get the raw pixels of the surface
            if let Some(pixels_data) = self.surface.lock_pixels() {
                let mut vec = Vec::new();
                vec.extend_from_slice(&pixels_data);
                Some(vec)
            } else {
                None
            }
        }

        fn goto_url(&self, url: &str) {
            self.view.load_url(url).unwrap();
        }
    }
}
