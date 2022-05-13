use crate::prelude::*;

pub fn t(s: &'static str) -> &'static str {
    if SETTINGS.lang() == Lang::French {
        return s;
    }
    
    match s {
        // In agenda.rs
        "Janvier" => "January",
        "Février" => "February",
        "Mars" => "March",
        "Avril" => "April",
        "Mai" => "May",
        "Juin" => "June",
        "Juillet" => "July",
        "Août" => "August",
        "Septembre" => "September",
        "Octobre" => "October",
        "Novembre" => "November",
        "Décembre" => "December",
        "Lundi" => "Monday",
        "Mardi" => "Tuesday",
        "Mercredi" => "Wednesday",
        "Jeudi" => "Thursday",
        "Vendredi" => "Friday",
        "Samedi" => "Saturday",
        "Dimanche" => "Sunday",
        "Options" => "Options",

        // In calendar.rs
        "Lun" => "Mon",
        "Mar" => "Tue",
        "Mer" => "Wed",
        "Jeu" => "Thu",
        "Ven" => "Fri",
        "Sam" => "Sat",
        "Dim" => "Sun",

        // In change_password.rs
        "Tous les champs doivent être remplis." => "All fields must be filled.",
        "Les mots de passe ne correspondent pas." => "Passwords do not match.",
        "Le nouveau mot de passe doit être différent du mot de passe actuel." => "New password must be different from the current one.",
        "Une erreur inconnue est survenue. Veuillez contacter le support: support@insagenda.fr" => "An unknown error has occurred. Please contact the support: support@insagenda.fr",
        "Impossible de se connecter au le serveur. Veuillez contacter le support: support@insagenda.fr" => "Unable to connect to the server. Please contact the support: support@insagenda.fr",
        "Changement de mot de passe" => "Password change",
        "Changer son mot de passse" => "Change password",
        "Mot de passe actuel" => "Current password",
        "Nouveau mot de passe" => "New password",
        "Nouveau mot de passe (confirmation)" => "New password (confirmation)",
        "Confirmer" => "Confirm",

        // In event.rs
        "Début : " => "Start: ",
        "Fin : " => "End: ",
        "Durée : " => "Duration: ",
        "Groupes : " => "Groups: ",
        "Groupe : " => "Group: ",
        "Professeur : " => "Teacher: ",
        "Salle : " => "Room: ",
        "Inconnue" => "Unknown",
        "Changer les couleurs" => "Change colors",
        "Fond" => "Background",
        "Texte" => "Text",
        "Annuler" => "Cancel",
        "Sauvegarder" => "Save",

        
        // In settings.rs
        "[indisponible]" => "[unavailable]",
        "[inconnue]" => "[unknown]",
        "Paramètres" => "Settings",
        "Général" => "General",
        "Paramètres du compte" => "Account parameters",
        "Mot de passe" => "Password",
        "Votre mot de passe a été changé il y a" => "Your password was changed ",
        "Modifier" => "Modify",
        "Adresse mail" => "Email address",
        "Votre adresse actuelle est" => "Your current email address is",
        " Elle n'a pas encore été vérifiée." => " It has not been verified yet.",
        "Changer de classe" => "Change class",
        "Vous êtes actuellement en STPI1 dans le groupe B.1 section 2." => "You are currently in STPI1 in the group B.1 section 2.",
        "Changer le type d'authentification" => "Change authentication type",
        "Vérifier mon adresse Email" => "Verify my email address",
        "Email" => "Email",
        "Affichage" => "Appearance",
        "Changer / Vérifier" => "Edit / Verify",
        "Thème" => "Theme",
        "Sombre" => "Dark",
        "Clair" => "Light",
        "Par défault, le thème est celui renseigné par votre navigateur." => "By default, the theme is the one set by your browser.",
        "Langue" => "Language",
        "Nom des bâtiments" => "Building names",
        "Court" => "Short",
        "Long" => "Long",
        "Système" => "System",
        "Valider" => "Save",
        "Se déconnecter" => "Log out",
        "Changer de mot de passse" => "Change password",
        s => {
            log!("Untranslated string: {}", s);
            s
        }
    }
}
