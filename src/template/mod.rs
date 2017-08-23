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
/// ```
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
                link rel="stylesheet" href="/static/main.css";
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
                        span class="c-nav__content nav-brand" (config::WEBSITE_TITLE)

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
