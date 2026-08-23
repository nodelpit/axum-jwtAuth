# Authentification par token (JWT) — Le modèle mental

Comment marche le flux JWT, indépendamment d'un langage. But : le **modèle mental**, pas le code.

---

## 1. Le problème

Client/serveur = **sans mémoire** : le serveur ne se souvient pas d'une requête à l'autre. Il faut donc une **preuve d'identité** qui :

- est délivrée **une fois** (après vérification) ;
- **voyage** avec chaque requête ;
- se vérifie **vite**, sans recontrôler le mot de passe ;
- est **infalsifiable**.

Cette preuve = le **JWT**.

---

## 2. L'idée centrale

- **Session** : le serveur stocke qui est connecté → *la vérité vit sur le serveur*.
- **JWT** : la preuve est **autoportante**, transportée par le client → *la vérité voyage dans le token*.

Le serveur délègue la mémoire au client, mais se protège avec une **signature**.

---

## 3. Anatomie du token

3 parties, **encodées (lisibles), pas chiffrées**, jointes par des points :

```
                          Token
                            │
        ┌───────────────────┼───────────────────┐
        ▼                   ▼                   ▼
     En-tête            Charge utile         Signature
     (header)        (payload / claims)
        │                   │                   │
        ▼                   ▼                   ▼
  Décrit comment      Transporte les       Preuve que
  le token est        données : qui,       l'en-tête et la
  signé               émis quand,          charge n'ont pas
                      expire quand         été modifiés et
                                           proviennent bien
                                           du serveur
```

**Point clé : la charge est lisible par tous → jamais de secret dedans.**

Les **claims** (contenu de la charge) :

- `iat` : émis quand · `exp` : expire quand · `sub` : identité (id, email…)
- + données personnalisées (rôle, droits…)

---

## 4. La signature = la confiance

Le serveur produit la signature avec sa **clé** :

```
     En-tête  +  Charge utile
              │
              ▼
   Le serveur applique sa clé
   de signature
              │
              ▼
          Signature
              │
              ▼
   Token = en-tête . charge . signature
```

- **Intégrité** : un caractère modifié → signature invalide → rejet.
- **Authenticité** : seul le serveur (qui a la clé) peut en produire une valide. Le client lit, mais ne forge pas.

Vérifier = refaire le contrôle sur le token présenté.

---

## 5. Les deux phases

- **Authentification** : prouver qui on est. **Une fois**, à la connexion → un token.
- **Autorisation** : présenter le token. **À chaque** requête protégée.

### Phase 1 — Obtenir le token

```
Client
  │  envoie son identifiant + son mot de passe
  ▼
Serveur : retrouve l'utilisateur correspondant
  │
  ▼
Serveur : compare le mot de passe fourni
  │        à l'empreinte stockée
  ▼
  ┌──────────────────┴──────────────────┐
  ▼                                      ▼
Empreinte différente               Empreinte identique
  │                                      │
  ▼                                      ▼
Accès refusé                       Serveur : construit la charge
                                   (identité + iat + exp)
                                        │
                                        ▼
                                   Serveur : signe le token
                                   avec sa clé de signature
                                        │
                                        ▼
                                   Renvoie le token au client
                                        │
                                        ▼
                                   Client : conserve le token
```

Le mot de passe n'est jamais stocké en clair : seule une **empreinte** (hash à sens unique) l'est, et on vérifie qu'elle correspond.

### Phase 2 — Utiliser le token

```
Client
  │  requête vers une ressource protégée
  │  token placé dans l'en-tête d'autorisation
  ▼
Point de contrôle : extrait le token de l'en-tête
  │
  ▼
Point de contrôle : vérifie la signature
  │                  ET la date d'expiration
  ▼
  ┌──────────────────┴──────────────────┐
  ▼                                      ▼
Signature invalide                   Signature valide
ou token expiré                      et token encore valide
  │                                      │
  ▼                                      ▼
Accès refusé                        Lit l'identité dans la charge
                                        │
                                        ▼
                                   Retrouve l'utilisateur complet
                                        │
                                        ▼
                                   Attache l'utilisateur à la requête
                                        │
                                        ▼
                                   La ressource protégée s'exécute
                                        │
                                        ▼
                                   Réponse renvoyée au client
```

Le **point de contrôle** filtre **avant** la ressource : pas de token valide → ressource jamais atteinte.

---

## 6. Le token « porteur » (Bearer)

Qui détient le token peut l'utiliser. D'où :

- **Transport chiffré** (sinon rejouable après interception).
- **Expiration** (borne la casse en cas de vol).

---

## 7. Vue d'ensemble

```
        ┌─────────────────────────────────┐
        │   PHASE 1 — AUTHENTIFICATION     │
        └─────────────────────────────────┘
                        │
        Le client prouve son identité
        (une seule fois)
                        │
                        ▼
        Le serveur renvoie un token signé
                        │
                        ▼
        Le client conserve le token
                        │
                        ▼
        ┌─────────────────────────────────┐
        │   PHASE 2 — AUTORISATION         │
        └─────────────────────────────────┘
                        │
        Le client rejoue le token à
        chaque requête protégée
                        │
                        ▼
        Le point de contrôle vérifie
        signature + expiration
                        │
              ┌─────────┴─────────┐
              ▼                   ▼
          Invalide             Valide
              │                   │
              ▼                   ▼
        Accès refusé        Accès accordé
                            (jusqu'à expiration)
```

---

## 8. Limites

- **Révocation difficile** : rien stocké → dur d'invalider avant expiration. (Remèdes : durée courte, liste de révocation, rotation.)
- **Charge lisible** : aucun secret dedans.
- **Taille** : voyage à chaque requête → plus de claims = requêtes plus lourdes.
- **Stockage côté client** : sujet de sécurité à part entière.

---

## À retenir

- **2 phases** : s'authentifier **une fois**, s'autoriser **à chaque requête**.
- Token **autoportant** : le serveur **vérifie**, ne **se souvient** pas.
- **Signature** = intégrité + authenticité (anti-falsification).
- Charge **lisible, pas secrète**.
- **Porteur** → transport chiffré + expiration.