// ======================================================
// ЗАДАНИЕ 5 — Перемещение лифта
//
// Идея:
// - Event (событие) = то, что произошло в системе лифта
//   (нажали кнопку, приехали на этаж, двери открылись/закрылись)
//
// В этом задании мы НЕ пишем контроллер лифта,
// только описываем события и функции-конструкторы.
//
// #[derive(Debug)] позволяет печатать через {:?}
// ======================================================

#[derive(Debug)]
/// Событие лифта, на которое должен реагировать контроллер.
enum Event {
    /// Кабина приехала на некоторый этаж.
    CarArrived(i32),

    /// Двери кабины открылись.
    CarDoorOpened,

    /// Двери кабины закрылись.
    CarDoorClosed,

    /// На этаже нажали кнопку вызова лифта (вверх/вниз).
    LobbyCallButtonPressed {
        floor: i32,
        dir: Direction,
    },

    /// Внутри кабины нажали кнопку этажа.
    CarFloorButtonPressed(i32),
}

/// Направление движения (вверх/вниз).
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

/// Кабина приехала на заданный этаж.
fn car_arrived(floor: i32) -> Event {
    Event::CarArrived(floor)
}

/// Двери кабины открыты.
fn car_door_opened() -> Event {
    Event::CarDoorOpened
}

/// Двери кабины закрыты.
fn car_door_closed() -> Event {
    Event::CarDoorClosed
}

/// Кнопка вызова лифта нажата на заданном этаже.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::LobbyCallButtonPressed { floor, dir }
}

/// Кнопка этажа нажата в кабине лифта.
fn car_floor_button_pressed(floor: i32) -> Event {
    Event::CarFloorButtonPressed(floor)
}

/// Превращает событие в понятное текстовое описание.
///
/// Здесь мы явно читаем поля enum-вариантов, поэтому
/// warning'и о "field is never read" исчезнут.
fn describe_event(event: &Event) -> String {
    match event {
        Event::CarArrived(floor) => {
            format!("Лифт прибыл на этаж {}", floor)
        }
        Event::CarDoorOpened => {
            "Двери лифта открылись".to_string()
        }
        Event::CarDoorClosed => {
            "Двери лифта закрылись".to_string()
        }
        Event::LobbyCallButtonPressed { floor, dir } => {
            let direction = match dir {
                Direction::Up => "вверх",
                Direction::Down => "вниз",
            };
            format!(
                "На этаже {} нажали кнопку вызова лифта в направлении {}",
                floor, direction
            )
        }
        Event::CarFloorButtonPressed(floor) => {
            format!("В кабине нажали кнопку этажа {}", floor)
        }
    }
}

fn main() {
    let events = [
        lobby_call_button_pressed(0, Direction::Up),
        car_arrived(0),
        car_door_opened(),
        car_floor_button_pressed(3),
        car_door_closed(),
        car_arrived(3),
        lobby_call_button_pressed(5, Direction::Down),
    ];

    for event in &events {
        println!("{}", describe_event(event));
    }
}