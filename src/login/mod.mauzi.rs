// Login error messages and other flash messages
unit err_user_not_found {
    De => "Der angegebene Nutzer wurde nicht gefunden.",
}
unit err_incorrect_secret {
    De => "Das eingegebene Passwort ist falsch.",
    En => "The given password is incorrect.",
}
unit err_provider_not_usable {
    De => "Der angegebene Nutzer kann nicht mit der gewählten Methode authentifiziert werden.",
}
unit successful_logout {
    De => "Du wurdest erfolgreich ausgeloggt.",
}

// The box containing the actual login-form
unit box_title {
    De => "Einloggen",
    En => "Login",
}
unit username_placeholder {
    De => "Nutzername",
    En => "Username",
}
unit username_hint -> Markup {
    De => { html! {
        "Deine RZ-Kennung (z.B. "
        i "xmuster"
        ")"
    }},
    En => { html! {
        "Your rz-login (e.g. "
        i "xmuster"
        ")"
    }},
}
unit password_placeholder {
    De => "Passwort",
    En => "Password",
}
unit password_hint {
    De => "Dein globales Uni-Passwort",
}
unit login_provider_label {
    De => "Anmelden mit:",
}
unit login_button_label { // TODO: it would be nice to say `login_button = box_title;`
    De => "Einloggen",
}


// The box containing further information about the login process
unit notice_box_title {
    De => "Hinweise",
    En => "Hints",
}
unit notice_box_content -> Markup {
    De => { html! {
        li "Nutze zum Einloggen deine normalen Uni-Login-Daten"
        li "Die Authentifizierung erfolgt über das Uni-LDAP"
        li "Dein Passwort wird zu keiner Zeit gespeichert"
    }},
    En => { html! {
        li "Use your usual university login data for logging in"
        li "The university's LDAP is used for authentification"
        li "Your password will never be stored"
    }},
}


// Names of login provider
unit provider_name_ldap {
    De => "Universität-LDAP",
    En => "University-LDAP",
}
unit provider_name_password {
    De => "Passwort (intern)",
    En => "Password (internal)",
}
