#[macro_export] macro_rules! make_text_input {
    ($app:expr, $x_offset:expr, $y_offset:expr, $label_text:expr, $input_widget:expr, $focus:expr) => {{
        let mut label = Default::default();
        let offset = $y_offset;

        Label::builder()
            .size((100, 35))
            .position(($x_offset + 190, offset + 5))
            .parent(&$app.window)
            .text(&format!("{}:", $label_text))
            .h_align(HTextAlign::Center)
            .build(&mut label)?;
        $app.widgets.labels.push(label);

        if let Ok(mut widget) = $input_widget.lock() {
            TextInput::builder()
                .size((185, 35))
                .position(($x_offset + 300, offset))
                .parent(&$app.window)
                .focus($focus)
                .build(&mut widget)?;
        }
    }};
}

#[macro_export] macro_rules! load_bitmap {
    ($embed:expr, $id:expr, $dimensions:expr) => {{
        $embed.bitmap_str($id, Some($dimensions)).as_ref()
    }};
    ($embed:expr, $id:expr) => {{
        $embed.bitmap_str($id, None).as_ref()
    }};
}

#[macro_export] macro_rules! load_icon {
    ($embed:expr, $id:expr, $dimensions:expr) => {{
        $embed.icon_str($id, Some($dimensions)).as_ref()
    }};
    ($embed:expr, $id:expr) => {{
        $embed.icon_str($id, None).as_ref()
    }};
}

#[macro_export] macro_rules! load_wave {
    ($embed:expr, $id:expr) => {{
        $embed.wave_str($id, None).as_ref()
    }};
}
