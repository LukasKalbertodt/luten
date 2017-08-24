use maud::{html, Markup};

pub fn content() -> Markup {
    html! {
        div class="o-container o-container--xsmall@small u-letter-box--medium" {
            div class="c-card" {
                div class="c-card__item c-card__item--brand" "Einloggen"
                form method="post" action="/login" class="u-window-box--small" {
                    // Field: "username"
                    div class="u-letter-box--small login-field-container" {
                        div class="o-field o-field--icon-left" {
                            i class="fa fa-fw fa-user c-icon" {}
                            input
                                type="text"
                                class="c-field"
                                name="id"
                                placeholder="Nutzerkennung";
                            div class="c-hint u-small" {
                                "Gib hier deine RZ-Kennung ein (z.B. "
                                i "xmuster"
                                ")"
                            }
                        }
                    }

                    // Field: "password"
                    div class="u-letter-box--small login-field-container" {
                        div class="o-field o-field--icon-left" {
                            i class="fa fa-fw fa-key c-icon" {}
                            input
                                type="password"
                                class="c-field"
                                name="secret"
                                placeholder="Password";
                            div class="c-hint u-small" "Dein globales Uni-Password"
                        }
                    }

                    // Button: "Login"
                    div class="u-letter-box--small" {
                        input
                            type="submit"
                            value="Login"
                            class="c-button c-button--success"
                            style="width: 100%";
                    }
                }
            }
        }
    }
}
