#Projet pour le cours "Projet SGBD"

---
---

## Introduction



## Technologies utilisées
* BACK-END : 

  * [Rust](https://github.com/rust-lang/rust)
   
     * Framework : [Rocket](https://github.com/SergioBenitez/Rocket)
      
     * Voir le fichier 'cargo.toml' pour les autres dépendances

        
* FRONT-END :

  * HTML
  
  * CSS
  
    * [SASS](https://github.com/sass/sass)
    
  * JavaScript

## Au cas où vous voulez build le projet

Il vous faudra tout d'abord installer le language ``Rust`` sur votre machine (via rustup).  
Ainsi qu' ajouter tout le toolchain à la variable d'environnement ``PATH``.


Il vous faudra également créer un fichier 'database.json' à la racine du projet.  
Ce fichier contient la configuration pour se connecter à la base de données.

``database.json``
```json
{
    "ip": "ip pour la connexion (expl: \"127.0.0.1\")",
    "user": "your_username",
    "password": "your_password",
    "database": "your_database",
    "port": "port used by mysql (expl: 3308)"
}
```

---

Il vous faura également une version ``nightly`` de Rust pour build le projet.

Naviguer à la racine du projet et entrez la commande ``rustup override set nightly``.  
Cela installera la version ``nightly`` de Rust et configurera le dossier pour utiliser cette version.

Une fois cela fait, il vous faudra build le projet, entrez la commande :

``cargo build``

Cela téléchargera toutes les dépendances nécessaires en plus de build le projet.

Une fois le projet build, entrez la commande :

``cargo run``

Cela lancera le serveur, ouvrez ensuite le navigateur web de votre choix et rendez vous à l'adresse ``127.0.0.1:8000``.
