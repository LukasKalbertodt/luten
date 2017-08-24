use std::borrow::Cow;
use maud::{html, DOCTYPE, Markup, Render};

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
    auth_user: Option<AuthUser>,
    content: Markup,
}

impl Page {
    /// An empty page without content, title, etc. Used to start the builder.
    pub fn empty() -> Self {
        Self {
            title: "".into(),
            nav_items: vec![],
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

    /// Set the main content of the page.
    pub fn with_content<T>(&mut self, content: T) -> &mut Self
        where T: Render
    {
        self.content = content.render();
        self
    }

    /// Finalize the page by rendering it into a `Markup` (basically a string).
    pub fn render(&self) -> Markup {
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
                        span
                            class="c-nav__content nav-title-box"
                            style={"color: " (title_color())}
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
                    (self.content)
                }
            }
        } }
    }
}

/// Generates the color for the title in the nav bar.
fn title_color() -> String {
    use chrono::{self, Timelike};
    use palette::{Hsl, IntoColor};

    // How much of the day is over (from 0 to 1).
    let day_progress = {
        let now = chrono::offset::Local::now();
        let minutes_of_day = now.hour() * 60 + now.minute();
        (minutes_of_day as f64) / (60.0 * 24.0)
    };

    let rgb_color = {
        // Throughout the day we go from red to green to blue to red.
        let hue = day_progress * 360.0;

        // We want a completely saturated, rather bright color.
        Hsl::new(hue.into(), 1.0, 0.9).into_rgb()
    };


    format!(
        "#{:2x}{:2x}{:2x}",
        (rgb_color.red * 255.0) as u8,
        (rgb_color.green * 255.0) as u8,
        (rgb_color.blue * 255.0) as u8,
    )
}
