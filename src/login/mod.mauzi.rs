// Login error messages
unit err_incorrect_secret {
    De => "Das eingegebene Passwort ist falsch.",
    En => "The given password is incorrect.",
}
unit err_provider_not_usable {
    De => "Der angegebene Nutzer kann nicht mit der gewählten Methode authentifiziert werden.",
}
unit err_user_not_found {
    De => "Der angegebene Nutzer wurde nicht gefunden.",
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
unit username_hint {
    De => "Deine RZ-Kennung (z.B. xmuster)",    // TODO: italics around xmuster
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


// TODO: unit notice_box_content


// The box containing further information about the login process
unit notice_box_title {
    De => "Hinweise",
    En => "Hints",
}
