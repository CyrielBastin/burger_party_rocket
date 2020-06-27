# Projet pour le cours "Projet SGBD"

---
---

## Introduction

Ceci est ma version du projet ``Fastfood`` pour le cours de SGBD.  
Je me suis concentré sur la partie ``commande`` du projet.

L'idée est de simuler un terminal pour passer notre commande.  
Comme lorsque vous allez au McDonald par exemple, au lieu de passer commande auprès du caissier, il est possible de  
passer commande auprès d'un terminal pour ensuite récupérer notre commande lorsqu' elle est prête.


### Fonctionnalités

1. Le projet contient une partie ``Qui sommes nous`` afin de présenter l'entreprise aux clients qui souhaitent en savoir  
plus sur notre marque.

2. Une autre partie est dédiée à l'information sur nos produits. Dans cette section, les clients peuvent se renseigner  
sur nos différents produits ainsi que leur composition de sorte qu'ills puissent choisir leur menu en fonction de  
leurs envies et/ou allergies.

3. La dernière partie concerne la commande des produits. Les clients peuvent choisir les burgers et boissons qu'ils  
désirent ajouter à leur commande.  
Une fois leur choix fait, les clients ont accès à un "résumé" de leur commande dans lequel ils peuvent trouver :  
-- Un récapitulatif des produits (combien de burgers/boissons ils ont commandé).  
-- Le nombre de calories que contient la commande.  
-- Le prix total à payer.

Une fois leur commande payée, les clients récupèrent le ticket et se dirigent au comptoir où ils attendront que leur  
commande soit prête avant de la récupérer et de déguster les savoureux mets préparés par nos soins.

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

Il vous faudra tout d'abord installer ``Rust`` sur votre machine (via rustup).  
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

Cela lancera le serveur, ouvrez ensuite le navigateur web de votre choix et rendez vous à l'adresse ``localhost:8000``.
