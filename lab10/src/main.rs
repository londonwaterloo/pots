use std::io::{Read, Result};

/// Декодер ROT-N, оборачивающий любой источник, реализующий Read.
struct RotDecoder<R: Read> {
    input: R,
    rot: u8,
}

impl<R: Read> Read for RotDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        // Читаем данные из внутреннего источника в буфер
        let n = self.input.read(buf)?;

        // Обрабатываем только прочитанные байты
        for byte in &mut buf[..n] {
            // Проверяем: строчная буква
            if byte.is_ascii_lowercase() {
                // Преобразуем: a-z → 0-25 → сдвиг → обратно
                *byte = ((*byte - b'a' + self.rot) % 26) + b'a';
            }
            // Проверяем: заглавная буква
            else if byte.is_ascii_uppercase() {
                *byte = ((*byte - b'A' + self.rot) % 26) + b'A';
            }
            // Всё остальное (цифры, символы) не трогаем
        }

        Ok(n)
    }
}

fn main() {
    let mut rot = RotDecoder {
        input: "Gb trg gb gur bgure fvqr!".as_bytes(),
        rot: 13,
    };

    let mut result = String::new();

    // read_to_string вызывает наш read внутри
    rot.read_to_string(&mut result).unwrap();

    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn joke() {
        let mut rot =
            RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
        let mut result = String::new();
        rot.read_to_string(&mut result).unwrap();
        assert_eq!(&result, "To get to the other side!");
    }

    #[test]
    fn binary() {
        let input: Vec<u8> = (0..=255u8).collect();
        let mut rot = RotDecoder::<&[u8]> { input: input.as_ref(), rot: 13 };
        let mut buf = [0u8; 256];
        assert_eq!(rot.read(&mut buf).unwrap(), 256);

        for i in 0..=255 {
            if input[i] != buf[i] {
                assert!(input[i].is_ascii_alphabetic());
                assert!(buf[i].is_ascii_alphabetic());
            }
        }
    }
}