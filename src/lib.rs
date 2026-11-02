use zed_extension_api as zed;

struct TodoTreeExtension;

impl zed::Extension for TodoTreeExtension {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(TodoTreeExtension);
