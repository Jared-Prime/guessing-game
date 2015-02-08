package main

import (
  "fmt"
  "math/rand"
  "time"
)

const (
  youWin  = iota
  tooLow  = iota
  tooHigh
)

func main() {
  fmt.Print("Guess a number between 1 and 100\n")

  rand.Seed( time.Now().UTC().UnixNano())

  // the secret number
  secret := (rand.Uint32() % 100) + 1

  // mutable variables
  var guess uint32
  var status int
  var mesg string

  for {
    _, err := fmt.Scanf("%d", &guess)

    if err != nil {
      fmt.Print("input must be a number")
    }

    mesg, status = cmp(guess, secret)

    fmt.Print(mesg)

    if status == 0 {
      return
    }
  }
}

func cmp(guess uint32, secret uint32) (string, int) {
  switch {
    case guess < secret:
      return "Too low; try again\n", tooLow
    case guess > secret:
      return "Too high; try again\n", tooHigh
    case guess == secret:
      return "You win!\n", youWin
  }
  return "unknown", 0
}
