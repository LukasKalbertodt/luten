//! Helper to actually output some HTML.

use std::borrow::Cow;
use maud::{html, DOCTYPE, Markup, Render};
use option_filter::OptionFilterExt;
use rocket::Request;
use rocket::request::FlashMessage;
use rocket::response::{self, Responder};

use config;
use dict::{self, Locale};
use state::FrozenState;
use user::AuthUser;


/// Builder type, used to render the whole page as HTML.
///
/// This type provides the main structure of the HTML page. It will generate
/// the surrounding tags, such as <html>, the <head> section and the main
/// structure of the <body> section. All the HTML generation can be influenced
/// by setting attributes on this type, such as the `title` attribute, which
/// is used to fill the `<title>` tag.
///
/// All routes which return a standard HTML result, will look roughly like
/// this:
///
/// ```ignore
/// use maud::html;
///
/// fn handler() -> Page {
///     // Do stuff...
///
///     Page::empty()
///         .with_title("foobar")
///         // .with_more_stuff
///         .with_content(html! {
///             h1 "Hello"
///         })
/// }
/// ```
///
#[derive(Debug)]
pub struct Page {
    title: Cow<'static, str>,
    nav_items: Vec<NavItem>,
    flashes: Vec<FlashBubble>,
    content: Markup,
    active_nav_route: Option<Cow<'static, str>>,
}

impl Page {
    /// An empty page without content, title, etc. Used to start the builder.
    pub fn empty() -> Self {
        Self {
            title: "".into(),
            nav_items: vec![],
            flashes: vec![],
            content: html!{},
            active_nav_route: None,
        }
    }

    /// An empty page with a single error flash.
    pub fn error<M: Render>(flash_content: M) -> Self {
        Self::empty()
            .add_flash(FlashBubble::error(flash_content))
    }

    /// An empty page showing a single error saying the page is unimplemented.
    pub fn unimplemented() -> Self {
        Self::error("This page is not implemented yet!")
    }

    /// Sets the title.
    ///
    /// Note that this "title" is only the changing part of the title. The
    /// value in the `<title>` tag will have a postfixed "-- Foo" where "Foo"
    /// is the value of `config::WEBSITE_TITLE`.
    pub fn with_title<T>(mut self, title: T) -> Self
        where T: Into<Cow<'static, str>>
    {
        self.title = title.into();
        self
    }

    /// Adds flashes to the page.
    ///
    /// A flash is a small box at the top of the page usually showing the
    /// outcome of a recent action. There are "error", "warning", "success" and
    /// "info" flashes, each with their individual color. For example, the
    /// message on a failed login attempt is a flash.
    ///
    pub fn add_flash<T: Into<FlashBubble>>(mut self, flash: T) -> Self {
        self.flashes.push(flash.into());
        self
    }

    /// Adds nav items to the page.
    ///
    /// The nav items are placed in the nav bar between the brand-text and the
    /// "Account" item.
    pub fn add_nav_items<I>(mut self, nav_items: I) -> Self
        where I: IntoIterator<Item=NavItem>
    {
        self.nav_items.extend(nav_items);
        self
    }

    /// Sets a nav route as active. The corresponding nav item will be
    /// highlighted.
    pub fn with_active_nav_route<T>(mut self, active_nav_route: T) -> Self
        where T: Into<Cow<'static, str>>
    {
        self.active_nav_route = Some(active_nav_route.into());
        self
    }

    /// Set the main content of the page.
    pub fn with_content<T>(mut self, content: T) -> Self
        where T: Render
    {
        self.content = content.render();
        self
    }

    /// Finalize the page by rendering it into a `Markup` (basically a string).
    fn render(mut self, req: &Request) -> Markup {
        let locale = req.guard::<Locale>().unwrap();
        let dict = dict::new(locale);

        // Check for Rocket flashes
        if let Some(flash) = req.guard::<FlashMessage>().succeeded() {
            self.flashes.push(flash.into());
        }

        // Check for a frozen application and show a flash in that case. This
        // flash is always shown first.
        if let Some(frozen_state) = req.guard::<FrozenState>().succeeded() {
            let flash = FlashBubble::info(
                dict.frozen_flash(frozen_state.0.reason(), frozen_state.0.next_state_switch)
            );
            self.flashes.insert(0, flash);
        }




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
                script type="text/javascript" src="/static/main.js" {}
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
                header (self.render_nav(req, locale))
                main class="o-container o-container--large u-pillar-box--small" {
                    // Show all flashes
                    div class="u-letter-box--small" {
                        @for flash in &self.flashes {
                            div class={"c-alert " (flash.kind.css_class())} {
                                button
                                    class="c-button c-button--close"
                                    onclick="Luten.Util.closeFlash(this)"
                                    "×"
                                (flash.content)
                            }
                        }
                    }

                    // The main content
                    (self.content)
                }
                footer style="height: 70px;" {}
            }
        } }
    }

    fn render_nav(&self, req: &Request, locale: Locale) -> Markup {
        let (title_fg, title_border) = title_colors();
        let dict = dict::new(locale);

        // Try to create an auth user from the request.
        let auth_user = req.guard::<AuthUser>().succeeded();

        // Add "Admin Panel" nav item if the user is an admin
        // TODO: l10n
        let mut nav_items = self.nav_items.clone();
        if auth_user.as_ref().filter(|u| u.is_admin()).is_some() {
            nav_items.push(NavItem::new("Admin Panel", "/admin_panel"));
        }

        html! {
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
                @for item in nav_items {
                    a class={
                        "c-nav__item"
                        @if Some(&item.url) == self.active_nav_route.as_ref() {
                            " c-nav__item--active"
                        } @else {
                            ""
                        }
                    } href=(item.url) (item.title)
                }

                // Show account entry if the user has been logged in
                @if let Some(auth_user) = auth_user {
                    div class="c-nav__item c-nav__item--right c-nav__item--info nav-account-box" {
                        i class="fa fa-user" {}
                        " " (dict.nav_account()) " (" (auth_user.username()) ")"

                        ul class="nav-account-children c-nav" {
                            @if let Some(name) = auth_user.name() {
                                li class="c-nav__content u-centered c-text--loud" (name)
                            }
                            li class="c-nav__item" {
                                a href="/settings" {
                                    i class="fa fa-sliders" {}
                                    " "
                                    (dict.nav_settings())
                                }
                            }
                            li class="c-nav__item" {

                                form action="/logout" method="POST" style="height: 100%" {
                                    input type="hidden" name="_method" value="DELETE" {}
                                    button type="submit" class="logout-button" {
                                        i class="fa fa-sign-out" {}
                                        " "
                                        (dict.nav_logout())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl<'r> Responder<'r> for Page {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        self.render(req).respond_to(req)
    }
}

/// An item in the navigation bar at the very top of the page.
#[derive(Debug, Clone)]
pub struct NavItem {
    title: Cow<'static, str>,
    url: Cow<'static, str>,
}

impl NavItem {
    pub fn new<T, U>(title: T, url: U) -> Self
        where T: Into<Cow<'static, str>>,
              U: Into<Cow<'static, str>>,
    {
        Self {
            title: title.into(),
            url: url.into(),
        }
    }
}

/// A small box at the top of the website.
#[derive(Debug)]
pub struct FlashBubble {
    content: Markup,
    kind: FlashKind,
}

impl FlashBubble {
    pub fn info<M: Render>(content: M) -> Self {
        Self {
            kind: FlashKind::Info,
            content: content.render(),
        }
    }

    pub fn success<M: Render>(content: M) -> Self {
        Self {
            kind: FlashKind::Success,
            content: content.render(),
        }
    }

    pub fn warning<M: Render>(content: M) -> Self {
        Self {
            kind: FlashKind::Warning,
            content: content.render(),
        }
    }

    pub fn error<M: Render>(content: M) -> Self {
        Self {
            kind: FlashKind::Error,
            content: content.render(),
        }
    }
}

impl From<FlashMessage> for FlashBubble {
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
