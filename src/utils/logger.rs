//! Logger utilities configuráveis via *feature flags*.
//!
//! Este arquivo define reexports de macros de logging (`info!`, `trace!`,
//! `warn!`, `debug!`, `error!`) e funções utilitárias para inicialização do
//! backend de logging apropriado conforme as *features* ativadas.
//!
//! Suporta atualmente as features: `log-itm`, `defmt`, `log-semihost`.
//!
//! A escolha do backend altera quais crates auxiliares são utilizados
//! (ex.: `cortex-m-log`, `defmt-rtt`, `cortex-m-semihosting`).

#![allow(unsafe_code)]

// Primeiro, reexporta as macros/funções de logging dependendo da feature.
cfg_if::cfg_if! {
    // Se a feature `defmt` estiver habilitada, reexportamos os símbolos do
    // `defmt` (que fornece um logger eficiente para embarcados).
    if #[cfg(feature = "defmt")] {
        #[allow(unused_imports)]
        pub use defmt::{info, trace, warn, debug, error};

    // Caso contrário, reexportamos a API compatível com `log`.
    } else {
        #[allow(unused_imports)]
        pub use log::{info, trace, warn, debug, error};
    }
}

// Em seguida, escolhemos o backend concreto para `init()` usando outra
// condição `cfg_if!` — cada branch configura o logger conforme o destino.
cfg_if::cfg_if! {
    // -- Backend: ITM (via `cortex-m-log`) -------------------------------------------------
    if #[cfg(feature = "log-itm")] {
        // Em builds normais, usamos `panic_itm` para saída de panic via ITM.
        #[cfg(not(test))]
        use panic_itm as _;

        // lazy_static para instanciar o LOGGER globalmente.
        use lazy_static::lazy_static;
        use log::LevelFilter;

        // Reexporta o tipo `Logger` para consumidores que queiram usá-lo.
        pub use cortex_m_log::log::Logger;

        // Importa tipos auxiliares do `cortex_m_log` para construir o LOGGER.
        use cortex_m_log::{
            destination::Itm as ItmDest,
            printer::itm::InterruptSync,
            modes::InterruptFree,
            printer::itm::ItmSync
        };

        // Instância estática do LOGGER configurada para ITM.
        lazy_static! {
            pub static ref LOGGER: Logger<ItmSync<InterruptFree>> = Logger {
                level: LevelFilter::Info,
                inner: unsafe {
                    // Criamos um destino ITM sem usar `Peripherals::steal()`.
                    InterruptSync::new(
                        // Transmute é usado para construir o tipo corretamente.
                        ItmDest::new(core::mem::transmute::<(), stm32g4xx_hal::stm32::ITM>(()))
                    )
                },
            };
        }

        // Macro `println!` local que delega para `log::info!()`.
        #[allow(unused_macros)]
        macro_rules! println {
            ($($arg:tt)+) => {
                log::info!($($arg)+);
            };
        }

        // Expõe a macro `println` para uso dentro do crate.
        #[allow(unused_imports)]
        pub(crate) use println;

        // Inicializa o logger global baseado em `LOGGER`.
        #[allow(dead_code)]
        pub fn init() {
            cortex_m_log::log::init(&LOGGER).unwrap();
        }

    // -- Backend: defmt (via RTT) ---------------------------------------------------------
    }
    else if #[cfg(feature = "defmt")] {
        // `defmt_rtt` fornece backend RTT para `defmt`.
        use defmt_rtt as _; // global logger
        #[cfg(not(test))]
        use panic_probe as _;

        #[allow(unused_imports)]
        pub use defmt::Logger;
        #[allow(unused_imports)]
        pub use defmt::println;

        // `defmt` não precisa de inicialização explícita aqui.
        #[allow(dead_code)]
        pub fn init() {}

    // -- Backend: semihosting (via `cortex-m-log`) --------------------------------------
    }
    else if #[cfg(feature = "log-semihost")] {
        #[cfg(not(test))]
        use panic_semihosting as _;

        use lazy_static::lazy_static;
        use log::LevelFilter;

        pub use cortex_m_log::log::Logger;
        use cortex_m_log::printer::semihosting;
        use cortex_m_log::printer::semihosting::Semihosting;
        use cortex_m_log::modes::InterruptOk;
        use cortex_m_semihosting::hio::HStdout;

        // Logger que escreve via semihosting stdout.
        lazy_static! {
            static ref LOGGER: Logger<Semihosting<InterruptOk, HStdout>> = Logger {
                level: LevelFilter::Info,
                inner: semihosting::InterruptOk::<_>::stdout().expect("Get Semihosting stdout"),
            };
        }

        // Macro `println!` que mapeia para hprintln! do semihosting.
        #[allow(unused_macros)]
        macro_rules! println {
            ($s:expr) => {
                cortex_m_semihosting::hprintln!($s).unwrap();
            };
            ($s:expr, $($tt:tt)*) => {
                cortex_m_semihosting::hprintln!($s, $($tt)*).unwrap();
            };
        }

        #[allow(unused_imports)]
        pub(crate) use println;

        #[allow(dead_code)]
        pub fn init() {
            cortex_m_log::log::init(&LOGGER).unwrap();
        }

    // -- Backend padrão: nenhum (fallback) --------------------------------------------
    }
    else {
        // Se nenhum backend de logging foi selecionado, usamos `panic-halt` para
        // definir o comportamento de panic sem imprimir nada.
        #[cfg(not(test))]
        use panic_halt as _;

        // `init()` não precisa fazer nada no fallback.
        #[allow(dead_code)]
        pub fn init() {}
    }
}