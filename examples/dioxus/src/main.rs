use dioxus::prelude::*;
use dioxus_logger::tracing;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/output.css");

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(app);
}

fn app() -> Element {
    rsx! {
        document::Stylesheet { href: TAILWIND_CSS },
        document::Link { rel: "icon", href: FAVICON }
        Examples {}
    }
}

use dioxus::prelude::*;
use slider_rs::dioxus::Slider;
use slider_rs::{Color, Orientation, Size};

#[component]
fn Disabled() -> Element {
    rsx! {
        Slider {
            label: "Disabled Slider",
            disabled: true,
        }
    }
}

#[component]
fn Sizes() -> Element {
    rsx! {
        div {
            class: "space-y-4",
            Slider {
                label: "Small",
                size: Size::Sm,
            }
            Slider {
                label: "Medium",
                size: Size::Md,
            }
            Slider {
                label: "Large",
                size: Size::Lg,
            }
        }
    }
}

#[component]
fn KeyboardStep() -> Element {
    rsx! {
        Slider {
            label: "Keyboard Step (x5)",
            keyboard_step: 5.0,
            show_value: true,
        }
    }
}

#[component]
fn Colors() -> Element {
    rsx! {
        div {
            class: "space-y-4",
            Slider {
                label: "Primary",
                color: Color::Primary,
            }
            Slider {
                label: "Success",
                color: Color::Success,
            }
            Slider {
                label: "Warning",
                color: Color::Warning,
            }
            Slider {
                label: "Danger",
                color: Color::Danger,
            }
        }
    }
}

#[component]
fn VerticalSlider() -> Element {
    rsx! {
        Slider {
            label: "Vertical Slider",
            orientation: Orientation::Vertical,
            show_steps: true,
            show_value: true,
        }
    }
}

#[component]
fn VisibleSteps() -> Element {
    rsx! {
        Slider {
            label: "Visible Steps",
            step: 1.0,
            show_steps: true,
        }
    }
}

#[component]
fn DoubleSlider() -> Element {
    let mut range = use_signal(|| (20.0, 80.0));

    rsx! {
        Slider {
            label: "Double Slider",
            double: true,
            range: Some(range()),
            on_change_range: move |r: (f64, f64)| range.set(r),
            show_value: true,
        }
        p {
            "Range: {range().0} - {range().1}"
        }
    }
}

#[component]
fn IconEdges() -> Element {
    rsx! {
        Slider {
            label: "Volume Control",
            icon_start: Some(rsx!(span { "🗣️" })),
            icon_end: Some(rsx!(span { "🔊" })),
            show_value: true,
        }
    }
}

#[component]
fn DynamicMinMax() -> Element {
    let mut min = use_signal(|| 10.0);
    let max = use_signal(|| 90.0);
    let mut value = use_signal(|| 50.0);

    rsx! {
        button {
            onclick: move |_| min.set(20.0),
            "Set Min to 20"
        }
        Slider {
            label: "Dynamic Range",
            min: min(),
            max: max(),
            value: Some(value()),
            on_change: move |v: f64| value.set(v),
            show_value: true,
        }
    }
}

#[component]
fn WithOutline() -> Element {
    rsx! {
        Slider {
            label: "Outlined Slider",
            container_class: "border border-gray-500 p-4 rounded",
        }
    }
}

#[component]
fn ValueFormatting() -> Element {
    rsx! {
        Slider {
            label: "Value: %",
            show_value: true,
        }
    }
}

#[component]
fn HideValue() -> Element {
    rsx! {
        Slider {
            label: "Hidden Value",
            show_value: false,
        }
    }
}

#[component]
fn Controlled() -> Element {
    let mut value = use_signal(|| 40.0);

    rsx! {
        Slider {
            label: "Controlled",
            value: Some(value()),
            on_change: move |v: f64| value.set(v),
        }
        p {
            "Current value: {value()}"
        }
    }
}

#[component]
fn CustomStyle() -> Element {
    rsx! {
        Slider {
            label: "Custom Style",
            input_style: "",
            show_steps: true,
        }
    }
}

#[component]
fn CustomLabel() -> Element {
    rsx! {
        Slider {
            label: "Custom Label",
            label_class: "text-purple-700 font-bold",
        }
    }
}

#[component]
fn Examples() -> Element {
    rsx! {
        div {
            class: "m-6 min-h-screen flex flex-col items-center justify-center",
            h1 {
                class: "text-3xl font-bold mb-8 text-white",
                "Slider RS Dioxus Examples"
            }
            div {
                class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8",

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md w-full",
                    h2 { class: "text-xl font-bold mb-2", "Disabled" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r#"use dioxus::prelude::*;
use slider_rs::dioxus::Slider;

#[component]
fn Disabled() -> Element {{
    rsx! {{
        Slider {{
            label: "Disabled Slider",
            disabled: true,
        }}
    }}
}}"#
                    }
                    Disabled {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md w-full",
                    h2 { class: "text-xl font-bold mb-2", "Sizes" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r#"use dioxus::prelude::*;
use slider_rs::dioxus::Slider;
use slider_rs::Size;

#[component]
fn Sizes() -> Element {{
    rsx! {{
        div {{
            class: "space-y-4",
            Slider {{
                label: "Small",
                size: Size::Sm,
            }}
            Slider {{
                label: "Medium",
                size: Size::Md,
            }}
            Slider {{
                label: "Large",
                size: Size::Lg,
            }}
        }}
    }}
}}"#
                    }
                    Sizes {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md w-full",
                    h2 { class: "text-xl font-bold mb-2", "Keyboard Step" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r#"use dioxus::prelude::*;
use slider_rs::dioxus::Slider;

#[component]
fn KeyboardStep() -> Element {{
    rsx! {{
        Slider {{
            label: "Keyboard Step (x5)",
            keyboard_step: 5.0,
            show_value: true,
        }}
    }}
}}"#
                    }
                    KeyboardStep {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md w-full",
                    h2 { class: "text-xl font-bold mb-2", "Colors" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r#"use dioxus::prelude::*;
use slider_rs::dioxus::Slider;
use slider_rs::Color;

#[component]
fn Colors() -> Element {{
    rsx! {{
        div {{
            class: "space-y-4",
            Slider {{
                label: "Primary",
                color: Color::Primary,
            }}
            Slider {{
                label: "Success",
                color: Color::Success,
            }}
            Slider {{
                label: "Warning",
                color: Color::Warning,
            }}
            Slider {{
                label: "Danger",
                color: Color::Danger,
            }}
        }}
    }}
}}"#
                    }
                    Colors {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md w-full",
                    h2 { class: "text-xl font-bold mb-2", "Vertical" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r#"use dioxus::prelude::*;
use slider_rs::dioxus::Slider;
use slider_rs::Orientation;

#[component]
fn VerticalSlider() -> Element {{
    rsx! {{
        Slider {{
            label: "Vertical Slider",
            orientation: Orientation::Vertical,
            show_steps: true,
            show_value: true,
        }}
    }}
}}"#
                    }
                    VerticalSlider {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md w-full",
                    h2 { class: "text-xl font-bold mb-2", "Visible Steps" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r#"use dioxus::prelude::*;
use slider_rs::dioxus::Slider;

#[component]
fn VisibleSteps() -> Element {{
    rsx! {{
        Slider {{
            label: "Visible Steps",
            step: 1.0,
            show_steps: true,
        }}
    }}
}}"#
                    }
                    VisibleSteps {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md w-full",
                    h2 { class: "text-xl font-bold mb-2", "Double Slider (WIP)" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r#"use dioxus::prelude::*;
use slider_rs::dioxus::Slider;

#[component]
fn DoubleSlider() -> Element {{
    let range = use_signal(|| (20.0, 80.0));

    rsx! {{
        Slider {{
            label: "Double Slider",
            double: true,
            range: Some(range()),
            on_change_range: move |r: (f64, f64)| range.set(r),
            show_value: true,
        }}
        p {{
            "Range: {{range().0}} - {{range().1}}"
        }}
    }}
}}"#
                    }
                    DoubleSlider {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md w-full",
                    h2 { class: "text-xl font-bold mb-2", "Icon Edges" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r#"use dioxus::prelude::*;
use slider_rs::dioxus::Slider;

#[component]
fn IconEdges() -> Element {{
    rsx! {{
        Slider {{
            label: "Volume Control",
            icon_start: Some(rsx!(span {{ "🗣️" }})),
            icon_end: Some(rsx!(span {{ "🔊" }})),
            show_value: true,
        }}
    }}
}}"#
                    }
                    IconEdges {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md w-full",
                    h2 { class: "text-xl font-bold mb-2", "Dynamic Min/Max" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r#"use dioxus::prelude::*;
use slider_rs::dioxus::Slider;

#[component]
fn DynamicMinMax() -> Element {{
    let mut min = use_signal(|| 10.0);
    let max = use_signal(|| 90.0);
    let mut value = use_signal(|| 50.0);

    rsx! {{
        button {{
            onclick: move |_| min.set(20.0),
            "Set Min to 20"
        }}
        Slider {{
            label: "Dynamic Range",
            min: min(),
            max: max(),
            value: Some(value()),
            on_change: move |v| value.set(v),
            show_value: true,
        }}
    }}
}}"#
                    }
                    DynamicMinMax {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md w-full",
                    h2 { class: "text-xl font-bold mb-2", "With Outline" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r#"use dioxus::prelude::*;
use slider_rs::dioxus::Slider;

#[component]
fn WithOutline() -> Element {{
    rsx! {{
        Slider {{
            label: "Outlined Slider",
            container_class: "border border-gray-500 p-4 rounded",
        }}
    }}
}}"#
                    }
                    WithOutline {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md w-full",
                    h2 { class: "text-xl font-bold mb-2", "Value Formatting" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r#"use dioxus::prelude::*;
use slider_rs::dioxus::Slider;

#[component]
fn ValueFormatting() -> Element {{
    rsx! {{
        Slider {{
            label: "Value: %",
            show_value: true,
        }}
    }}
}}"#
                    }
                    ValueFormatting {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md w-full",
                    h2 { class: "text-xl font-bold mb-2", "Hide Value" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r#"use dioxus::prelude::*;
use slider_rs::dioxus::Slider;

#[component]
fn HideValue() -> Element {{
    rsx! {{
        Slider {{
            label: "Hidden Value",
            show_value: false,
        }}
    }}
}}"#
                    }
                    HideValue {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md w-full",
                    h2 { class: "text-xl font-bold mb-2", "Controlled" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r#"use dioxus::prelude::*;
use slider_rs::dioxus::Slider;

#[component]
fn Controlled() -> Element {{
    let mut value = use_signal(|| 40.0);

    rsx! {{
        Slider {{
            label: "Controlled",
            value: Some(value()),
            on_change: move |v| value.set(v),
        }}
        p {{
            "Current value: {{value()}}"
        }}
    }}
}}"#
                    }
                    Controlled {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md w-full",
                    h2 { class: "text-xl font-bold mb-2", "Custom Style" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r#"use dioxus::prelude::*;
use slider_rs::dioxus::Slider;

#[component]
fn CustomStyle() -> Element {{
    rsx! {{
        Slider {{
            label: "Custom Style",
            input_style: "",
            show_steps: true,
        }}
    }}
}}"#
                    }
                    CustomStyle {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md w-full",
                    h2 { class: "text-xl font-bold mb-2", "Custom Label" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r#"use dioxus::prelude::*;
use slider_rs::dioxus::Slider;

#[component]
fn CustomLabel() -> Element {{
    rsx! {{
        Slider {{
            label: "Custom Label",
            label_class: "text-purple-700 font-bold",
        }}
    }}
}}"#
                    }
                    CustomLabel {}
                }
            }
        }
    }
}
