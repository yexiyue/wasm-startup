use dialogue_macro::dialogue_define;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependence {
    pub name: &'static str,
    pub features: &'static str,
}

impl ToString for Dependence {
    fn to_string(&self) -> String {
        self.name.to_string()
    }
}

pub(crate) static DEPENDENCE: [Dependence; 13] = [
    Dependence {
        name: "wasm-bindgen",
        features: "",
    },
    Dependence {
        name: "wasm-bindgen-futures",
        features: "",
    },
    Dependence {
        name: "js-sys",
        features: "",
    },
    Dependence {
        name: "web-sys",
        features: "",
    },
    Dependence {
        name: "serde",
        features: "serde/derive",
    },
    Dependence {
        name: "serde_json",
        features: "",
    },
    Dependence {
        name: "serde-wasm-bindgen",
        features: "",
    },
    Dependence {
        name: "tracing",
        features: "",
    },
    Dependence {
        name: "tracing-wasm",
        features: "",
    },
    Dependence {
        name: "console_error_panic_hook",
        features: "",
    },
    Dependence {
        name: "wgpu",
        features: "",
    },
    Dependence {
        name: "rayon",
        features: "",
    },
    Dependence {
        name: "wasm-bindgen-rayon",
        features: "",
    },
];

pub(crate) static DEFAULT_DEPENDENCE: [Dependence; 8] = [
    Dependence {
        name: "wasm-bindgen",
        features: "",
    },
    Dependence {
        name: "js-sys",
        features: "",
    },
    Dependence {
        name: "web-sys",
        features: "",
    },
    Dependence {
        name: "serde",
        features: "serde/derive",
    },
    Dependence {
        name: "serde-wasm-bindgen",
        features: "",
    },
    Dependence {
        name: "tracing",
        features: "",
    },
    Dependence {
        name: "tracing-wasm",
        features: "",
    },
    Dependence {
        name: "console_error_panic_hook",
        features: "",
    },
];


