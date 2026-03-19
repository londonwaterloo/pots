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
///
/// Возвращаем событие `Event::CarArrived(floor)`.
fn car_arrived(floor: i32) -> Event {
    Event::CarArrived(floor)
}

/// Двери кабины открыты.
///
/// Возвращаем событие `Event::CarDoorOpened`.
fn car_door_opened() -> Event {
    Event::CarDoorOpened
}

/// Двери кабины закрыты.
///
/// Возвращаем событие `Event::CarDoorClosed`.
fn car_door_closed() -> Event {
    Event::CarDoorClosed
}

/// Кнопка вызова лифта нажата на заданном этаже.
///
/// Мы используем "структурный" вариант enum-а:
/// Event::LobbyCallButtonPressed { floor: ..., dir: ... }
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::LobbyCallButtonPressed { floor, dir }
}

/// Кнопка этажа нажата в кабине лифта.
///
/// Возвращаем событие `Event::CarFloorButtonPressed(floor)`.
fn car_floor_button_pressed(floor: i32) -> Event {
    Event::CarFloorButtonPressed(floor)
}

fn main() {
    println!(
        "Пассажир на первом этаже нажал кнопку вызова: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!("Лифт приехал на первый этаж: {:?}", car_arrived(0));
    println!("Дверь лифта открылась: {:?}", car_door_opened());
    println!(
        "Пассажир нажал кнопку третьего этажа: {:?}",
        car_floor_button_pressed(3)
    );
    println!("Двери лифта закрылись: {:?}", car_door_closed());
    println!("Лифт прибыл на третий этаж: {:?}", car_arrived(3));
}