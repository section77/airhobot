# stepper: Steuerung der Schrittmotoren. #

Aktuell ein eigenes _crate_, um:

  - die Schrittmotoren zu testen (ohne die ganze App auf dem raspberry zu bauen)
  - einen stepper actor server zu bauen, welcher udp-telegramme empfängt und diese als Schritte
    für den Schrittmotor interpretiert. Dadurch kann die App auf einem normal Rechner implementiert werden.

## Inhalt ##

### crate (`src/lib.rs`) ###

```rust
let mut stepper = Stepper::new("Stepper 1", EnablePin(6), StepPin(3), DirectionPin(4)).unwrap();
stepper.enable();

// einfacher schritt
stepper.step(Direction::Left);
stepper.step(Direction::Right);

// mehrere schritte
stepper.step_n(Direction::Left, 100);
stepper.step_n(Direction::Right, 100);

stepper.disable();
```

### Schrittmotoren testen (`src/bin/simple-stepper-test.rs`) ###

  - pin's sind hardcodiert im source
  - run:

    > cargo run --bin simple-stepper-test


### `airhobot` stepper actor (`src/bin/airhobot-stepper-actor.rs`) ###

  - pin's sind hardcodiert im source
  - lauscht auf dem udp socket: 0.0.0.0:6789
  - erwartet als Telegram zwei Zahlen, welche mit einem `:` getrennt sind.
    - die Zahlen werden als Schritte interpretiert
    - erste Zahl für den Linken, die Zweite für den Rechten Motor
    - Format: `[+|-]\d:[+|-]\d`
    - positive Zahlen: Schritte nach rechts
    - negative Zahlen: Schritte nach links
  - run:

    > cargo run --bin airhobot-stepper-actor

  - test - linker Motor 20 Schritte nach rechts, rechter Motor 10 Schritte nach links

    > echo 20:-10 | nc -u 192.168.1.222 6789

