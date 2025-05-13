# Slider RS Yew Usage

Adding Slider RS to your project is simple:

1. Make sure your project is set up with **Yew**. Follow their [Getting Started Guide](https://yew.rs/docs/getting-started/introduction) for setup instructions.

1. Add the Slider RS component to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add slider-rs --features=yew
   ```

1. Import the `Slider` component into your Yew component and start using it in your app.

## 🛠️ Usage

Incorporating Slider RS into your Yew application is easy. Follow these steps:

1. Import the `Slider` component into your Yew project:

   ```rust
   use yew::prelude::*;
   use slider_rs::yew::Slider;
   ```

1. Use the `Slider` component within your Yew application:

   ```rust
   use yew::prelude::*;
   use slider_rs::yew::Slider;
   use slider_rs::Orientation;

   #[function_component(App)]
   pub fn app() -> Html {
       let min = use_state(|| 10.0);
       let max = use_state(|| 90.0);
       let value = use_state(|| 50.0);

       html! {
           <Slider
               min={*min}
               max={*max}
               step={1.0}
               value={Some(*value)}
               on_change={Callback::from(|val| log::info!("Slider changed to: {}", val))}
               orientation={Orientation::Horizontal}
               show_value=true
               show_steps=true
           />
       }
   }
   ```

## 🔧 Props

### `Slider` Component Props

#### Main Props

| Property   | Type                 | Description                                            | Default |
| ---------- | -------------------- | ------------------------------------------------------ | ------- |
| `label`    | `&'static str`       | Label text displayed above the slider.                 | `""`    |
| `min`      | `f64`                | The minimum value of the slider.                       | `0.0`   |
| `max`      | `f64`                | The maximum value of the slider.                       | `10.0`  |
| `step`     | `f64`                | The step size between slider values.                   | `1.0`   |
| `value`    | `Option<f64>`        | The current value of the slider (single mode).         | `None`  |
| `range`    | `Option<(f64, f64)>` | The current range values (start, end) in double mode.  | `None`  |
| `double`   | `bool`               | Enables double slider mode (range selector).           | `false` |
| `disabled` | `bool`               | Disables interaction with the slider if set to `true`. | `false` |

#### Styling & Layout Props

```sh
+---------------------------------------------------------------+
|                     [Slider Container]                        |  <-- `container_class` & `container_style`
|                                                               |
|   +------------------------[Label]------------------------+   |  <-- `label_class` & `label_style`
|   |                                                       |   |
|   +-------------------------------------------------------+   |
|                                                               |
|   +----------------[Icons & Inputs Wrapper]---------------+   |  <-- layout wrapper
|   |  [Icon Start]   [Input Thumb 1]   [Input Thumb 2]     |   |  <-- double thumb if enabled
|   |                                                       |   |
|   +-------------------------------------------------------+   |
|                                                               |
|   +------------------------[Ticks]------------------------+   |  <-- visual tick marks (if enabled)
|                                                               |
|   +------------------------[Output]-----------------------+   |  <-- value display (if `show_value`)
|                                                               |
|   +------------------------[Steps]------------------------+   |  <-- step indicators (if `show_steps`)
+---------------------------------------------------------------+
```

| Property            | Type                   | Description                                            | Default                              |
| ------------------- | ---------------------- | ------------------------------------------------------ | ------------------------------------ |
| `orientation`       | `Orientation`          | Orientation of the slider: `Horizontal` or `Vertical`. | `Horizontal`                         |
| `size`              | `Size`                 | Size variant for the slider appearance.                | `Default`                            |
| `color`             | `Color`                | Color theme variant for styling the slider.            | `Default`                            |
| `cursor_style`      | `Cursor`               | Cursor style when hovering over the slider.            | `Default`                            |
| `container_class`   | `&'static str`         | CSS class for the outer container.                     | `"slider-container"`                 |
| `container_style`   | `&'static str`         | Inline style for the outer container.                  | `flex column center layout`          |
| `label_class`       | `&'static str`         | CSS class for the label element.                       | `"slider-label"`                     |
| `label_style`       | `&'static str`         | Inline style for the label element.                    | `font-size, margin`                  |
| `input_class`       | `&'static str`         | CSS class for the slider input element.                | `"slider-input"`                     |
| `input_style`       | `&'static str`         | Inline style for the slider input element.             | `border-radius, appearance, outline` |
| `output_class`      | `&'static str`         | CSS class for the output value display.                | `"slider-output"`                    |
| `output_style`      | `&'static str`         | Inline style for the output value display.             | `font-size, margin`                  |
| `tooltip_style`     | `&'static str`         | Inline style for the tooltip element above the thumb.  | `dark background tooltip styling`    |
| `steps_style`       | `&'static str`         | Inline style for the step indicators below the track.  | `flex spaced indicators`             |
| `slider_width`      | `Width`                | Custom width for the slider track.                     | `Default`                            |
| `slider_height`     | `Height`               | Custom height for the slider track.                    | `Default`                            |
| `custom_thumb_css`  | `Option<&'static str>` | Custom CSS applied to the slider thumb.                | `None`                               |
| `custom_thumb_html` | `Option<Html>`         | Custom HTML content inside the slider thumb.           | `None`                               |
| `icon_start`        | `Option<Html>`         | Optional icon displayed before the slider track.       | `None`                               |
| `icon_end`          | `Option<Html>`         | Optional icon displayed after the slider track.        | `None`                               |

#### Behavioral Props

| Property          | Type                   | Description                                            | Default |
| ----------------- | ---------------------- | ------------------------------------------------------ | ------- |
| `show_value`      | `bool`                 | Whether to display the current value below the slider. | `false` |
| `show_steps`      | `bool`                 | Whether to display step indicators below the slider.   | `false` |
| `show_tooltip`    | `bool`                 | Whether to show a tooltip on hover above the thumb.    | `false` |
| `on_change`       | `Callback<f64>`        | Callback when slider value changes (single mode).      | No-op   |
| `on_change_range` | `Callback<(f64, f64)>` | Callback when range changes (double mode).             | No-op   |
| `on_focus`        | `Callback<()>`         | Callback triggered when slider gains focus.            | No-op   |
| `on_blur`         | `Callback<()>`         | Callback triggered when slider loses focus.            | No-op   |
| `keyboard_step`   | `f64`                  | Increment step for keyboard arrow key adjustments.     | `1.0`   |

#### Accessibility Props

| Property           | Type                   | Description                                         | Default |
| ------------------ | ---------------------- | --------------------------------------------------- | ------- |
| `aria_label`       | `Option<&'static str>` | ARIA label for screen readers.                      | `None`  |
| `aria_describedby` | `Option<&'static str>` | ARIA describedby attribute for accessibility hints. | `None`  |

## 💡 Notes

- `value` is for single sliders; `range` is for double sliders (set `double: true`).
- Props like `size`, `color`, `cursor_style` are used for visual styling presets.
- The component is **accessible** with proper ARIA support.
- **Callbacks** like `on_change`, `on_input` (implicit), and focus events help in state handling.
- Inline styles and CSS classes allow full **custom styling**.
- Tooltips, ticks, steps, and icons are **optional add-ons** for richer UI.
