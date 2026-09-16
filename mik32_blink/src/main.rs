// blink according to google ai regarding on mik32-pac

#![no_std]
#![no_main]

use panic_halt as _;
use riscv_rt::entry;

// Используем библиотеку в соответствии с вашим Cargo.toml
use mik32_pac as pac;

fn delay(cycles: u32) {
    for _ in 0..cycles {
        core::hint::black_box(());
    }
}

#[entry]
fn main() -> ! {
    // 1. Получаем периферию. Все поля теперь строго в snake_case (строчные буквы)
    let peripherals = pac::Peripherals::take().unwrap();

    // let pm = &peripherals.pm;// alexeit fix do not get pm since (*mik32_pac::Pm::ptr()) below
    let gpio_0 = &peripherals.gpio16_0; // alexeit fix
    // let gpio_0 = &peripherals.gpio_0;
    let pad_config = &peripherals.pad_config;

    // 2. Включаем тактирование GPIO_0.
    // Название регистра включения APB периферии в MIK32 обычно называется `clk_apb_set`
    // Проверим его в строчном формате:
    unsafe {
     (*mik32_pac::Pm::ptr()).clk_apb_p_set().modify(|_, w| w.gpio_0().set_bit()); // alexeit fix
    }
    //  (*mik32_pac::Pm::ptr()).clk_ahb_set().modify(|_, w| w.gpio_0().set_bit());
    //  (*mik32_pac::Pm::ptr()).clk_ahb_set.modify(|_, w| w.gpio_0().set_bit());
    // mik32_pac::Pm::ptr().clk_ahb_set.modify(|_, w| w.gpio_0().set_bit());
    // pm.ptr().clk_ahb_set.modify(|_, w| w.gpio_0().set_bit());
    // pm.clk_apb_set.modify(|_, w| w.gpio_0().set_bit());

    // 3. Конфигурация вывода PORT0_2 (LED на ELBEAR ACE-NANO)
    // В SVD-файлах порты часто нумеруются через массивы или сдвоенные регистры.
    // Если `port0_2` выдал ошибку, значит в PadConfig регистры называются по номерам пинов.
    // Попробуем универсальный безопасный доступ к регистру конфигурации пина:
    unsafe {
        pad_config.pad0_cfg().modify(|_, w| w.bits(0)); // alexeit fix
        // pad_config.pad0_cfg().modify(|_, w| w.mux().bits(0));
        // pad_config.pad0_cfg().Port0_2.modify(|_, w| w.mux().bits(0));
        // pad_config.port0_2.modify(|_, w| w.mux().bits(0));
    }

    // 4. Устанавливаем направление PORT0_2 как выход (Output)
    unsafe {
        // Выставляем 2-й бит в единицу в регистре направления (direction)
        gpio_0.direction_out().modify(|r, w| w.bits(r.bits() | (1 << 2)));
        // gpio_0.direction.modify(|r, w| w.bits(r.bits() | (1 << 2)));
    }

    // 5. Цикл мигания
    loop {
        unsafe {
            // Включаем светодиод (PORT0_2 = 1)
            gpio_0.output().modify(|r, w| w.bits(r.bits() | (1 << 2)));
        }
        delay(2_000_000);

        unsafe {
            // Выключаем светодиод (PORT0_2 = 0)
            gpio_0.output().modify(|r, w| w.bits(r.bits() & !(1 << 2)));
        }
        delay(2_000_000);
    }
}


// #![no_std]
// #![no_main]

// use panic_halt as _;
// use riscv_rt::entry;

// // Используем точное имя библиотеки из вашего Cargo.toml
// use mik32_pac as pac;

// fn delay(cycles: u32) {
//     for _ in 0..cycles {
//         core::hint::black_box(());
//     }
// }

// #[entry]
// fn main() -> ! {
//     // 1. Берем периферию в точности как в примерах репозитория
//     let peripherals = pac::Peripherals::take().unwrap();

//     let pm = &peripherals.pm;
//     let gpio_0 = &peripherals.Gpio0;
//     let pad_config = &peripherals.pad_config;

//     // 2. Включаем тактирование GPIO_0
//     // По синтаксису svd2rust, если поле простое, мы взводим его через set_bit() внутри modify
//     pm.clk_apb_set.modify(|_, w| w.gpio_0().set_bit());

//     // 3. Конфигурация вывода PORT0_2 (LED на ELBEAR ACE-NANO)
//     // В MIK32 нужно переключить пин в режим GPIO. В PAD_CONFIG для каждого пина есть регистр.
//     // Обычно они называются port0_2 или pin_2. Настроим функцию 0 (GPIO).
//     pad_config.port0_2.modify(|_, w| unsafe { w.mux().bits(0) });

//     // 4. Устанавливаем направление PORT0_2 как выход (Output)
//     // В зависимости от версии SVD, направление задается либо битовой маской, либо через именованное поле pin
//     unsafe {
//         // Безопасный способ выставить 2-й бит в единицу через чтение-модификацию-запись
//         gpio_0.direction.modify(|r, w| w.bits(r.bits() | (1 << 2)));
//     }

//     // 5. Цикл мигания
//     loop {
//         unsafe {
//             // Включаем светодиод (PORT0_2 = 1)
//             gpio_0.output.modify(|r, w| w.bits(r.bits() | (1 << 2)));
//         }
//         delay(2_000_000);

//         unsafe {
//             // Выключаем светодиод (PORT0_2 = 0)
//             gpio_0.output.modify(|r, w| w.bits(r.bits() & !(1 << 2)));
//         }
//         delay(2_000_000);
//     }
// }



// // blink according to google ai
// #![no_std]
// #![no_main]

// use panic_halt as _;
// use riscv_rt::entry;
// use mik32_pac;

// // Простейшая функция задержки на "пустых" циклах процессора.
// // При тактовой частоте ~32 МГц это даст примерно заметную задержку.
// fn delay(cycles: u32) {
//     for _ in 0..cycles {
//         // inline-ассемблер 'nop' предотвращает удаление цикла оптимизатором компилятора
//         core::hint::black_box(());
//     }
// }

// #[entry]
// fn main() -> ! {
//     // 1. Получаем доступ ко всей периферии К1948ВК018
//     let peripherals = mik32_pac::Peripherals::take().unwrap();
//     let pm = &peripherals.pm;
//     // let pm = &peripherals.PM;
//     let gpio_0 = &peripherals.gpio_0;
//     // let gpio_0 = &peripherals.GPIO_0;
//     let pad_config = &peripherals.pad_config;
//     // let pad_config = &peripherals.PAD_CONFIG;

//     // 2. Включаем тактирование GPIO_0 в Power Manager (PM)
//     // Разрешаем тактирование шины периферии и самого модуля GPIO
//     pm.clk_ahb_set().write(|w| w.gpio_0().set_bit());
//     // pm.clk_apb_set().write(|w| w.gpio_0().set_bit());

//     // 3. Настраиваем вывод PORT0_2 на плате ELBEAR ACE-NANO
//     // Сначала настраиваем режим вывода (PAD CONFIG) — переключаем в режим GPIO (функция 0)
//     // И устанавливаем направление в GPIO_0 как выход (Output)
//     unsafe {
//         // Устанавливаем бит направления для 2-го пина (1 -> Выход, 0 -> Вход)
//         gpio_0.direction().modify(|r, w| w.bits(r.bits() | (1 << 2)));
//     }

//     // 4. Бесконечный цикл мигания
//     loop {
//         unsafe {
//             // Включаем светодиод (подаем логическую 1 на PORT0_2)
//             gpio_0.output().modify(|r, w| w.bits(r.bits() | (1 << 2)));
//         }
//         delay(1_000_000);

//         unsafe {
//             // Выключаем светодиод (сбрасываем в 0)
//             gpio_0.output().modify(|r, w| w.bits(r.bits() & !(1 << 2)));
//         }
//         delay(1_000_000);
//     }
// }



// minimal app according to google ai

// #![no_std]
// #![no_main]

// use panic_halt as _; // Подключаем обработчик паники
// use riscv_rt::entry;

// // Хитрый импорт: активирует реализацию _critical_section_1_0_acquire
// // use riscv_critical_section as _;

// // Автоматически генерирует символы _max_hart_id (0) и _hart_stack_size для одного ядра
// // riscv::device!(cores = 1);

// #[entry]
// fn main() -> ! {
//     // Получаем доступ ко всей периферии чипа
//     let peripherals = mik32_pac::Peripherals::take().unwrap();

//     // Пример: Инициализируем или читаем регистры (управление тактированием, GPIO и т.д.)
//     // let pm = &peripherals.PM;
//     // let gpio_0 = &peripherals.GPIO_0;

//     loop {
//         // Ваш код управления периферией «Амура»
//     }
// }
