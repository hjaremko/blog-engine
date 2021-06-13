# Silnik blogowy - Hubert Jaremko
Programowanie dla WWW, rok akademicki 2020/2021

## Cel projektu

Celem projektu było zbudowanie kompleksowej platformy
blogowej umożliwającej administratorowi
dodawanie nowych postów, zarejestrowanym użytkownikom
dodawanie komentarzy pod postami
oraz wszystkim innym odwiedzającym wyświetlanie zarówno postów
jak i komentarzy.

## Zrealizowane funkcjonalności

### Frontend
 - **Accordion** w formie rozwijanego podglądu komentarzy pod postami.
 - **Dotyczywanie postów przy scrollowaniu.**
 - Dekoracyjny **parallax scrolling** jako belka z tytułem postu.
 - Logowanie na konto.
 - Dodawnie nowych postów i komentarzy.

### Backend
 - Baza danych
   - 3 tabele: użytkownicy, posty, komentarze.
 - Autoryzacja użytkowników poprzez token JWT wygasający po 60 minutach.
 - System uprawnień:
  - `ADMIN` posiadający dostęp do wszystkich zapytań,
  - `USER` posiadający dostęp tylko do dodawania komentarzy i wyświetlania tresci.

## Wykorzystane techonologie

### Frontend
 - React.Js
 - axios
 - react-parallax
 - universal-cookie
 - Bootstrap 5
 - FontAwesome

### Backend
 - Całość backendu napisana w języku **Rust**.
 - Wykorzystany framework: `rocket` w wersji `0.4.8`.
 - Baza danych **SQLite**.

## Zrzuty ekranu
![](screenshots/homepage.png)
![](screenshots/comments.png)
![](screenshots/newcomment.png)
![](screenshots/invalidlogin.png)

## Przykładowe wywołania API

### Pobranie wszystkich postów
```
curl --location --request GET 'localhost:8000/api/posts'
```

### Pobranie trzech postów z pierwszej strony
```
curl --location --request GET 'localhost:8000/api/posts?page=0&limit=3'
```

### Pobranie komentarzy posta o id 9
```
curl --location --request GET 'localhost:8000/api/posts/comments/9'
```

### Zalogowanie się jako administator
W odpowiedzi otrzymujemy token JWT oraz zakres uprawnień.
```
curl --location --request POST 'localhost:8000/api/login' \
--header 'Content-Type: application/json' \
--data-raw '{
    "login": "admin",
    "password": "admin"
}'
```

### Założenie nowego konta
```
curl --location --request POST 'localhost:8000/api/register' \
--header 'Content-Type: application/json' \
--data-raw '{
    "login": "hubert",
    "password": "jaremko",
    "name": "Hubert J"
}'
```

### Dodanie nowego posta, wymagany token

```
curl --location --request POST 'localhost:8000/api/posts' \
--header 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJsb2dpbiI6Imh1YmVydCIsInBhc3N3b3JkIjoiaGFzbG8iLCJyb2xlIjoiVVNFUiIsImV4cCI6MTYyMzU4Njg0NX0.qrx5iSG1SLIQ0ciQeF92Ilq7T4t2niU9T1eipQJTpED9zVZcrvjNcwh8jQi1xfr6Ltqm8p46DEnBSuLW691ohg' \
--header 'Content-Type: application/json' \
--data-raw '{
    "title": "Tytuł nowego posta",
    "content": "Nowy post, wymagane logowanie."
}'
```

### Dodanie nowego komentarza pod postem o id 13, wymagany token

```
curl --location --request POST 'localhost:8000/api/posts/comments/13' \
--header 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJsb2dpbiI6Imh1YmVydCIsInBhc3N3b3JkIjoiaGFzbG8iLCJyb2xlIjoiVVNFUiIsImV4cCI6MTYyMzU4Njg0NX0.qrx5iSG1SLIQ0ciQeF92Ilq7T4t2niU9T1eipQJTpED9zVZcrvjNcwh8jQi1xfr6Ltqm8p46DEnBSuLW691ohg' \
--header 'Content-Type: text/plain' \
--data-raw 'Nowy komentarz.'
```

## Budowa i uruchomienie projektu

### Krok 1 - Budowa frontendu
```
cd frontend
npm run build
```

### Krok 2 - Instalacja Rust nightly

https://www.rust-lang.org/tools/install
```
rustup toolchain install nightly
```

### Krok 3 - Budowa i uruchomienie backendu
```
cargo +nightly run
```

Po kompilacji i uruchomieniu, klient oraz serwer są dostępne na porcie 8000.

