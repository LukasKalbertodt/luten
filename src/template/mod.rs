use std::borrow::Cow;
use maud::{html, DOCTYPE, Markup, Render};
use rocket::request::FlashMessage;

use config;
use user::AuthUser;


/// Builder type, used to render the whole page as HTML.
///
/// This type provides the main structure of the HTML page. It will generate
/// the surrounding tags, such as <html>, the <head> section and the main
/// structure of the <body> section. All the HTML generation can be influenced
/// by setting attributes on this type, such as the `title` attribute, which
/// is used to fill the <title> tag.
///
/// All routes which return a standard HTML result, will look roughly like
/// this:
///
/// ```ignore
/// use maud::{html, Markup};
///
/// fn handler() -> Markup {
///     // Do stuff...
///
///     Page::empty()
///         .with_title("foobar")
///         // .with_more_stuff
///         .with_content(html! {
///             h1 "Hello"
///         })
///         .render()
/// }
/// ```
///
pub struct Page {
    title: Cow<'static, str>,
    nav_items: Vec<Cow<'static, str>>,
    flashes: Vec<Flash>,
    auth_user: Option<AuthUser>,
    content: Markup,
}

impl Page {
    /// An empty page without content, title, etc. Used to start the builder.
    pub fn empty() -> Self {
        Self {
            title: "".into(),
            nav_items: vec![],
            flashes: vec![],
            auth_user: None,
            content: html!{},
        }
    }

    /// Sets the title.
    ///
    /// Note that this "title" is only the changing part of the title. The
    /// value in the <title> tag will have a postfixed "-- Foo" where "Foo" is
    /// the value of `config::WEBSITE_TITLE`.
    pub fn with_title<T>(&mut self, title: T) -> &mut Self
        where T: Into<Cow<'static, str>>
    {
        self.title = title.into();
        self
    }

    // /// Sets the items of the navigation bar.
    // pub fn with_nav<T>(&mut self, nav: T) -> &mut Self
    //     where T: IntoIterator,
    //           T::Item: Into<Cow<'static, str>>,
    // {
    //     self.nav_items = nav.into_iter().map(|e| e.into()).collect();
    //     self
    // }

    /// Sets the "auth user" (the user that is logged in).
    ///
    /// This should always be called if a user is logged in. Setting the user
    /// generates the "Account" item in the nav bar, which is hidden otherwise.
    pub fn with_auth_user(&mut self, auth_user: &AuthUser) -> &mut Self {
        self.auth_user = Some(auth_user.clone());
        self
    }

    /// Adds flashes to the page.
    ///
    /// A flash is a small box at the top of the page usually showing the
    /// outcome of a recent action. There are "error", "warning", "success" and
    /// "info" flashes, each with their individual color. For example, the
    /// message on a failed login attempt is a flash.
    ///
    /// This method accepts anything that can be turned into an iterator which
    /// yields elements that can be turned into a `Flash`. This conveniently
    /// allows to pass `Option<rocket::FlashMessage>`!
    pub fn add_flashes<I, T>(&mut self, flashes: I) -> &mut Self
        where I: IntoIterator<Item=T>,
              T: Into<Flash>,
    {
        self.flashes.extend(flashes.into_iter().map(|t| t.into()));
        self
    }

    /// Set the main content of the page.
    pub fn with_content<T>(&mut self, content: T) -> &mut Self
        where T: Render
    {
        self.content = content.render();
        self
    }

    /// Finalize the page by rendering it into a `Markup` (basically a string).
    pub fn render(&self) -> Markup {
        let (title_fg, title_border) = title_colors();

        html! { (DOCTYPE) html {
            // ===============================================================
            // Start <head>
            // ===============================================================
            head {
                link rel="stylesheet" href="/static/blaze@3.3.0.css";
                link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/\
                    font-awesome/4.7.0/css/font-awesome.min.css";
                link rel="stylesheet" href="/static/main.css";
                link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Courgette"
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title {
                    (self.title)
                    ({
                        if self.title.is_empty() {
                            ""
                        } else {
                            " – "
                        }
                    })
                    (config::WEBSITE_TITLE)
                }
            }

            // ===============================================================
            // Start <body>
            // ===============================================================
            body class="c-text" {
                header {
                    nav class="c-nav c-nav--inline" {
                        // The title of the page in the nav bar (branding)
                        a
                            href="/"
                            class="c-nav__content nav-title-box"
                            style={
                                "color: "
                                (title_fg)
                                ";"
                                "border-right: 1px dashed "
                                (title_border)
                            }
                            (config::WEBSITE_TITLE)

                        // All given nav items
                        @for item in &self.nav_items {
                            span class="c-nav__item" (item)
                        }

                        // TODO: hide help?
                        span class="c-nav__item" "Hilfe"

                        // Show account entry if the user has been logged in
                        @if let Some(ref auth_user) = self.auth_user {
                            span class="c-nav__item c-nav__item--right c-nav__item--info" {
                                "Account (" (auth_user.username()) ")"
                            }
                        }
                    }
                }
                main class="o-container o-container--large u-pillar-box--small" {
                    // Show all flashes
                    div class="u-letter-box--small" {
                        @for flash in &self.flashes {
                            div class={"c-alert " (flash.kind.css_class())} {
                                button class="c-button c-button--close" "×"
                                (flash.content)
                            }
                        }
                    }

                    // The main content
                    (self.content)
                }
            }
        } }
    }
}

/// A small box at the top of the website.
#[derive(Debug)]
pub struct Flash {
    content: Markup,
    kind: FlashKind,
}

impl Flash {
    pub fn info(content: Markup) -> Self {
        Self {
            kind: FlashKind::Info,
            content,
        }
    }

    pub fn success(content: Markup) -> Self {
        Self {
            kind: FlashKind::Success,
            content,
        }
    }

    pub fn warning(content: Markup) -> Self {
        Self {
            kind: FlashKind::Warning,
            content,
        }
    }

    pub fn error(content: Markup) -> Self {
        Self {
            kind: FlashKind::Error,
            content,
        }
    }
}

impl From<FlashMessage> for Flash {
    fn from(msg: FlashMessage) -> Self {
        let kind = match msg.name() {
            "success" => FlashKind::Success,
            "warning" => FlashKind::Warning,
            "error" => FlashKind::Error,
            _ => FlashKind::Info,
        };

        Self {
            kind,
            content: html! { (msg.msg()) },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlashKind {
    Info,
    Success,
    Warning,
    Error,
}

impl FlashKind {
    fn css_class(&self) -> &'static str {
        use self::FlashKind::*;

        match *self {
            Info => "c-alert--info",
            Success => "c-alert--success",
            Warning => "c-alert--warning",
            Error => "c-alert--error",
        }
    }
}

/// Generates the color for the title in the nav bar.
fn title_colors() -> (String, String) {
    use chrono::{self, Timelike};
    use palette::{Hsl, IntoColor};

    // How much of the day is over (from 0 to 1).
    let day_progress = {
        let now = chrono::offset::Local::now();
        let minutes_of_day = now.hour() * 60 + now.minute();
        (minutes_of_day as f32) / (60.0 * 24.0)
    };

    fn hue_to_hex_rgb(hsl: Hsl) -> String {
        let rgb = hsl.into_rgb();

        format!(
            "#{:02x}{:02x}{:02x}",
            (rgb.red * 255.0) as u8,
            (rgb.green * 255.0) as u8,
            (rgb.blue * 255.0) as u8,
        )
    }

    // Throughout the day we go from red to green to blue to red.
    let hue = day_progress * 360.0;

    // We want a completely saturated, rather bright color.
    let text_color = Hsl::new(hue.into(), 1.0, 0.9);

    // The border is less saturated and less bright.
    let border_color = Hsl::new(hue.into(), 0.2, 0.5);

    (hue_to_hex_rgb(text_color), hue_to_hex_rgb(border_color))
}
