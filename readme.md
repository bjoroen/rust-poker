# Rust-poker

Rust-poker is a small server with two end-points, one GET and one POST.

**GET**: Return five random cards and the rank of that hand in the game of poker: pair, full house, two pair, etc

**POST**: It takes five cards as input and returns the rank of that hand.

## End-points

`GET /api/v1/hand`

Response:

```json
{
  "cards": ["2h", "3s", "4r", "5h", "ar"],
  "rank": "Straight"
}
```

`POST /api/v1/hand`

**Request**

```json
{
  "cards": ["2h", "2r", "4s", "5s", "tk"]
}
```

**200 Response**

```json
{
  "rank": "Pair"
}
```

**Error responses**

**400 Response**

`Invalid card` - If one of the cards given are not a valid card

`Not enough cards` - If less than 5 cards are given as input

`Too many cards` - If more than 5 cards are given as input

`Duplicate cards` - If same card is given twice as input

## Run

The server runs on `Port 3000`

**Build docker container**

```bash
$ docker build -t rust-poker .
```

**Run docker container**

```bash
$ docker run -p 3000:3000 rust-poker
```

This should expose the server running inside the docker container to `port 3000`
