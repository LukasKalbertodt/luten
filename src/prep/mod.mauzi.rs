unit state_title {
    De => "Terminfindung"
}

unit nav_overview_title {
    De => "Übersicht",
}

unit nav_timeslots_title {
    De => "Zeitslots",
}

unit overview_title {
    // TODO: avoid duplicated text
    De => "Terminfindung: Übersicht",
}

unit timeslots_title {
    // TODO: avoid duplicated text
    De => "Terminfindung: Zeitslots",
}


// ===========================================================================
// General preferences
// ===========================================================================
unit explanation_box_title {
    De => "Terminfindung: Informationen",
}

unit explanation_for_students {
    De => "Bevor der Testatbetrieb richtig losgeht, müssen sich erstmale alle \
           Studenten in Zweier-Gruppen zusammenfinden und jede Gruppe muss \
           einen festen Testattermin bei einem festen Tutor zugeordnet \
           bekommen. Damit jeder einen möglichst guten Termin mit einem \
           passenden Partner findet, kannst du hier deine Termin- sowie \
           Partner-Präferenzen angeben.",
}

unit student_status(timeslot_status: Markup, partner: Markup, lang: Markup) -> Markup {
    De => { html! {
        b "Dein Status:"
        ul {
            li { "Terminwahl: " (timeslot_status) }
            li { "Partner: " (partner) }
            li { "Sprache: " (lang) }
        }
    }}
    En => { html! {
        b "Your status:"
        ul {
            li { "Chosen timeslots: " (timeslot_status) }
            li { "Partner: " (partner) }
            li { "Language: " (lang) }
        }
    }}
}

unit random_partner {
    De => "Zufälliger Partner",
}

unit settings_headline {
    De => "Einstellungen",
}

unit partner_sub_headline {
    De => "Partner",
}

unit language_sub_headline {
    De => "Bevorzugte Sprache",
}

unit partner_hints {
    De => "Du kannst dir entweder einen Zufallspartner zuweisen lassen oder \
           einen Kommilitonen angeben, den du gerne als Partner hättest."
}

unit hints_title {
    De => "Hinweise",
}

unit choose_partner {
    De => "Partner auswählen",
}

unit id_of_partner_placeholder {
    De => "RZ-Kennung des Partners",
}

unit language_hints {
    De => "Wenn du der deutschen Sprache nicht mächtig bist, kannst du hier \
           festlegen, dass du lieber ein Testat auf Englisch möchtest."
}


unit save_form {
    De => "Speichern",
}

unit flash_success_storing_preferences {
    De => "Die Einstellungen wurden erfolgreich gespeichert.",
}

unit flash_partner_not_a_student(username: &str) {
    De => "Der angegebene Nutzer '{username}' ist kein Student.",
}

unit flash_user_not_found {
    De => "Der angegebene Nutzer existiert nicht in der Datenbank. Hinweis: \
           dein Partner muss sich einmal auf dieser Website eingeloggt haben, \
           damit du ihn/sie als gewünschten Partner angegeben kannst!"
}
