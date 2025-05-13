use slider_rs::yew::Slider;
use slider_rs::{Color, Orientation, Size};
use yew::prelude::*;

#[function_component(Disabled)]
fn disabled() -> Html {
    html! { <Slider label="Disabled Slider" disabled=true /> }
}

#[function_component(Sizes)]
fn sizes() -> Html {
    html! {
        <div class="space-y-4">
            <Slider label="Small" size={Size::Sm} />
            <Slider label="Medium" size={Size::Md} />
            <Slider label="Large" size={Size::Lg} />
        </div>
    }
}

#[function_component(KeyboardStep)]
fn keyboard_step() -> Html {
    html! { <Slider label="Keyboard Step (x5)" keyboard_step=5.0 show_value=true /> }
}

#[function_component(Colors)]
fn colors() -> Html {
    html! {
        <div class="space-y-4">
            <Slider label="Primary" color={Color::Primary} />
            <Slider label="Success" color={Color::Success} />
            <Slider label="Warning" color={Color::Warning} />
            <Slider label="Danger" color={Color::Danger} />
        </div>
    }
}

#[function_component(VerticalSlider)]
fn vertical_slider() -> Html {
    html! {
        <Slider
            label="Vertical Slider"
            orientation={Orientation::Vertical}
            show_steps=true
            show_value=true
        />
    }
}

#[function_component(VisibleSteps)]
fn visible_steps() -> Html {
    html! { <Slider label="Visible Steps" step=1.0 show_steps=true /> }
}

#[function_component(DoubleSlider)]
fn double_slider() -> Html {
    let range = use_state(|| (20.0, 80.0));
    let on_change_range = {
        let range = range.clone();
        Callback::from(move |r: (f64, f64)| range.set(r))
    };

    html! {
        <>
            <Slider
                label="Double Slider"
                double=true
                range={Some(*range)}
                on_change_range={on_change_range}
                show_value=true
            />
            <p>{ format!("Range: {} - {}", range.0, range.1) }</p>
        </>
    }
}

#[function_component(IconEdges)]
fn icon_edges() -> Html {
    html! {
        <Slider
            label="Volume Control"
            icon_start={Some(html! { <span>{ "🗣️" }</span> })}
            icon_end={Some(html! { <span>{ "🔊" }</span> })}
            show_value=true
        />
    }
}

#[function_component(DynamicMinMax)]
fn dynamic_min_max() -> Html {
    let min = use_state(|| 10.0);
    let max = use_state(|| 90.0);
    let value = use_state(|| 50.0);

    let on_change = {
        let value = value.clone();
        Callback::from(move |v: f64| value.set(v))
    };

    html! {
        <>
            <button
                onclick={Callback::from({
                let min = min.clone();
                move |_| min.set(20.0)
            })}
            >
                { "Set Min to 20" }
            </button>
            <Slider
                label="Dynamic Range"
                min={*min}
                max={*max}
                value={Some(*value)}
                on_change={on_change}
                show_value=true
            />
        </>
    }
}

#[function_component(WithOutline)]
fn with_outline() -> Html {
    html! { <Slider label="Outlined Slider" container_class="border border-gray-500 p-4 rounded" /> }
}

#[function_component(ValueFormatting)]
fn value_formatting() -> Html {
    html! { <Slider label="Value: %" show_value=true /> }
}

#[function_component(HideValue)]
fn hide_value() -> Html {
    html! { <Slider label="Hidden Value" show_value=false /> }
}

#[function_component(Controlled)]
fn controlled() -> Html {
    let value = use_state(|| 40.0);
    let on_change = {
        let value = value.clone();
        Callback::from(move |v: f64| value.set(v))
    };
    html! {
        <>
            <Slider label="Controlled" value={Some(*value)} on_change={on_change.clone()} />
            <p>{ format!("Current value: {}", *value) }</p>
        </>
    }
}

#[function_component(CustomStyle)]
fn custom_style() -> Html {
    html! {
        <Slider
            label="Custom Style"
            input_style=""
            show_steps=true
        />
    }
}

#[function_component(CustomLabel)]
fn custom_label() -> Html {
    html! { <Slider label="Custom Label" label_class="text-purple-700 font-bold" /> }
}

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-8 text-white">{ "Slider RS Yew Examples" }</h1>
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Disabled" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use slider_rs::yew::Slider;

#[function_component(Disabled)]
fn disabled() -> Html {
    html! {
        <Slider label="Disabled Slider" disabled=true />
    }
}"# }
                    </pre>
                    <Disabled />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Sizes" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use slider_rs::yew::Slider;
use slider_rs::Size;

#[function_component(Sizes)]
fn sizes() -> Html {
    html! {
        <div class="space-y-4">
            <Slider label="Small" size={Size::Sm} />
            <Slider label="Medium" size={Size::Md} />
            <Slider label="Large" size={Size::Lg} />
        </div>
    }
}"# }
                    </pre>
                    <Sizes />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Keyboard Step" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use slider_rs::yew::Slider;

#[function_component(KeyboardStep)]
fn keyboard_step() -> Html {
    html! {
        <Slider
            label="Keyboard Step (x5)"
            keyboard_step={5.0}
            show_value=true
        />
    }
}"# }
                    </pre>
                    <KeyboardStep />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Colors" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use slider_rs::yew::Slider;
use slider_rs::Color;

#[function_component(Colors)]
fn colors() -> Html {
    html! {
        <div class="space-y-4">
            <Slider label="Primary" color={Color::Primary} />
            <Slider label="Success" color={Color::Success} />
            <Slider label="Warning" color={Color::Warning} />
            <Slider label="Danger" color={Color::Danger} />
        </div>
    }
}"# }
                    </pre>
                    <Colors />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Vertical" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use slider_rs::yew::Slider;
use slider_rs::Orientation;

#[function_component(VerticalSlider)]
fn vertical_slider() -> Html {
    html! {
        <Slider label="Vertical Slider"
            orientation={Orientation::Vertical}
            show_steps=true 
            show_value={true}
        />
    }
}"# }
                    </pre>
                    <VerticalSlider />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Visible Steps" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use slider_rs::yew::Slider;

#[function_component(VisibleSteps)]
fn visible_steps() -> Html {
    html! {
        <Slider label="Visible Steps" step=1.0 show_steps=true />
    }
}"# }
                    </pre>
                    <VisibleSteps />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Double Slider" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use slider_rs::yew::Slider;

#[function_component(DoubleSlider)]
fn double_slider() -> Html {
    let range = use_state(|| (20.0, 80.0));
    let on_change_range = {
        let range = range.clone();
        Callback::from(move |r: (f64, f64)| range.set(r))
    };

    html! {
        <>
            <Slider
                label="Double Slider"
                double={true}
                range={Some(*range)}
                on_change_range={on_change_range}
                show_value=true
            />
            <p>{ format!("Range: {} - {}", range.0, range.1) }</p>
        </>
    }
}"# }
                    </pre>
                    <DoubleSlider />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Icon Edges" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use slider_rs::yew::Slider;

#[function_component(IconEdges)]
fn icon_edges() -> Html {
    html! {
        <Slider
            label="Volume Control"
            icon_start={Some(html! { <span>{ "🗣️" }</span> })}
            icon_end={Some(html! { <span>{ "🔊" }</span> })}
            show_value=true
        />
    }
}"# }
                    </pre>
                    <IconEdges />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Dynamic Min Max" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use slider_rs::yew::Slider;

#[function_component(DynamicMinMax)]
fn dynamic_min_max() -> Html {
    let min = use_state(|| 10.0);
    let max = use_state(|| 90.0);
    let value = use_state(|| 50.0);

    let on_change = {
        let value = value.clone();
        Callback::from(move |v: f64| value.set(v))
    };

    html! {
        <>
            <button onclick={Callback::from({
                let min = min.clone();
                move |_| min.set(20.0)
            })}>{ "Set Min to 20" }</button>
            <Slider
                label="Dynamic Range"
                min={*min}
                max={*max}
                value={Some(*value)}
                on_change={on_change}
                show_value=true
            />
        </>
    }
}"# }
                    </pre>
                    <DynamicMinMax />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Outline" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use slider_rs::yew::Slider;

#[function_component(WithOutline)]
fn with_outline() -> Html {
    html! {
        <Slider
            label="Outlined Slider"
            container_class="border border-gray-500 p-4 rounded"
        />
    }
}"# }
                    </pre>
                    <WithOutline />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Value Formatting" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use slider_rs::yew::Slider;

#[function_component(ValueFormatting)]
fn value_formatting() -> Html {
    html! {
        <Slider label="Value: %" show_value=true />
    }
}"# }
                    </pre>
                    <ValueFormatting />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Hide Value" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use slider_rs::yew::Slider;

#[function_component(HideValue)]
fn hide_value() -> Html {
    html! {
        <Slider label="Hidden Value" show_value=false />
    }
}"# }
                    </pre>
                    <HideValue />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Controlled" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use slider_rs::yew::Slider;

#[function_component(Controlled)]
fn controlled() -> Html {
    let value = use_state(|| 40.0);
    let on_change = {
        let value = value.clone();
        Callback::from(move |v: f64| value.set(v))
    };
    html! {
        <>
            <Slider label="Controlled" value={Some(*value)} on_change={on_change.clone()} />
            <p>{ format!("Current value: {}", *value) }</p>
        </>
    }
}"# }
                    </pre>
                    <Controlled />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Custom Style" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use slider_rs::yew::Slider;

#[function_component(CustomStyle)]
fn custom_style() -> Html {
    html! {
        <Slider
            label="Custom Style"
            input_style=""
            show_steps=true
        />
    }
}"# }
                    </pre>
                    <CustomStyle />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Custom Label" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use slider_rs::yew::Slider;

#[function_component(CustomLabel)]
fn custom_label() -> Html {
    html! {
        <Slider
            label="Custom Label"
            label_class="text-purple-700 font-bold"
        />
    }
}"# }
                    </pre>
                    <CustomLabel />
                </div>
            </div>
        </div>
    }
}
