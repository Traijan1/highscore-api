# Highscore

This is a simple API for everyone who want to host a Highscore API for his app/game/whatever.

---

## API

Every route has to begin with: `your-domain-or-ip/api`

- **[GET]** /score?id
  - Get score by id.
- **[GET]** /scores?offset&limit
  - Get all scores, limit and offset are optionally. Default limit is set to 100.
- **[POST]** /score
  - Create a new score entry with following data:
    - name   -> String
    - score  -> float64
    - custom -> Your own JSON Data (optionally)
- **[PUT]** /score
  - Update your score entry by id (id has to be in json body currently)
- **[DELETE]** /score?id
  - Delete a single score entry by id
