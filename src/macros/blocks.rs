/// Builds a header block from a string (treated as `plain_text`). Optional `block_id`.
#[macro_export]
macro_rules! header {
    ($text:expr $(, block_id = $block_id:expr)? $(,)?) => {{
        let mut builder = $crate::blocks::Header::new($crate::blocks::PlainText::new($text));
        $( builder = builder.block_id($block_id); )?
        builder.build()
    }};
}

/// Builds a divider block. Optional `block_id`.
#[macro_export]
macro_rules! divider {
    ($(block_id = $block_id:expr)? $(,)?) => {{
        let mut builder = $crate::blocks::Divider::new();
        $( builder = builder.block_id($block_id); )?
        builder.build()
    }};
}

/// Builds a section block using named args.
/// Supported keys: `text`, `fields`, `accessory`, `expand`, `block_id`.
/// Note: Strings for `text` are treated as mrkdwn (Slack’s default for section).
#[macro_export]
macro_rules! section {
    ( $( $key:ident = $value:expr ),+ $(,)? ) => {{
        let mut builder = $crate::blocks::Section::new();
        $( section!(@apply builder, $key, $value); )+
        builder.build()
    }};
    (@apply $builder:ident, text, $value:expr) => { $builder = $builder.text($value); };
    (@apply $builder:ident, fields, $value:expr) => { $builder = $builder.fields($value); };
    (@apply $builder:ident, accessory, $value:expr) => { $builder = $builder.accessory($value); };
    (@apply $builder:ident, expand, $value:expr) => { $builder = $builder.expand($value); };
    (@apply $builder:ident, block_id, $value:expr) => { $builder = $builder.block_id($value); };
    (@apply $_builder:ident, $unexpected:ident, $_value:expr) => {
        compile_error!(concat!("unsupported section! argument: ", stringify!($unexpected)));
    };
}

/// Builds a video block with named args (any order).
/// Required: `title`, `video_url`, `thumbnail_url`, `alt_text`.
/// Optional: `description`, `title_url`, `provider_icon_url`, `provider_name`, `author_name`, `block_id`.
#[macro_export]
macro_rules! video {
    ( $( $key:ident = $value:expr ),+ $(,)? ) => {{
        let mut __title: Option<$crate::blocks::PlainText> = None;
        let mut __video_url: Option<String> = None;
        let mut __thumb: Option<String> = None;
        let mut __alt: Option<String> = None;
        let mut __description: Option<$crate::blocks::PlainText> = None;
        let mut __title_url: Option<String> = None;
        let mut __provider_icon_url: Option<String> = None;
        let mut __provider_name: Option<String> = None;
        let mut __author_name: Option<String> = None;
        let mut __block_id: Option<String> = None;
        $(
            video!(@assign __title, __video_url, __thumb, __alt, __description, __title_url, __provider_icon_url, __provider_name, __author_name, __block_id; $key = $value);
        )+
        let title = __title.expect("video! macro requires `title = ...`");
        let video_url = __video_url.expect("video! macro requires `video_url = ...`");
        let thumbnail_url = __thumb.expect("video! macro requires `thumbnail_url = ...`");
        let alt_text = __alt.expect("video! macro requires `alt_text = ...`");
        let mut builder = $crate::blocks::Video::new(title, video_url, thumbnail_url, alt_text);
        if let Some(desc) = __description { builder = builder.description(desc); }
        if let Some(url) = __title_url { builder = builder.title_url(url); }
        if let Some(icon) = __provider_icon_url { builder = builder.provider_icon_url(icon); }
        if let Some(name) = __provider_name { builder = builder.provider_name(name); }
        if let Some(author) = __author_name { builder = builder.author_name(author); }
        if let Some(id) = __block_id { builder = builder.block_id(id); }
        builder.build()
    }};
    (@assign $title:ident, $video_url:ident, $thumb:ident, $alt:ident, $desc:ident, $title_url:ident, $icon:ident, $provider_name:ident, $author:ident, $block_id:ident; title = $value:expr) => { $title = Some($crate::blocks::PlainText::new($value)); };
    (@assign $title:ident, $video_url:ident, $thumb:ident, $alt:ident, $desc:ident, $title_url:ident, $icon:ident, $provider_name:ident, $author:ident, $block_id:ident; video_url = $value:expr) => { $video_url = Some($value.into()); };
    (@assign $title:ident, $video_url:ident, $thumb:ident, $alt:ident, $desc:ident, $title_url:ident, $icon:ident, $provider_name:ident, $author:ident, $block_id:ident; thumbnail_url = $value:expr) => { $thumb = Some($value.into()); };
    (@assign $title:ident, $video_url:ident, $thumb:ident, $alt:ident, $desc:ident, $title_url:ident, $icon:ident, $provider_name:ident, $author:ident, $block_id:ident; alt_text = $value:expr) => { $alt = Some($value.into()); };
    (@assign $title:ident, $video_url:ident, $thumb:ident, $alt:ident, $desc:ident, $title_url:ident, $icon:ident, $provider_name:ident, $author:ident, $block_id:ident; description = $value:expr) => { $desc = Some($crate::blocks::PlainText::new($value)); };
    (@assign $title:ident, $video_url:ident, $thumb:ident, $alt:ident, $desc:ident, $title_url:ident, $icon:ident, $provider_name:ident, $author:ident, $block_id:ident; title_url = $value:expr) => { $title_url = Some($value.into()); };
    (@assign $title:ident, $video_url:ident, $thumb:ident, $alt:ident, $desc:ident, $title_url:ident, $icon:ident, $provider_name:ident, $author:ident, $block_id:ident; provider_icon_url = $value:expr) => { $icon = Some($value.into()); };
    (@assign $title:ident, $video_url:ident, $thumb:ident, $alt:ident, $desc:ident, $title_url:ident, $icon:ident, $provider_name:ident, $author:ident, $block_id:ident; provider_name = $value:expr) => { $provider_name = Some($value.into()); };
    (@assign $title:ident, $video_url:ident, $thumb:ident, $alt:ident, $desc:ident, $title_url:ident, $icon:ident, $provider_name:ident, $author:ident, $block_id:ident; author_name = $value:expr) => { $author = Some($value.into()); };
    (@assign $title:ident, $video_url:ident, $thumb:ident, $alt:ident, $desc:ident, $title_url:ident, $icon:ident, $provider_name:ident, $author:ident, $block_id:ident; block_id = $value:expr) => { $block_id = Some($value.into()); };
    (@assign $title:ident, $video_url:ident, $thumb:ident, $alt:ident, $desc:ident, $title_url:ident, $icon:ident, $provider_name:ident, $author:ident, $block_id:ident; $unexpected:ident = $_value:expr) => {
        compile_error!(concat!("unsupported video! argument: ", stringify!($unexpected)));
    };
}

/// Collects Result<Block> values into a Result<Vec<Block>>.
#[macro_export]
macro_rules! blocks {
    ($($block:expr),+ $(,)?) => {{
        let mut items = ::std::vec::Vec::new();
        $( items.push($block?); )+
        ::std::result::Result::Ok(items)
    }};
    () => { ::std::result::Result::Ok(::std::vec::Vec::new()) };
}

/// Builds an image block with named args (any order).
/// Required: `alt_text` and exactly one of (`image_url` | `slack_file_id`/`slack_file_url`).
/// Optional: `title`, `block_id`.
#[macro_export]
macro_rules! image {
    ( $( $key:ident = $value:expr ),+ $(,)? ) => {{
        let mut __alt_text: Option<String> = None;
        let mut __image_url: Option<String> = None;
        let mut __slack_file_id: Option<String> = None;
        let mut __slack_file_url: Option<String> = None;
        let mut __title: Option<String> = None;
        let mut __block_id: Option<String> = None;
        $( image!(@assign __alt_text, __image_url, __slack_file_id, __slack_file_url, __title, __block_id; $key = $value); )+

        let alt_text = __alt_text.expect("image! macro requires `alt_text = ...`");
        let mut builder = $crate::blocks::Image::new(alt_text);

        if let Some(url) = __image_url { builder = builder.image_url(url); }
        if __slack_file_id.is_some() || __slack_file_url.is_some() {
            let sf = $crate::blocks::SlackFileRef { url: __slack_file_url, id: __slack_file_id };
            builder = builder.slack_file(sf);
        }
        if let Some(t) = __title { builder = builder.title($crate::blocks::PlainText::new(t)); }
        if let Some(id) = __block_id { builder = builder.block_id(id); }

        builder.build()
    }};
    (@assign $alt:ident, $img_url:ident, $sf_id:ident, $sf_url:ident, $title:ident, $block_id:ident; alt_text = $value:expr) => { $alt = Some($value.into()); };
    (@assign $alt:ident, $img_url:ident, $sf_id:ident, $sf_url:ident, $title:ident, $block_id:ident; image_url = $value:expr) => { $img_url = Some($value.into()); };
    (@assign $alt:ident, $img_url:ident, $sf_id:ident, $sf_url:ident, $title:ident, $block_id:ident; slack_file_id = $value:expr) => { $sf_id = Some($value.into()); };
    (@assign $alt:ident, $img_url:ident, $sf_id:ident, $sf_url:ident, $title:ident, $block_id:ident; slack_file_url = $value:expr) => { $sf_url = Some($value.into()); };
    (@assign $alt:ident, $img_url:ident, $sf_id:ident, $sf_url:ident, $title:ident, $block_id:ident; title = $value:expr) => { $title = Some($value.into()); };
    (@assign $alt:ident, $img_url:ident, $sf_id:ident, $sf_url:ident, $title:ident, $block_id:ident; block_id = $value:expr) => { $block_id = Some($value.into()); };
    (@assign $_alt:ident, $_img_url:ident, $_sf_id:ident, $_sf_url:ident, $_title:ident, $_block_id:ident; $unexpected:ident = $_value:expr) => {
        compile_error!(concat!("unsupported image! argument: ", stringify!($unexpected)));
    };
}

/// Builds a file block with named args (any order).
/// Required: `external_id`. Source defaults to `remote`.
/// Optional: `block_id`.
#[macro_export]
macro_rules! file {
    ( $( $key:ident = $value:expr ),+ $(,)? ) => {{
        let mut __external_id: Option<String> = None;
        let mut __block_id: Option<String> = None;
        $( file!(@assign __external_id, __block_id; $key = $value); )+

        let external_id = __external_id.expect("file! macro requires `external_id = ...`");
        let mut builder = $crate::blocks::File::new(external_id, $crate::blocks::FileSource::Remote);
        if let Some(id) = __block_id { builder = builder.block_id(id); }
        builder.build()
    }};
    (@assign $ext:ident, $block_id:ident; external_id = $value:expr) => { $ext = Some($value.into()); };
    (@assign $ext:ident, $block_id:ident; block_id = $value:expr) => { $block_id = Some($value.into()); };
    (@assign $_ext:ident, $_block_id:ident; $unexpected:ident = $_value:expr) => {
        compile_error!(concat!("unsupported file! argument: ", stringify!($unexpected)));
    };
}

/// Builds a context block from an inline list.
/// Elements supported: `mrkdwn("...")`, `plain("...")`, `image(url, alt)`, or string literals.
/// Default for string/text(...) is plain_text (for consistency with header!).
#[macro_export]
macro_rules! context {
    ( elements = [ $($rest:tt)* ] $(, block_id = $block_id:expr)? $(,)? ) => {{
        let mut __items: ::std::vec::Vec<$crate::blocks::ContextElement> = ::std::vec::Vec::new();
        context!(@list __items; $($rest)*);
        let mut builder = $crate::blocks::Context::new(__items);
        $( builder = builder.block_id($block_id); )?
        builder.build()
    }};
    ( [ $($rest:tt)* ] $(, block_id = $block_id:expr)? $(,)? ) => {{
        context!(elements = [ $($rest)* ] $(, block_id = $block_id )? )
    }};
    // Recursive list processor
    (@list $v:ident; ) => {};
    (@list $v:ident; mrkdwn($text:expr) , $($rest:tt)* ) => {{ $v.push($crate::blocks::ContextElement::mrkdwn($text)); context!(@list $v; $($rest)*); }};
    (@list $v:ident; mrkdwn($text:expr) ) => {{ $v.push($crate::blocks::ContextElement::mrkdwn($text)); }};
    (@list $v:ident; plain($text:expr) , $($rest:tt)* ) => {{ $v.push($crate::blocks::ContextElement::plain_text($text)); context!(@list $v; $($rest)*); }};
    (@list $v:ident; plain($text:expr) ) => {{ $v.push($crate::blocks::ContextElement::plain_text($text)); }};
    (@list $v:ident; image($url:expr, $alt:expr) , $($rest:tt)* ) => {{ $v.push($crate::blocks::ContextElement::image($url, $alt)); context!(@list $v; $($rest)*); }};
    (@list $v:ident; image($url:expr, $alt:expr) ) => {{ $v.push($crate::blocks::ContextElement::image($url, $alt)); }};
    // Convenience: allow string literals; default to plain text for consistency
    (@list $v:ident; $text:literal , $($rest:tt)* ) => {{ $v.push($crate::blocks::ContextElement::plain_text($text)); context!(@list $v; $($rest)*); }};
    (@list $v:ident; $text:literal ) => {{ $v.push($crate::blocks::ContextElement::plain_text($text)); }};
    // Convenience: explicit text($expr) treated as plain text
    (@list $v:ident; text($text:expr) , $($rest:tt)* ) => {{ $v.push($crate::blocks::ContextElement::plain_text($text)); context!(@list $v; $($rest)*); }};
    (@list $v:ident; text($text:expr) ) => {{ $v.push($crate::blocks::ContextElement::plain_text($text)); }};
    (@list $_v:ident; $unexpected:tt ) => {{
        compile_error!("unsupported context! element; use mrkdwn(...), plain(...), image(url, alt), or a string literal");
    }};
}

