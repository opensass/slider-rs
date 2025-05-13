#![doc = include_str!("../YEW.md")]

use crate::common::{Color, Cursor, Height, Orientation, Size, Width};
use uuid::Uuid;
use web_sys::{FocusEvent, HtmlInputElement, InputEvent, KeyboardEvent};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LabelProps {
    #[prop_or_default]
    pub label: &'static str,
    #[prop_or("font-size: 14px; margin-bottom: 8px; text-align: center;")]
    pub label_style: &'static str,
    #[prop_or("slider-label")]
    pub label_class: &'static str,
}

#[function_component(Label)]
fn slider_label(props: &LabelProps) -> Html {
    html! { <label class={props.label_class} style={props.label_style}>{ props.label }</label> }
}

#[derive(Properties, PartialEq)]
pub struct StepsProps {
    #[prop_or(0.0)]
    pub min: f64,
    #[prop_or(10.0)]
    pub max: f64,
    #[prop_or(1.0)]
    pub step: f64,
    #[prop_or(
        "width: 100%; display: flex; justify-content: space-between; margin-top: 8px; font-size: 10px;"
    )]
    pub steps_style: &'static str,
    #[prop_or_default]
    pub orientation: Orientation,
}

#[function_component(Steps)]
fn slider_steps(props: &StepsProps) -> Html {
    let count = ((props.max - props.min) / props.step).floor() as usize;

    let steps = (0..=count)
        .map(|i| {
            let val = props.min + (i as f64 * props.step);
            html! {
                <span
                    style={if props.orientation.is_vertical() {
                    "margin: 4px 0; writing-mode: vertical-rl; text-align: center;"
                } else {
                    "text-align: center;"
                }}
                >
                    { format!("{:.0}", val) }
                </span>
            }
        })
        .collect::<Html>();

    let style = if props.orientation.is_vertical() {
        "display: flex; flex-direction: column; align-items: center; height: 100%; font-size: 10px;"
    } else {
        props.steps_style
    };

    html! { <div style={style}>{ steps }</div> }
}

#[derive(Properties, PartialEq)]
pub struct OutputProps {
    #[prop_or_default]
    pub value_display: String,
    #[prop_or("font-size: 12px; margin-top: 8px; text-align: center;")]
    pub output_style: &'static str,
    #[prop_or("slider-output")]
    pub output_class: &'static str,
    #[prop_or(
        "background-color: #333; color: #fff; padding: 4px 8px; border-radius: 4px; font-size: 12px; position: absolute; transform: translate(-50%, -120%); display: block; pointer-events: none;"
    )]
    pub tooltip_style: &'static str,
    #[prop_or(false)]
    pub show_tooltip: bool,
    #[prop_or_default]
    pub tooltip_left: String,
}

#[function_component(Output)]
fn slider_output(props: &OutputProps) -> Html {
    html! {
        <>
            <output class={props.output_class} style={props.output_style} aria-live="polite">
                { &props.value_display }
            </output>
            { if props.show_tooltip {
                    let mut style = props.tooltip_style.to_string();
                    style.push_str(&format!(" left: {}; ", props.tooltip_left));
                    html! {
                        <div class={props.output_class} style={style}>
                            { &props.value_display }
                        </div>
                    }
                } else {
                    html! {}
                } }
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct TicksProps {
    #[prop_or_default]
    pub id: String,
    #[prop_or(0.0)]
    pub min: f64,
    #[prop_or(10.0)]
    pub max: f64,
    #[prop_or(1.0)]
    pub step: f64,
}

#[function_component(Ticks)]
fn slider_ticks(props: &TicksProps) -> Html {
    let mut current = props.min;
    let mut children = vec![];

    while current <= props.max {
        children.push(html! { <option value={current.to_string()} /> });
        current += props.step;
    }

    html! { <datalist id={props.id.clone()}>{ for children }</datalist> }
}

#[derive(Properties, PartialEq)]
pub struct InputProps {
    #[prop_or_default]
    pub input_ref: NodeRef,
    #[prop_or(0.0)]
    pub min: f64,
    #[prop_or(10.0)]
    pub max: f64,
    #[prop_or(1.0)]
    pub step: f64,
    #[prop_or(0.0)]
    pub value: f64,
    #[prop_or_default]
    pub orientation: Orientation,
    #[prop_or_default]
    pub size: Size,
    #[prop_or_default]
    pub width: Width,
    #[prop_or_default]
    pub height: Height,
    #[prop_or_default]
    pub color: Color,
    #[prop_or_default]
    pub cursor_style: Cursor,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub on_input: Callback<InputEvent>,
    #[prop_or_default]
    pub on_focus: Callback<FocusEvent>,
    #[prop_or_default]
    pub on_blur: Callback<FocusEvent>,
    #[prop_or_default]
    pub aria_label: Option<&'static str>,
    #[prop_or_default]
    pub aria_describedby: Option<&'static str>,
    #[prop_or_default]
    pub datalist_id: Option<String>,
    #[prop_or("slider-input")]
    pub input_class: &'static str,
    #[prop_or("border-radius: 8px; appearance: none; outline: none;")]
    pub input_style: &'static str,
    #[prop_or(true)]
    pub use_gradient: bool,
    #[prop_or_default]
    pub custom_thumb_css: Option<&'static str>,
    #[prop_or_default]
    pub custom_thumb_html: Option<Html>,
    #[prop_or(1.0)]
    pub keyboard_step: f64,
}

#[function_component(Input)]
fn slider_input(props: &InputProps) -> Html {
    let value_percent = ((props.value - props.min) / (props.max - props.min)) * 100.0;
    let fill_color = props.color.to_color_code();
    let gradient = if props.use_gradient {
        if props.orientation.is_vertical() {
            format!(
                "background: linear-gradient(to bottom, {} 0%, {} {:.2}%, #ccc {:.2}%, #ccc 100%);",
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

    let thumb_style = props.custom_thumb_css.unwrap_or("");

    let on_key_down = {
        let on_input = props.on_input.clone();
        let input_ref = props.input_ref.clone();
        let keyboard_step = props.keyboard_step;
        let min = props.min;
        let max = props.max;
        Callback::from(move |e: KeyboardEvent| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let current = input.value().parse::<f64>().unwrap_or(0.0);
                let new_val = match e.key().as_str() {
                    "ArrowLeft" | "ArrowDown" => current - keyboard_step,
                    "ArrowRight" | "ArrowUp" => current + keyboard_step,
                    _ => current,
                }
                .clamp(min, max);
                input.set_value(&new_val.to_string());

                on_input.emit(InputEvent::new("input").unwrap());
            }
        })
    };

    html! {
        <>
            <input
                ref={props.input_ref.clone()}
                type="range"
                class={props.input_class}
                min={props.min.to_string()}
                max={props.max.to_string()}
                step={if props.step == 0.0 { "any".to_string() } else { props.step.to_string() }}
                value={props.value.to_string()}
                list={props.datalist_id.clone()}
                oninput={props.on_input.clone()}
                onfocus={props.on_focus.clone()}
                onblur={props.on_blur.clone()}
                onkeydown={on_key_down}
                style={format!("{} {}", base_style, thumb_style)}
                orient={props.orientation.to_orient()}
                disabled={props.disabled}
                aria-valuemin={props.min.to_string()}
                aria-valuemax={props.max.to_string()}
                aria-valuenow={props.value.to_string()}
                aria-orientation={if props.orientation.is_vertical() { "vertical" } else { "horizontal" }}
                aria-disabled={props.disabled.to_string()}
                aria-label={props.aria_label.unwrap_or_default()}
                aria-describedby={props.aria_describedby.unwrap_or_default()}
            />
            { props.custom_thumb_html.clone().unwrap_or(html! {}) }
        </>
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
#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    /// Label text displayed above the slider.
    #[prop_or_default]
    pub label: &'static str,

    /// Minimum value for the slider.
    #[prop_or(0.0)]
    pub min: f64,

    /// Maximum value for the slider.
    #[prop_or(10.0)]
    pub max: f64,

    /// Increment step size.
    #[prop_or(1.0)]
    pub step: f64,

    /// Current value for a single slider mode.
    #[prop_or_default]
    pub value: Option<f64>,

    /// Current range values (start, end) for double slider mode.
    #[prop_or_default]
    pub range: Option<(f64, f64)>,

    /// Whether to enable double slider mode (range selector).
    #[prop_or(false)]
    pub double: bool,

    /// Slider orientation: horizontal or vertical.
    #[prop_or_default]
    pub orientation: Orientation,

    /// Size variant for styling the slider track and thumb.
    #[prop_or_default]
    pub size: Size,

    /// Color variant for styling the slider.
    #[prop_or_default]
    pub color: Color,

    /// Cursor style when hovering over the slider.
    #[prop_or_default]
    pub cursor_style: Cursor,

    /// Whether to show the current value as an output.
    #[prop_or(false)]
    pub show_value: bool,

    /// Whether to show step ticks along the slider track.
    #[prop_or(false)]
    pub show_steps: bool,

    /// Whether to show tooltip above the thumb on hover.
    #[prop_or(false)]
    pub show_tooltip: bool,

    /// Whether to disable interaction with the slider.
    #[prop_or(false)]
    pub disabled: bool,

    /// Callback triggered when the slider value changes.
    #[prop_or_default]
    pub on_change: Callback<f64>,

    /// Callback triggered when the slider range changes (double mode).
    #[prop_or_default]
    pub on_change_range: Callback<(f64, f64)>,

    /// Callback triggered when the slider gains focus.
    #[prop_or_default]
    pub on_focus: Callback<()>,

    /// Callback triggered when the slider loses focus.
    #[prop_or_default]
    pub on_blur: Callback<()>,

    /// ARIA label for accessibility.
    #[prop_or_default]
    pub aria_label: Option<&'static str>,

    /// ARIA describedby attribute for accessibility.
    #[prop_or_default]
    pub aria_describedby: Option<&'static str>,

    /// CSS class for the container wrapping the slider.
    #[prop_or("slider-container")]
    pub container_class: &'static str,

    /// Inline style for the container wrapping the slider.
    #[prop_or(
        "display: flex; flex-direction: column; align-items: center; margin: 20px; position: relative;"
    )]
    pub container_style: &'static str,

    /// CSS class for the slider label.
    #[prop_or("slider-label")]
    pub label_class: &'static str,

    /// Inline style for the slider label.
    #[prop_or("font-size: 14px; margin-bottom: 8px;")]
    pub label_style: &'static str,

    /// CSS class for the slider input element.
    #[prop_or("slider-input")]
    pub input_class: &'static str,

    /// Inline style for the slider input element.
    #[prop_or("border-radius: 8px; appearance: none; outline: none;")]
    pub input_style: &'static str,

    /// CSS class for the value/output display.
    #[prop_or("slider-output")]
    pub output_class: &'static str,

    /// Inline style for the value/output display.
    #[prop_or("font-size: 12px; margin-top: 8px;")]
    pub output_style: &'static str,

    /// Inline style for the tooltip element.
    #[prop_or(
        "background-color: #333; color: #fff; padding: 4px 8px; border-radius: 4px; font-size: 12px; display: none;"
    )]
    pub tooltip_style: &'static str,

    /// Inline style for the steps indicator below the slider track.
    #[prop_or(
        "width: 100%; display: flex; justify-content: space-between; margin-top: 8px; font-size: 10px;"
    )]
    pub steps_style: &'static str,

    /// Custom width for the slider track.
    #[prop_or_default]
    pub slider_width: Width,

    /// Custom height for the slider track.
    #[prop_or_default]
    pub slider_height: Height,

    /// Optional custom CSS for the slider thumb.
    #[prop_or_default]
    pub custom_thumb_css: Option<&'static str>,

    /// Optional custom HTML content for the slider thumb.
    #[prop_or_default]
    pub custom_thumb_html: Option<Html>,

    /// Keyboard step increment for arrow key adjustments.
    #[prop_or(1.0)]
    pub keyboard_step: f64,

    /// Optional icon element displayed before the slider.
    #[prop_or_default]
    pub icon_start: Option<Html>,

    /// Optional icon element displayed after the slider.
    #[prop_or_default]
    pub icon_end: Option<Html>,
}

/// Slider Component
///
/// A Yew slider (range input) component, supporting both single and double handle sliders.
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
/// use yew::prelude::*;
/// use slider_rs::yew::{Slider};
///
/// #[function_component]
/// fn App() -> Html {
///     html! {
///         <Slider
///             label="Volume"
///             min={0.0}
///             max={100.0}
///             step={1.0}
///             value={Some(50.0)}
///             show_value={true}
///             on_change={Callback::from(|v| log::info!("Value: {}", v))}
///         />
///     }
/// }
/// ```
///
/// ## Double Range Slider
/// ```rust
/// use yew::prelude::*;
/// use slider_rs::yew::{Slider};
///
/// #[function_component]
/// fn App() -> Html {
///     html! {
///         <Slider
///             label="Range"
///             min={0.0}
///             max={100.0}
///             step={1.0}
///             range={Some((20.0, 80.0))}
///             double={true}
///             show_value={true}
///             on_change_range={Callback::from(|(start, end)| log::info!("Range: {} - {}", start, end))}
///         />
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
#[function_component(Slider)]
pub fn slider(props: &Props) -> Html {
    let input_ref1 = use_node_ref();
    let input_ref2 = use_node_ref();
    let (val1, val2) = props.range.unwrap_or((props.min, props.max));
    let val1 = use_state(|| props.value.unwrap_or(val1));
    let val2 = use_state(|| val2);
    let list_id = format!("slider-list-{}", Uuid::new_v4());

    let update_range = {
        let val1 = val1.clone();
        let val2 = val2.clone();
        let on_change_range = props.on_change_range.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |_| {
            on_change_range.emit((*val1, *val2));
            on_change.emit(*val1);
        })
    };

    let on_input1 = {
        let val1 = val1.clone();
        let val2 = val2.clone();
        let update_range = update_range.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                if let Ok(v) = input.value().parse::<f64>() {
                    val1.set(v.min(*val2));
                    update_range.emit(());
                    on_change.emit(*val1);
                }
            }
        })
    };

    let on_input2 = {
        let val1 = val1.clone();
        let val2 = val2.clone();
        let update_range = update_range.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                if let Ok(v) = input.value().parse::<f64>() {
                    val2.set(v.max(*val1));
                    update_range.emit(());
                    on_change.emit(*val1);
                }
            }
        })
    };

    let on_focus_cb = {
        let cb = props.on_focus.clone();
        Callback::from(move |_| cb.emit(()))
    };

    let on_blur_cb = {
        let cb = props.on_blur.clone();
        Callback::from(move |_| cb.emit(()))
    };

    let (input_style1, input_style2): (&'static str, &'static str) = if props.double {
        let stacked_style1 = Box::leak(Box::new(format!(
            "{}; position: absolute; width: 100%; top: 0; left: 0; z-index: 3;",
            props.input_style
        )));
        let stacked_style2 = Box::leak(Box::new(format!(
            "{}; position: absolute; width: 100%; top: 0; left: 0; z-index: 2;",
            props.input_style
        )));
        (stacked_style1, stacked_style2)
    } else {
        (props.input_style, props.input_style)
    };

    html! {
        <div
            class={props.container_class}
            style={props.container_style}
            role="group"
            aria-orientation={if props.orientation.is_vertical() { "vertical" } else { "horizontal" }}
            aria-disabled={props.disabled.to_string()}
        >
            <Label
                label={props.label}
                label_class={props.label_class}
                label_style={props.label_style}
            />
            { if props.orientation.is_vertical() {
                    html! {
                        <div style="display: flex; flex-direction: row; align-items: flex-start;">
                            { props.icon_start.clone().unwrap_or_default() }
                            <Input
                                input_ref={input_ref1}
                                min={props.min}
                                max={props.max}
                                step={props.step}
                                value={*val1}
                                orientation={props.orientation.clone()}
                                disabled={props.disabled}
                                size={props.size.clone()}
                                color={props.color.clone()}
                                cursor_style={props.cursor_style.clone()}
                                input_class={props.input_class}
                                on_input={on_input1}
                                on_focus={on_focus_cb.clone()}
                                on_blur={on_blur_cb.clone()}
                                datalist_id={Some(list_id.clone())}
                                aria_label={props.aria_label}
                                aria_describedby={props.aria_describedby}
                                width={props.slider_width.clone()}
                                height={props.slider_height.clone()}
                                input_style={input_style1}
                                custom_thumb_css={props.custom_thumb_css}
                                custom_thumb_html={props.custom_thumb_html.clone()}
                                keyboard_step={props.keyboard_step}
                            />
                            {
                                if props.double {
                                    html! {
                                        <Input
                                            input_ref={input_ref2}
                                            min={props.min}
                                            max={props.max}
                                            step={props.step}
                                            value={*val2}
                                            orientation={props.orientation.clone()}
                                            disabled={props.disabled}
                                            size={props.size.clone()}
                                            color={props.color.clone()}
                                            cursor_style={props.cursor_style.clone()}
                                            input_class={props.input_class}
                                            on_input={on_input2}
                                            on_focus={on_focus_cb}
                                            on_blur={on_blur_cb}
                                            datalist_id={Some(list_id.clone())}
                                            aria_label={props.aria_label}
                                            aria_describedby={props.aria_describedby}
                                            width={props.slider_width.clone()}
                                            height={props.slider_height.clone()}
                                            input_style={input_style2}
                                            custom_thumb_css={props.custom_thumb_css}
                                            custom_thumb_html={props.custom_thumb_html.clone()}
                                            keyboard_step={props.keyboard_step}
                                        />
                                    }
                                } else {
                                    html! {}
                                }
                            }
                            { props.icon_end.clone().unwrap_or_default() }
                        </div>
                    }
                } else if props.double {
                    html! {
                        <div style="position: relative; width: 100%; flex; align-items: center;">
                            { props.icon_start.clone().unwrap_or_default() }
                            <Input
                                input_ref={input_ref1}
                                min={props.min}
                                max={props.max}
                                step={props.step}
                                value={*val1}
                                orientation={props.orientation.clone()}
                                disabled={props.disabled}
                                size={props.size.clone()}
                                color={props.color.clone()}
                                cursor_style={props.cursor_style.clone()}
                                input_class={props.input_class}
                                on_input={on_input1}
                                on_focus={on_focus_cb.clone()}
                                on_blur={on_blur_cb.clone()}
                                datalist_id={Some(list_id.clone())}
                                aria_label={props.aria_label}
                                aria_describedby={props.aria_describedby}
                                width={props.slider_width.clone()}
                                height={props.slider_height.clone()}
                                input_style={input_style1}
                                custom_thumb_css={props.custom_thumb_css}
                                custom_thumb_html={props.custom_thumb_html.clone()}
                                keyboard_step={props.keyboard_step}
                            />
                            <Input
                                input_ref={input_ref2}
                                min={props.min}
                                max={props.max}
                                step={props.step}
                                value={*val2}
                                orientation={props.orientation.clone()}
                                disabled={props.disabled}
                                size={props.size.clone()}
                                color={props.color.clone()}
                                cursor_style={props.cursor_style.clone()}
                                input_class={props.input_class}
                                on_input={on_input2}
                                on_focus={on_focus_cb}
                                on_blur={on_blur_cb}
                                datalist_id={Some(list_id.clone())}
                                aria_label={props.aria_label}
                                aria_describedby={props.aria_describedby}
                                width={props.slider_width.clone()}
                                height={props.slider_height.clone()}
                                input_style={input_style2}
                                custom_thumb_css={props.custom_thumb_css}
                                custom_thumb_html={props.custom_thumb_html.clone()}
                                keyboard_step={props.keyboard_step}
                            />
                            { props.icon_end.clone().unwrap_or_default() }
                        </div>
                    }
                } else {
                    html! {
                        <div style="display: flex; align-items: center; width: 100%;">
                            { props.icon_start.clone().unwrap_or_default() }
                            <Input
                                input_ref={input_ref1}
                                min={props.min}
                                max={props.max}
                                step={props.step}
                                value={*val1}
                                orientation={props.orientation.clone()}
                                disabled={props.disabled}
                                size={props.size.clone()}
                                color={props.color.clone()}
                                cursor_style={props.cursor_style.clone()}
                                input_class={props.input_class}
                                on_input={on_input1}
                                on_focus={on_focus_cb.clone()}
                                on_blur={on_blur_cb.clone()}
                                datalist_id={Some(list_id.clone())}
                                aria_label={props.aria_label}
                                aria_describedby={props.aria_describedby}
                                width={props.slider_width.clone()}
                                height={props.slider_height.clone()}
                                input_style={input_style1}
                                custom_thumb_css={props.custom_thumb_css}
                                custom_thumb_html={props.custom_thumb_html.clone()}
                                keyboard_step={props.keyboard_step}
                            />
                            { props.icon_end.clone().unwrap_or_default() }
                        </div>
                    }
                } }
            <Ticks id={list_id.clone()} min={props.min} max={props.max} step={props.step} />
            { if props.show_value {
                html! {
                    <Output
                        value_display={format!("{:.1}", *val1)}
                        output_class={props.output_class}
                        output_style={props.output_style}
                        tooltip_style={props.tooltip_style}
                        show_tooltip={props.show_tooltip}
                        tooltip_left={format!("{:.2}%", ((*val1 - props.min) / (props.max - props.min)) * 100.0)}
                    />
                }
            } else {
                html! {}
            } }
            { if props.show_steps && !props.orientation.is_vertical() {
                html! {
                    <Steps
                        min={props.min}
                        max={props.max}
                        step={props.step}
                        steps_style={props.steps_style}
                        orientation={props.orientation.clone()}
                    />
                }
            } else {
                html! {}
            } }
        </div>
    }
}
