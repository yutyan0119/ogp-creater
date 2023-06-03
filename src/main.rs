use image::imageops::overlay;
use image::{ImageBuffer, Rgba};
use imageproc::drawing::draw_filled_circle_mut;
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::drawing::draw_text_mut;
use lindera_core::{mode::Mode, LinderaResult};
use lindera_dictionary::{DictionaryConfig, DictionaryKind, UserDictionaryConfig};
use lindera_tokenizer::tokenizer::{Tokenizer, TokenizerConfig};
use rusttype::{Font, Scale};
use std::env;
use std::path::PathBuf;

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

fn split_string(
    text: &str,
    target_width: f32,
    font: &Font,
    default_scale: Scale,
) -> LinderaResult<Vec<String>> {
    let dictionary: DictionaryConfig = DictionaryConfig {
        kind: Some(DictionaryKind::IPADIC),
        path: None,
    };

    let user_dictionary = Some(UserDictionaryConfig {
        kind: Some(DictionaryKind::IPADIC),
        path: PathBuf::from("/home/yuto/ogp-creater/user-dict.csv"),
    });

    let config: TokenizerConfig = TokenizerConfig {
        dictionary,
        user_dictionary,
        mode: Mode::Normal,
    };

    let tokenizer: Tokenizer = Tokenizer::from_config(config)?;

    let tokens: Vec<lindera_tokenizer::token::Token> = tokenizer.tokenize(text)?;
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0.0;

    for token in tokens {
        let word = token.text;
        let word_width: f32 = word
            .chars()
            .map(|c| {
                font.glyph(c)
                    .scaled(default_scale)
                    .h_metrics()
                    .advance_width
            })
            .sum::<f32>();

        if current_width + word_width <= target_width {
            current_line.push_str(word);
            current_width += word_width;
        } else {
            lines.push(current_line);
            current_line = word.to_string();
            current_width = word_width;
        }
    }

    lines.push(current_line); // Add the last line
    Ok(lines)
}

fn draw_centered_lines(
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    font: &Font,
    lines: &[String],
    color: Rgba<u8>,
    scale: Scale,
) {
    let v_metrics: rusttype::VMetrics = font.v_metrics(scale);
    let line_height: f32 = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
    let text_height: f32 = line_height * lines.len() as f32;
    let start_y: f32 = (image.height() as f32 - text_height) / 2.0;

    for (i, line) in lines.iter().enumerate() {
        let text_width: f32 = line
            .chars()
            .map(|c| font.glyph(c).scaled(scale).h_metrics().advance_width)
            .sum::<f32>();

        let start_x: f32 = (image.width() as f32 - text_width) / 2.0;
        let y_position: f32 = start_y + line_height * i as f32;

        draw_text_mut(
            image,
            color,
            start_x as i32,
            y_position as i32,
            scale,
            &font,
            line,
        );
    }
}

fn draw_text_on_image(
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    font: &Font,
    text: &str,
    color: Rgba<u8>,
) -> LinderaResult<()> {
    let target_width: f32 = 630.0;
    let scale: Scale = Scale::uniform(80.0);

    let lines: Vec<String> = split_string(text, target_width, font, scale)?;

    draw_centered_lines(image, font, &lines, color, scale);

    Ok(())
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
    let _ = draw_text_on_image(&mut image, font, text, maincolor);
    // draw_centered_text(&mut image, font, text, maincolor);
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
