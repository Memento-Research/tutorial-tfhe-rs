use std::io::Write;
use std::path::Path;
use std::time::Instant;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint8};
use tfhe::{prelude::*, Config, FheUint16, FheUint32, FheUint64};
enum IntegerType {
    Uint8,
    Uint16,
    Uint32,
    Uint64,
}

impl IntegerType {
    fn benchmark(self: &Self) {
        let (path, text_to_print) = match self {
            IntegerType::Uint8 => {
                let config = ConfigBuilder::all_disabled().enable_default_uint8().build();
                let text_to_print = benchmark(config, IntegerType::Uint8, 1, 1);
                let path = Path::new("benchmarks/benchmark_u8.txt");
                (path, text_to_print)
            }
            IntegerType::Uint16 => {
                let config = ConfigBuilder::all_disabled()
                    .enable_default_uint16()
                    .build();
                let text_to_print = benchmark(config, IntegerType::Uint16, 1, 1);
                let path = Path::new("benchmarks/benchmark_u16.txt");
                (path, text_to_print)
            }
            IntegerType::Uint32 => {
                let config = ConfigBuilder::all_disabled()
                    .enable_default_uint32()
                    .build();
                let text_to_print = benchmark(config, IntegerType::Uint32, 1, 1);
                let path = Path::new("benchmarks/benchmark_u32.txt");
                (path, text_to_print)
            }
            IntegerType::Uint64 => {
                let config = ConfigBuilder::all_disabled()
                    .enable_default_uint64()
                    .build();
                let text_to_print = benchmark(config, IntegerType::Uint64, 1, 1);
                let path = Path::new("benchmarks/benchmark_u64.txt");
                (path, text_to_print)
            }
        };

        let mut file = match std::fs::File::create(path) {
            Ok(file) => file,
            Err(_) => panic!("Error creating file"),
        };

        match file.write_all(text_to_print.as_bytes()) {
            Ok(_) => println!("File saved"),
            Err(_) => println!("Error saving file"),
        }
    }

    fn benchmark_all() {
        IntegerType::Uint8.benchmark();
        IntegerType::Uint16.benchmark();
        IntegerType::Uint32.benchmark();
        IntegerType::Uint64.benchmark();
    }
}

fn benchmark(config: Config, integer_type: IntegerType, clear_a: u64, clear_b: u64) -> String {
    let mut text_to_print = String::new();
    let total_time = Instant::now();

    let time_keys = Instant::now();
    let (client_key, server_key) = generate_keys(config);
    text_to_print.push_str(format!("Time to generate keys: {:?}\n", time_keys.elapsed()).as_str());

    set_server_key(server_key);

    match integer_type {
        IntegerType::Uint8 => {
            let time_encrypt_a = Instant::now();
            let encrypted_a = FheUint8::encrypt(clear_a as u8, &client_key);
            text_to_print
                .push_str(format!("Time to encrypt a: {:?}\n", time_encrypt_a.elapsed()).as_str());

            let time_encrypt_b = Instant::now();
            let encrypted_b = FheUint8::encrypt(clear_b as u8, &client_key);
            text_to_print
                .push_str(format!("Time to encrypt b: {:?}\n", time_encrypt_b.elapsed()).as_str());

            let time_add = Instant::now();
            let encrypted_sum = encrypted_a + encrypted_b;
            text_to_print.push_str(format!("Time to add: {:?}\n", time_add.elapsed()).as_str());

            let time_decrypt = Instant::now();
            let _decrypted_sum: u8 = encrypted_sum.decrypt(&client_key);
            text_to_print
                .push_str(format!("Time to decrypt: {:?}\n", time_decrypt.elapsed()).as_str());
        }
        IntegerType::Uint16 => {
            let time_encrypt_a = Instant::now();
            let encrypted_a = FheUint16::encrypt(clear_a as u16, &client_key);
            text_to_print
                .push_str(format!("Time to encrypt a: {:?}\n", time_encrypt_a.elapsed()).as_str());

            let time_encrypt_b = Instant::now();
            let encrypted_b = FheUint16::encrypt(clear_b as u16, &client_key);
            text_to_print
                .push_str(format!("Time to encrypt b: {:?}\n", time_encrypt_b.elapsed()).as_str());

            let time_add = Instant::now();
            let encrypted_sum = encrypted_a + encrypted_b;
            text_to_print.push_str(format!("Time to add: {:?}\n", time_add.elapsed()).as_str());

            let time_decrypt = Instant::now();
            let _decrypted_sum: u16 = encrypted_sum.decrypt(&client_key);
            text_to_print
                .push_str(format!("Time to decrypt: {:?}\n", time_decrypt.elapsed()).as_str());
        }
        IntegerType::Uint32 => {
            let time_encrypt_a = Instant::now();
            let encrypted_a = FheUint32::try_encrypt(clear_a as u32, &client_key).unwrap();
            text_to_print
                .push_str(format!("Time to encrypt a: {:?}\n", time_encrypt_a.elapsed()).as_str());

            let time_encrypt_b = Instant::now();
            let encrypted_b = FheUint32::try_encrypt(clear_b as u32, &client_key).unwrap();
            text_to_print
                .push_str(format!("Time to encrypt b: {:?}\n", time_encrypt_b.elapsed()).as_str());

            let time_add = Instant::now();
            let encrypted_sum = encrypted_a + encrypted_b;
            text_to_print.push_str(format!("Time to add: {:?}\n", time_add.elapsed()).as_str());

            let time_decrypt = Instant::now();
            let _decrypted_sum: u32 = encrypted_sum.decrypt(&client_key);
            text_to_print
                .push_str(format!("Time to decrypt: {:?}\n", time_decrypt.elapsed()).as_str());
        }
        IntegerType::Uint64 => {
            let time_encrypt_a = Instant::now();
            let encrypted_a = FheUint64::try_encrypt(clear_a, &client_key).unwrap();
            text_to_print
                .push_str(format!("Time to encrypt a: {:?}\n", time_encrypt_a.elapsed()).as_str());

            let time_encrypt_b = Instant::now();
            let encrypted_b = FheUint64::try_encrypt(clear_b, &client_key).unwrap();
            text_to_print
                .push_str(format!("Time to encrypt b: {:?}\n", time_encrypt_b.elapsed()).as_str());

            let time_add = Instant::now();
            let encrypted_sum = encrypted_a + encrypted_b;
            text_to_print.push_str(format!("Time to add: {:?}\n", time_add.elapsed()).as_str());

            let time_decrypt = Instant::now();
            let _decrypted_sum: u64 = encrypted_sum.decrypt(&client_key);
            text_to_print
                .push_str(format!("Time to decrypt: {:?}\n", time_decrypt.elapsed()).as_str());
        }
    }

    text_to_print.push_str(format!("Total time: {:?}\n", total_time.elapsed()).as_str());
    return text_to_print;
}

fn main() {
    // Todo proceso de TFHE tiene como principales etapas:
    // 1. Generar las claves de cifrado
    // 2. Enviar la clave pública al servidor
    // 3. Cifrar los datos
    // 4. Enviar los datos cifrados al servidor
    // 5. Realizar las operaciones en el servidor
    // 6. Recuperar los datos cifrados
    // 7. Descifrar los datos

    IntegerType::benchmark_all();
}
