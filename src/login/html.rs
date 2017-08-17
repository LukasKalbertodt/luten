use maud::{html, Markup};

pub fn content() -> Markup {
    html! {
        div class="login-box" {
            h2 "Einloggen"
            hr;
            form method="post" action="/login" class="basgit-form" {
                input type="text" name="id" placeholder="Nutzerkennung";
                input type="password" name="secret" placeholder="Password";
                input type="submit" value="Login";
            }

            hr;
            p "Uhm..."
        }
    }
}
