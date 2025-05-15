# 🧬 Slider RS Dioxus Usage

Adding Slider RS to your project is simple:

1. Make sure your project is set up with **Dioxus**. Refer to the [Dioxus Getting Started Guide](https://dioxuslabs.com/learn/0.6/getting_started) for setup instructions.

1. Add the **Slider RS** library to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add slider-rs --features=dio
   ```

1. Import the `Slider` component into your Dioxus application.

## 🛠️ Usage

Incorporate the `Slider` component into your Dioxus application by following these steps:

### 1. Import the `Slider` Component

```rust
use dioxus::prelude::*;
use slider_rs::dioxus::Slider;
use slider_rs::Orientation;
```

### 2. Use the `Slider` in Your Component

```rust
use dioxus::prelude::*;
use slider_rs::dioxus::Slider;
use slider_rs::Orientation;

#[component]
fn App() -> Element {
    let mut value = use_signal(|| 50.0);

    let on_change = {
        Callback::new(move |val: f64| value.set(val))
    };

    rsx! {
        Slider {
            min: 0.0,
            max: 100.0,
            step: 1.0,
            value: Some(value()),
            on_change: on_change,
            orientation: Orientation::Horizontal,
            show_value: true,
            show_steps: true,
        }
    }
}
```

## 🔧 Props

### Main Props

| Property       | Type                | Description                                  | Default                                     |
| -------------- | ------------------- | -------------------------------------------- | ------------------------------------------- |
| `label`        | `&'static str`      | Label text displayed above the slider.       | `""`                                        |
| `min`          | `f64`               | Minimum value for the slider.                | `0.0`                                       |
| `max`          | `f64`               | Maximum value for the slider.                | `10.0`                                      |
| `step`         | `f64`               | Increment step size.                         | `1.0`                                       |
| `value`        | `Option<f64>`       | Current value for single slider mode.        | `None`                                      |
| `range`        | `Option<(f64,f64)>` | Current range for double slider mode.        | `None`                                      |
| `double`       | `bool`              | Enables double slider mode (range selector). | `false`                                     |
| `orientation`  | `Orientation`       | Slider orientation: horizontal or vertical.  | `Orientation::Horizontal` (assumed default) |
| `size`         | `Size`              | Size variant for styling the slider.         | `Size::Default` (assumed)                   |
| `color`        | `Color`             | Color variant for styling the slider.        | `Color::Default` (assumed)                  |
| `cursor_style` | `Cursor`            | Cursor style when hovering over the slider.  | `Cursor::Default` (assumed)                 |

### Behavioral Props

| Property          | Type                  | Description                                          | Default |
| ----------------- | --------------------- | ---------------------------------------------------- | ------- |
| `show_value`      | `bool`                | Show the current value as output below the slider.   | `false` |
| `show_steps`      | `bool`                | Show step ticks along the slider track.              | `false` |
| `show_tooltip`    | `bool`                | Show tooltip above thumb on hover.                   | `false` |
| `disabled`        | `bool`                | Disable interaction with the slider.                 | `false` |
| `on_change`       | `Callback<f64>`       | Callback triggered when single value changes.        | No-op   |
| `on_change_range` | `Callback<(f64,f64)>` | Callback triggered when range changes (double mode). | No-op   |
| `on_focus`        | `Callback<()>`        | Callback triggered on slider focus.                  | No-op   |
| `on_blur`         | `Callback<()>`        | Callback triggered on slider blur.                   | No-op   |
| `keyboard_step`   | `f64`                 | Keyboard arrow key increment step size.              | `1.0`   |

### Accessibility Props

| Property           | Type                   | Description                                         | Default |
| ------------------ | ---------------------- | --------------------------------------------------- | ------- |
| `aria_label`       | `Option<&'static str>` | ARIA label for screen readers.                      | `None`  |
| `aria_describedby` | `Option<&'static str>` | ARIA describedby attribute for accessibility hints. | `None`  |

### Styling & Layout Props

| Property          | Type           | Description                                            | Default                                                                                                        |
| ----------------- | -------------- | ------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------- |
| `container_class` | `&'static str` | CSS class for the container wrapping the slider.       | `"slider-container"`                                                                                           |
| `container_style` | `&'static str` | Inline style for the container wrapping the slider.    | `"display: flex; flex-direction: column; align-items: center; margin: 20px; position: relative;"`              |
| `label_class`     | `&'static str` | CSS class for the slider label.                        | `"slider-label"`                                                                                               |
| `label_style`     | `&'static str` | Inline style for the slider label.                     | `"font-size: 14px; margin-bottom: 8px;"`                                                                       |
| `input_class`     | `&'static str` | CSS class for the slider input element.                | `"slider-input"`                                                                                               |
| `input_style`     | `&'static str` | Inline style for the slider input element.             | `"border-radius: 8px; appearance: none; outline: none;"`                                                       |
| `output_class`    | `&'static str` | CSS class for the value/output display.                | `"slider-output"`                                                                                              |
| `output_style`    | `&'static str` | Inline style for the value/output display.             | `"font-size: 12px; margin-top: 8px;"`                                                                          |
| `tooltip_style`   | `&'static str` | Inline style for the tooltip element.                  | `"background-color: #333; color: #fff; padding: 4px 8px; border-radius: 4px; font-size: 12px; display: none;"` |
| `steps_style`     | `&'static str` | Inline style for the steps indicator below the slider. | `"width: 100%; display: flex; justify-content: space-between; margin-top: 8px; font-size: 10px;"`              |

### Track & Thumb Customization Props

| Property            | Type                   | Description                                | Default                     |
| ------------------- | ---------------------- | ------------------------------------------ | --------------------------- |
| `slider_width`      | `Width`                | Custom width for the slider track.         | `Width::Default` (assumed)  |
| `slider_height`     | `Height`               | Custom height for the slider track.        | `Height::Default` (assumed) |
| `custom_thumb_css`  | `Option<&'static str>` | Optional custom CSS for the slider thumb.  | `None`                      |
| `custom_thumb_html` | `Option<Element>`      | Optional custom HTML content inside thumb. | `None`                      |

### Icon Props

| Property     | Type              | Description                            | Default |
| ------------ | ----------------- | -------------------------------------- | ------- |
| `icon_start` | `Option<Element>` | Optional icon displayed before slider. | `None`  |
| `icon_end`   | `Option<Element>` | Optional icon displayed after slider.  | `None`  |

## 💡 Notes

- `value` is for single sliders; `range` is for double sliders (`double: true`).
- Props like `size`, `color`, `cursor_style` are used for visual styling presets.
- The component supports full accessibility with ARIA attributes.
- Callbacks (`on_change`, `on_change_range`, `on_focus`, `on_blur`) help manage state and interactions.
- Styles and classes can be customized extensively via provided props.
- Tooltips, step marks, and icons are optional features to enhance the UI.
