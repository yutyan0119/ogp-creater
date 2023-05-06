use image::imageops::overlay;
use image::{ImageBuffer, Rgba};
use imageproc::drawing::draw_filled_circle_mut;
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::env;

fn create_background(width: u32, height: u32, color: Rgba<u8>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    ImageBuffer::from_fn(width, height, |_, _| color)
}

fn draw_rounded_white_rect(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let rect = imageproc::rect::Rect::at(50, 100).of_size(1100, 500);

    let white = Rgba([255, 255, 255, 255]);
    let corner_radius = 10;
    draw_filled_circle_mut(
        image,
        (
            rect.left() + corner_radius as i32,
            rect.top() + corner_radius as i32,
        ),
        corner_radius,
        white,
    );
    draw_filled_circle_mut(
        image,
        (
            rect.right() - corner_radius as i32,
            rect.top() + corner_radius as i32,
        ),
        corner_radius,
        white,
    );
    draw_filled_circle_mut(
        image,
        (
            rect.left() + corner_radius as i32,
            rect.bottom() - corner_radius as i32,
        ),
        corner_radius,
        white,
    );
    draw_filled_circle_mut(
        image,
        (
            rect.right() - corner_radius as i32,
            rect.bottom() - corner_radius as i32,
        ),
        corner_radius,
        white,
    );

    let top_rect = imageproc::rect::Rect::at(rect.left(), rect.top() + corner_radius as i32)
        .of_size(rect.width(), rect.height() - 2 * corner_radius as u32);
    let left_rect = imageproc::rect::Rect::at(rect.left() + corner_radius as i32, rect.top())
        .of_size(rect.width() - 2 * corner_radius as u32, rect.height());
    draw_filled_rect_mut(image, top_rect, white);
    draw_filled_rect_mut(image, left_rect, white);
}

fn draw_title_text(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, font: &Font, text: &str) {
    let title_scale = Scale::uniform(70.0);
    let title_width: f32 = text
        .chars()
        .map(|c| font.glyph(c).scaled(title_scale).h_metrics().advance_width)
        .sum::<f32>();
    let title_start_x: f32 = (image.height() as f32 - title_width) / 2.0;
    let title_start_y: f32 = 30.0; // 枠の上部の余白
    draw_text_mut(
        image,
        Rgba([255, 255, 255, 255]),
        title_start_x as i32,
        title_start_y as i32,
        title_scale,
        &font,
        text,
    );
}

fn draw_icon(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, icon_path: &str) {
    let icon: image::DynamicImage =
        image::open(icon_path)
            .unwrap()
            .resize(300, 300, image::imageops::FilterType::Lanczos3);
    let icon_width: u32 = icon.width();
    let icon_height: u32 = icon.height();
    overlay(
        image,
        &icon,
        (image.width() - icon_width) as i64,
        (image.height() - icon_height) as i64,
    );
}

fn draw_centered_text(
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    font: &Font,
    text: &str,
    color: Rgba<u8>,
) {
    let target_width: f32 = 630.0;
    let default_scale: Scale = Scale::uniform(50.0);
    let text_width_default_scale: f32 = text
        .chars()
        .map(|c| {
            font.glyph(c)
                .scaled(default_scale)
                .h_metrics()
                .advance_width
        })
        .sum::<f32>();
    let scale_factor: f32 = target_width / text_width_default_scale;
    let scale: Scale = Scale::uniform(50.0 * scale_factor);
    let max_scale = 70.0;
    let v_metrics: rusttype::VMetrics = font.v_metrics(scale);
    let text_height: f32 = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
    let start_x: f32 = 285.0;
    let start_y: f32 = (image.height() as f32 - text_height) / 2.0 + 35.0;
    if max_scale < scale.x {
        if scale.x > 100 as f32 {
            let text_width_scale: f32 = text
                .chars()
                .map(|c| {
                    font.glyph(c)
                        .scaled(Scale::uniform(100.0))
                        .h_metrics()
                        .advance_width
                })
                .sum::<f32>();
            let v_metrics: rusttype::VMetrics = font.v_metrics(Scale::uniform(100.0));
            let text_height: f32 = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
            let start_x = (image.width() as f32 - text_width_scale) / 2.0;
            let start_y = (image.height() as f32 - text_height) / 2.0 + 35.0;
            draw_text_mut(
                image,
                color,
                start_x as i32,
                start_y as i32,
                Scale::uniform(100.0),
                &font,
                text,
            );
        } else {
            draw_text_mut(
                image,
                color,
                start_x as i32,
                start_y as i32,
                scale,
                &font,
                text,
            );
        }
    } else {
        let split_index: usize = text.chars().count() / 2;
        let (first_half, second_half) = text.chars().enumerate().fold(
            (String::new(), String::new()),
            |(mut first, mut second), (i, c)| {
                if i < split_index {
                    first.push(c);
                } else {
                    second.push(c);
                }
                (first, second)
            },
        );

        let first_half_width_default_scale: f32 = first_half
            .chars()
            .map(|c| {
                font.glyph(c)
                    .scaled(default_scale)
                    .h_metrics()
                    .advance_width
            })
            .sum::<f32>();
        let second_half_width_default_scale = second_half
            .chars()
            .map(|c| {
                font.glyph(c)
                    .scaled(default_scale)
                    .h_metrics()
                    .advance_width
            })
            .sum::<f32>();
        let max_half_width: f32 =
            first_half_width_default_scale.max(second_half_width_default_scale);
        let scale_factor: f32 = target_width / max_half_width;
        let new_scale: Scale = Scale::uniform(50.0 * scale_factor);

        let first_half_width: f32 = first_half
            .chars()
            .map(|c| font.glyph(c).scaled(new_scale).h_metrics().advance_width)
            .sum::<f32>();
        let second_half_width = second_half
            .chars()
            .map(|c| font.glyph(c).scaled(new_scale).h_metrics().advance_width)
            .sum::<f32>();
        let start_x_first: f32 = (image.width() as f32 - first_half_width) / 2.0;
        let start_x_second: f32 = (image.width() as f32 - second_half_width) / 2.0;
        let start_y_first: f32 = image.height() as f32 / 2.0 - text_height * scale_factor - 10.0;
        let start_y_second: f32 = image.height() as f32 / 2.0;

        draw_text_mut(
            image,
            color,
            start_x_first as i32,
            start_y_first as i32,
            new_scale,
            &font,
            &first_half,
        );
        draw_text_mut(
            image,
            color,
            start_x_second as i32,
            start_y_second as i32,
            new_scale,
            &font,
            &second_half,
        );
    }
}

fn draw_caption(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, font: &Font, text: &str) {
    let scale = Scale::uniform(30.0);
    let start_x: f32 = 50.0; // 左端からの余白
    let start_y: f32 = 630.0 - 30.0;

    draw_text_mut(
        image,
        Rgba([255, 255, 255, 255]),
        start_x as i32,
        start_y as i32,
        scale,
        font,
        text,
    );
}

fn create_ogp_image(text: &str, icon_path: &str, font: &Font) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = (1200, 630);
    let maincolor = Rgba([34, 40, 49, 255]);
    let mut image: ImageBuffer<Rgba<u8>, Vec<u8>> = create_background(width, height, maincolor);

    draw_rounded_white_rect(&mut image);
    draw_title_text(&mut image, font, "ゆーちゃんのブログ");
    draw_icon(&mut image, icon_path);
    draw_centered_text(&mut image, font, text, maincolor);
    draw_caption(&mut image, font, "@yutyan_ut");

    image
}

fn main() {
    let text: &str = "ゆーちゃんのブログ"; // 文字列
    let text: &str = &env::args().nth(1).unwrap_or(text.to_string()); // 引数がなければデフォルトの文字列を使用
    let icon_path: &str = "icon.png"; // アイコン画像のパス
    let text_without_spaces = text.replace(" ", "-");
    let out_path: String = format!("{}.png", text_without_spaces); // 出力先のパス
    let font_data = include_bytes!("/home/yuto/ogp-creater/NotoSansJP-Bold.ttf"); // 利用するフォントファイルのパス
    let font: Font = Font::try_from_bytes(font_data as &[u8]).unwrap();
    let image = create_ogp_image(text, icon_path, &font);
    image.save(out_path).unwrap();
}
