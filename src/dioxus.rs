#![doc = include_str!("../DIOXUS.md")]

use crate::common::{Color, Cursor, Height, Orientation, Size, Width};
use dioxus::prelude::*;
use std::rc::Rc;
use uuid::Uuid;
use web_sys::HtmlInputElement;

#[derive(Props, PartialEq, Clone)]
pub struct LabelProps {
    #[props(default)]
    label: &'static str,
    #[props(default = "font-size: 14px; margin-bottom: 8px; text-align: center;")]
    label_style: &'static str,
    #[props(default = "slider-label")]
    label_class: &'static str,
}

#[component]
fn Label(props: LabelProps) -> Element {
    rsx! {
        label {
            class: "{props.label_class}",
            style: "{props.label_style}",
            "{props.label}"
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct StepsProps {
    #[props(default = 0.0)]
    min: f64,
    #[props(default = 10.0)]
    max: f64,
    #[props(default = 1.0)]
    step: f64,
    #[props(
        default = "width: 100%; display: flex; justify-content: space-between; margin-top: 8px; font-size: 10px;"
    )]
    steps_style: &'static str,
    #[props(default)]
    orientation: Orientation,
}

#[component]
fn Steps(props: StepsProps) -> Element {
    let count = ((props.max - props.min) / props.step).floor() as usize;

    let steps = (0..=count).map(|i| {
        let val = props.min + (i as f64 * props.step);
        let style = if props.orientation.is_vertical() {
            "margin: 4px 0; writing-mode: vertical-rl; text-align: center;"
        } else {
            "text-align: center;"
        };
        rsx! {
            span {
                style: "{style}",
                "{val:.0}"
            }
        }
    });

    let container_style = if props.orientation.is_vertical() {
        "display: flex; flex-direction: column; align-items: center; height: 100%; font-size: 10px;"
    } else {
        props.steps_style
    };

    rsx! {
        div {
            style: "{container_style}",
            {steps}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct OutputProps {
    #[props(default)]
    value_display: String,
    #[props(default = "font-size: 12px; margin-top: 8px; text-align: center;")]
    output_style: &'static str,
    #[props(default = "slider-output")]
    output_class: &'static str,
    #[props(
        default = "background-color: #333; color: #fff; padding: 4px 8px; border-radius: 4px; font-size: 12px; position: absolute; transform: translate(-50%, -120%); display: block; pointer-events: none;"
    )]
    tooltip_style: &'static str,
    #[props(default = false)]
    show_tooltip: bool,
    #[props(default)]
    tooltip_left: String,
}

#[component]
fn Output(props: OutputProps) -> Element {
    let style = format!("{} left: {};", props.tooltip_style, props.tooltip_left);
    rsx! {
        output {
            class: "{props.output_class}",
            style: "{props.output_style}",
            aria_live: "polite",
            "{props.value_display}"
        }
        if props.show_tooltip {
            div {
                class: "{props.output_class}",
                style: "{style}",
                "{props.value_display}"
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct TicksProps {
    #[props(default)]
    id: String,
    #[props(default = 0.0)]
    min: f64,
    #[props(default = 10.0)]
    max: f64,
    #[props(default = 1.0)]
    step: f64,
}

#[component]
fn Ticks(props: TicksProps) -> Element {
    let mut current = props.min;
    let mut options = vec![];

    while current <= props.max {
        options.push(rsx! {
            option {
                value: "{current}"
            }
        });
        current += props.step;
    }

    rsx! {
        datalist {
            id: "{props.id}",
            {options.into_iter()}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct InputProps {
    #[props(default)]
    input_ref: Signal<Option<Rc<MountedData>>>,
    #[props(default = 0.0)]
    min: f64,
    #[props(default = 10.0)]
    max: f64,
    #[props(default = 1.0)]
    step: f64,
    #[props(default = 0.0)]
    value: f64,
    #[props(default)]
    orientation: Orientation,
    #[props(default)]
    size: Size,
    #[props(default)]
    width: Width,
    #[props(default)]
    height: Height,
    #[props(default)]
    color: Color,
    #[props(default)]
    cursor_style: Cursor,
    #[props(default = false)]
    disabled: bool,
    #[props(default)]
    on_input: Callback<FormEvent>,
    #[props(default)]
    on_focus: Callback<FocusEvent>,
    #[props(default)]
    on_blur: Callback<FocusEvent>,
    #[props(default)]
    aria_label: Option<&'static str>,
    #[props(default)]
    aria_describedby: Option<&'static str>,
    #[props(default)]
    datalist_id: Option<String>,
    #[props(default = "slider-input")]
    input_class: &'static str,
    #[props(default = "border-radius: 8px; appearance: none; outline: none;")]
    input_style: &'static str,
    #[props(default = true)]
    use_gradient: bool,
    #[props(default)]
    custom_thumb_css: Option<&'static str>,
    #[props(default)]
    custom_thumb_html: Option<Element>,
    #[props(default = 1.0)]
    keyboard_step: f64,
    #[props(default = false)]
    rtl_fill: bool,
}

#[component]
fn Input(props: InputProps) -> Element {
    let mut props = props.clone();
    let value_percent = ((props.value - props.min) / (props.max - props.min)) * 100.0;
    let fill_color = props.color.to_color_code();
    let gradient = if props.use_gradient {
        if props.orientation.is_vertical() {
            if props.rtl_fill {
                format!(
                    "background: linear-gradient(to top, {} 0%, {} {:.2}%, #ccc {:.2}%, #ccc 100%);",
                    fill_color, fill_color, value_percent, value_percent
                )
            } else {
                format!(
                    "background: linear-gradient(to bottom, {} 0%, {} {:.2}%, #ccc {:.2}%, #ccc 100%);",
                    fill_color, fill_color, value_percent, value_percent
                )
            }
        } else if props.rtl_fill {
            format!(
                "background: linear-gradient(to left, {} 0%, {} {:.2}%, #ccc {:.2}%, #ccc 100%);",
                fill_color, fill_color, value_percent, value_percent
            )
        } else {
            format!(
                "background: linear-gradient(to right, {} 0%, {} {:.2}%, #ccc {:.2}%, #ccc 100%);",
                fill_color, fill_color, value_percent, value_percent
            )
        }
    } else {
        format!("background: {};", fill_color)
    };

    let base_style = format!(
        "cursor: pointer; transition: background 0.3s; {} {} {} {} {} {}",
        props.input_style,
        props.width.to_style(),
        props.height.to_style(),
        gradient,
        props.orientation.to_style(),
        props.size.to_style(),
    );

    let on_key_down = Callback::new({
        move |e: Event<KeyboardData>| {
            let data = e.data;
            if let Some(el) = (props.input_ref)() {
                if let Some(input) = el.downcast::<HtmlInputElement>() {
                    let current = input.value().parse::<f64>().unwrap_or(0.0);
                    let new_val = match data.key() {
                        Key::ArrowLeft | Key::ArrowDown => current - props.keyboard_step,
                        Key::ArrowRight | Key::ArrowUp => current + props.keyboard_step,
                        _ => current,
                    }
                    .clamp(props.min, props.max);

                    input.set_value(&new_val.to_string());

                    if let Ok(event) = web_sys::Event::new("input") {
                        let _ = input.dispatch_event(&event);
                    }
                }
            }
        }
    });

    rsx! {
        input {
            onmounted: move |cx| props.input_ref.set(Some(cx.data())),
            r#type: "range",
            class: "{props.input_class}",
            min: "{props.min}",
            max: "{props.max}",
            step: if props.step == 0.0 { "any".to_string() } else { props.step.to_string() },
            value: "{props.value}",
            list: props.datalist_id.clone().unwrap_or_default(),
            oninput: move |e| props.on_input.call(e),
            onfocus: move |e| props.on_focus.call(e),
            onblur: move |e| props.on_blur.call(e),
            onkeydown: on_key_down,
            disabled: props.disabled,
            aria_label: props.aria_label.unwrap_or("Slider"),
            aria_describedby: props.aria_describedby.unwrap_or("Slider description"),
            style: "{base_style}",
        }
        if let Some(custom_html) = props.custom_thumb_html.clone() {
            {custom_html}
        }
    }
}

/// Props for the `Slider` component.
///
/// These props configure the behavior, appearance, and accessibility of a slider input component
/// supporting both single and double (range) sliders.
///
/// # Features
/// - Single or double slider modes.
/// - Vertical or horizontal orientation.
/// - Customizable appearance with CSS styles and classes.
/// - Tooltip, value display, and step indicators.
/// - Full accessibility support (ARIA attributes, keyboard steps).
#[derive(PartialEq, Clone, Props)]
pub struct SliderProps {
    /// Label text displayed above the slider.
    #[props(default)]
    pub label: &'static str,

    /// Minimum value for the slider.
    #[props(default = 0.0)]
    pub min: f64,

    /// Maximum value for the slider.
    #[props(default = 10.0)]
    pub max: f64,

    /// Increment step size.
    #[props(default = 1.0)]
    pub step: f64,

    /// Current value for a single slider mode.
    #[props(default)]
    pub value: Option<f64>,

    /// Current range values (start, end) for double slider mode.
    #[props(default)]
    pub range: Option<(f64, f64)>,

    /// Whether to enable double slider mode (range selector).
    #[props(default = false)]
    pub double: bool,

    /// Slider orientation: horizontal or vertical.
    #[props(default)]
    pub orientation: Orientation,

    /// Size variant for styling the slider track and thumb.
    #[props(default)]
    pub size: Size,

    /// Color variant for styling the slider.
    #[props(default)]
    pub color: Color,

    /// Cursor style when hovering over the slider.
    #[props(default)]
    pub cursor_style: Cursor,

    /// Whether to show the current value as an output.
    #[props(default = false)]
    pub show_value: bool,

    /// Whether to show step ticks along the slider track.
    #[props(default = false)]
    pub show_steps: bool,

    /// Whether to show tooltip above the thumb on hover.
    #[props(default = false)]
    pub show_tooltip: bool,

    /// Whether to disable interaction with the slider.
    #[props(default = false)]
    pub disabled: bool,

    /// Callback triggered when the slider value changes.
    #[props(default)]
    pub on_change: Callback<f64>,

    /// Callback triggered when the slider range changes (double mode).
    #[props(default)]
    pub on_change_range: Callback<(f64, f64)>,

    /// Callback triggered when the slider gains focus.
    #[props(default)]
    pub on_focus: Callback<()>,

    /// Callback triggered when the slider loses focus.
    #[props(default)]
    pub on_blur: Callback<()>,

    /// ARIA label for accessibility.
    #[props(default)]
    pub aria_label: Option<&'static str>,

    /// ARIA describedby attribute for accessibility.
    #[props(default)]
    pub aria_describedby: Option<&'static str>,

    /// CSS class for the container wrapping the slider.
    #[props(default = "slider-container")]
    pub container_class: &'static str,

    /// Inline style for the container wrapping the slider.
    #[props(
        default = "display: flex; flex-direction: column; align-items: center; margin: 20px; position: relative;"
    )]
    pub container_style: &'static str,

    /// CSS class for the slider label.
    #[props(default = "slider-label")]
    pub label_class: &'static str,

    /// Inline style for the slider label.
    #[props(default = "font-size: 14px; margin-bottom: 8px;")]
    pub label_style: &'static str,

    /// CSS class for the slider input element.
    #[props(default = "slider-input")]
    pub input_class: &'static str,

    /// Inline style for the slider input element.
    #[props(default = "border-radius: 8px; appearance: none; outline: none;")]
    pub input_style: &'static str,

    /// CSS class for the value/output display.
    #[props(default = "slider-output")]
    pub output_class: &'static str,

    /// Inline style for the value/output display.
    #[props(default = "font-size: 12px; margin-top: 8px;")]
    pub output_style: &'static str,

    /// Inline style for the tooltip element.
    #[props(
        default = "background-color: #333; color: #fff; padding: 4px 8px; border-radius: 4px; font-size: 12px; display: none;"
    )]
    pub tooltip_style: &'static str,

    /// Inline style for the steps indicator below the slider track.
    #[props(
        default = "width: 100%; display: flex; justify-content: space-between; margin-top: 8px; font-size: 10px;"
    )]
    pub steps_style: &'static str,

    /// Custom width for the slider track.
    #[props(default)]
    pub slider_width: Width,

    /// Custom height for the slider track.
    #[props(default)]
    pub slider_height: Height,

    /// Optional custom CSS for the slider thumb.
    #[props(default)]
    pub custom_thumb_css: Option<&'static str>,

    /// Optional custom HTML content for the slider thumb.
    #[props(default)]
    pub custom_thumb_html: Option<Element>,

    /// Keyboard step increment for arrow key adjustments.
    #[props(default = 1.0)]
    pub keyboard_step: f64,

    /// Optional icon element displayed before the slider.
    #[props(default)]
    pub icon_start: Option<Element>,

    /// Optional icon element displayed after the slider.
    #[props(default)]
    pub icon_end: Option<Element>,
}

/// Slider Component
///
/// A Dioxus range slider component with extensive customization and accessibility support.
///
/// # Features
/// - Single value or double range mode (TODO).
/// - Horizontal and vertical orientations.
/// - Customizable track, thumb, tooltip, and step indicators.
/// - Full ARIA support for accessibility.
/// - Keyboard controls with adjustable increments.
///
/// # Examples
///
/// ## Single Value Slider
/// ```rust
/// use dioxus::prelude::*;
/// use slider_rs::dioxus::Slider;
///
/// fn app() -> Element {
///     rsx! {
///         Slider {
///             label: "Volume",
///             min: 0.0,
///             max: 100.0,
///             step: 1.0,
///             show_value: true,
///             on_change: Callback::new(move |v| log::info!("Value: {}", v)),
///         }
///     }
/// }
/// ```
///
/// ## Double Range Slider
/// ```rust
/// use dioxus::prelude::*;
/// use slider_rs::dioxus::Slider;
///
/// fn app() -> Element {
///     rsx! {
///         Slider {
///             label: "Select Range",
///             min: 0.0,
///             max: 100.0,
///             step: 5.0,
///             range: Some((20.0, 80.0)),
///             double: true,
///             show_value: true,
///             on_change_range: Callback::new(move |(start, end)| log::info!("Range: {} - {}", start, end)),
///         }
///     }
/// }
/// ```
///
/// # Notes
/// - Subcomponents include: `Label`, `Input`, `Ticks`, `Output`, `Steps`.
/// - Double slider mode renders two overlapping input elements.
/// - Styling is fully customizable via `*_style` and `*_class` props.
///
/// # Accessibility
/// - Supports ARIA roles, orientation, and description attributes.
/// - Keyboard navigation with configurable step increments.
///
/// # See Also
/// - [MDN <input type="range"> Element](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input/range)
#[component]
pub fn Slider(props: SliderProps) -> Element {
    let mut val1 = use_signal(|| props.range.unwrap_or((props.min, props.max)).0);
    let mut val2 = use_signal(|| props.range.unwrap_or((props.min, props.max)).1);

    let input_ref1: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    let input_ref2: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    let list_id = use_memo(|| format!("slider-list-{}", Uuid::new_v4()));

    let update_range = {
        Callback::new(move |_| {
            props.on_change_range.call((val1(), val2()));
            props.on_change.call(val1());
        })
    };

    let on_input1 = {
        Callback::new(move |e: FormEvent| {
            if let Ok(input) = e.value().parse::<f64>() {
                val1.set(input);
                update_range.call(());
                props.on_change.call(input);
            }
        })
    };

    let on_input2 = {
        Callback::new(move |e: FormEvent| {
            if let Ok(input) = e.value().parse::<f64>() {
                val2.set(input);
                update_range.call(());
                props.on_change.call(input);
            }
        })
    };

    let on_focus_cb = { Callback::new(move |_e: FocusEvent| props.on_focus.call(())) };

    let on_blur_cb = { Callback::new(move |_e: FocusEvent| props.on_blur.call(())) };

    let (input_style1, input_style2): (&'static str, &'static str) = if props.double {
        let flipped_style = Box::leak(Box::new(format!(
            "{}; transform: rotate(0deg); direction: rtl; z-index: 3; position: relative; flex: 1;",
            props.input_style
        )));
        let normal_style = Box::leak(Box::new(format!(
            "{}; z-index: 2; position: relative; flex: 1;",
            props.input_style
        )));
        (flipped_style, normal_style)
    } else {
        (props.input_style, props.input_style)
    };

    let orientation_attr = if props.orientation.is_vertical() {
        "vertical"
    } else {
        "horizontal"
    };

    let steps_component = if props.show_steps {
        rsx! {
            Ticks { id: list_id().clone(), min: props.min, max: props.max, step: props.step }
            Steps {
                min: props.min,
                max: props.max,
                step: props.step,
                steps_style: props.steps_style,
                orientation: props.orientation.clone()
            }
        }
    } else {
        rsx! {}
    };

    let double_input = if props.double {
        rsx! {
            Input {
                input_ref: input_ref2,
                min: props.min,
                max: props.max,
                step: props.step,
                value: val2(),
                orientation: props.orientation.clone(),
                disabled: props.disabled,
                size: props.size.clone(),
                color: props.color.clone(),
                cursor_style: props.cursor_style.clone(),
                input_class: props.input_class,
                input_style: input_style2,
                on_input: on_input2,
                on_focus: on_focus_cb,
                on_blur: on_blur_cb,
                datalist_id: Some(list_id().clone()),
                aria_label: props.aria_label,
                aria_describedby: props.aria_describedby,
                width: props.slider_width.clone(),
                height: props.slider_height.clone(),
                custom_thumb_css: props.custom_thumb_css,
                custom_thumb_html: props.custom_thumb_html.clone(),
                keyboard_step: props.keyboard_step,
            }
        }
    } else {
        rsx! {}
    };

    let value_display = if props.show_value {
        rsx! {
            Output {
                value_display: format!("{:.1}", val1()),
                output_class: props.output_class,
                output_style: props.output_style,
                tooltip_style: props.tooltip_style,
                show_tooltip: props.show_tooltip,
                tooltip_left: format!("{:.2}%", ((val1() - props.min) / (props.max - props.min)) * 100.0),
            }
        }
    } else {
        rsx! {}
    };

    let input_group = if props.orientation.is_vertical() {
        rsx! {
            div {
                style: "display: flex; flex-direction: row; align-items: flex-start;",
                {props.icon_start.unwrap_or(rsx!{})}
                Input {
                    input_ref: input_ref1,
                    min: props.min,
                    max: props.max,
                    step: props.step,
                    value: val1(),
                    orientation: props.orientation.clone(),
                    disabled: props.disabled,
                    size: props.size,
                    color: props.color,
                    cursor_style: props.cursor_style,
                    input_class: props.input_class,
                    input_style: input_style1,
                    on_input: on_input1,
                    on_focus: on_focus_cb,
                    on_blur: on_blur_cb,
                    datalist_id: Some(list_id()),
                    aria_label: props.aria_label,
                    aria_describedby: props.aria_describedby,
                    width: props.slider_width,
                    height: props.slider_height,
                    custom_thumb_css: props.custom_thumb_css,
                    custom_thumb_html: props.custom_thumb_html,
                    keyboard_step: props.keyboard_step,
                }
                {double_input}
                {props.icon_end.unwrap_or(rsx!{})}
                {steps_component}
            }
        }
    } else if props.double {
        rsx! {
            div {
                style: "position: relative; width: 100%; display: flex; align-items: center;",
                {props.icon_start.unwrap_or(rsx!{})}
                Input {
                    input_ref: input_ref1,
                    min: props.min,
                    max: props.max,
                    step: props.step,
                    value: val1(),
                    orientation: props.orientation.clone(),
                    disabled: props.disabled,
                    size: props.size,
                    color: props.color.clone(),
                    cursor_style: props.cursor_style.clone(),
                    input_class: props.input_class,
                    input_style: input_style1,
                    on_input: on_input1,
                    on_focus: on_focus_cb,
                    on_blur: on_blur_cb,
                    datalist_id: Some(list_id().clone()),
                    aria_label: props.aria_label,
                    aria_describedby: props.aria_describedby,
                    width: props.slider_width.clone(),
                    height: props.slider_height.clone(),
                    custom_thumb_css: props.custom_thumb_css,
                    custom_thumb_html: props.custom_thumb_html.clone(),
                    keyboard_step: props.keyboard_step,
                }
                {double_input}
                {props.icon_end.clone().unwrap_or(rsx!{})}
            }
        }
    } else {
        rsx! {
            div {
                style: "display: flex; align-items: center; width: 100%;",
                {props.icon_start.clone().unwrap_or(rsx!{})}
                Input {
                    input_ref: input_ref1,
                    min: props.min,
                    max: props.max,
                    step: props.step,
                    value: val1(),
                    orientation: props.orientation.clone(),
                    disabled: props.disabled,
                    size: props.size.clone(),
                    color: props.color.clone(),
                    cursor_style: props.cursor_style.clone(),
                    input_class: props.input_class,
                    input_style: input_style1,
                    on_input: on_input1,
                    on_focus: on_focus_cb,
                    on_blur: on_blur_cb,
                    datalist_id: Some(list_id().clone()),
                    aria_label: props.aria_label,
                    aria_describedby: props.aria_describedby,
                    width: props.slider_width.clone(),
                    height: props.slider_height.clone(),
                    custom_thumb_css: props.custom_thumb_css,
                    custom_thumb_html: props.custom_thumb_html.clone(),
                    keyboard_step: props.keyboard_step,
                }
                {props.icon_end.clone().unwrap_or(rsx!{})}
            }
        }
    };

    let horizontal_steps = if props.show_steps && !props.orientation.is_vertical() {
        rsx! {
            Steps {
                min: props.min,
                max: props.max,
                step: props.step,
                steps_style: props.steps_style,
                orientation: props.orientation.clone()
            }
        }
    } else {
        rsx! {}
    };

    rsx! {
        div {
            class: "{props.container_class}",
            style: "{props.container_style}",
            role: "group",
            aria_orientation: "{orientation_attr}",
            aria_disabled: "{props.disabled}",
            Label {
                label: props.label,
                label_class: props.label_class,
                label_style: props.label_style
            }
            {input_group}
            Ticks { id: list_id().clone(), min: props.min, max: props.max, step: props.step }
            {value_display}
            {horizontal_steps}
        }
    }
}
