//! Exemplo mínimo: piscar o LED da placa Nucleo G474RE.
//!
//! Este binário é construído com `no_std` e `cortex-m-rt` e demonstra um
//! loop simples que alterna um pino GPIO para acender/apagar um LED.
//!
//! Comentários linha-a-linha foram adicionados para servir como documentação
//! e facilitar entendimento durante leitura do código e geração de docs.

// Não permitir warnings e código unsafe no exemplo — facilita ensino e testes.
#![deny(warnings)]
#![deny(unsafe_code)]
// `no_main` : usamos o entry point fornecido pelo `cortex-m-rt`.
#![no_main]
// `no_std` : ambiente embarcado sem biblioteca padrão.
#![no_std]

// Importa traits de conveniência para configurar pinos e clocks.
use hal::prelude::*;
// Acesso às estruturas de periféricos dos pacotes HAL.
use hal::stm32;
// Alias do crate HAL para uso consistente no código.
use stm32g4xx_hal as hal;

// Macro `#[entry]` para marcar o ponto de entrada do programa.
use cortex_m_rt::entry;

// Importa o módulo `utils` que contém o logger e macros auxiliares.
#[macro_use]
mod utils;

// Importa a macro `info!` reexportada pelo módulo `utils::logger`.
use utils::logger::info;

// Ponto de entrada da aplicação embarcada.
#[entry]
fn main() -> ! {
    // Inicializa o logger (se configurado via features).
    utils::logger::init();

    // Mensagem de início (via logger configurado).
    info!("start");

    // Obtém acesso aos periféricos do microcontrolador.
    // `take()` retorna `Some(Peripherals)` apenas uma vez; falha se já
    // tiverem sido tomados por outra parte do código.
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");

    // Constrói a configuração do Reset & Clock Control (RCC).
    let mut rcc = dp.RCC.constrain();

    // Log de inicialização do hardware usado neste exemplo.
    info!("Init Led");

    // Separa o controlador GPIOA em partes para configuração de pinos.
    let gpioa = dp.GPIOA.split(&mut rcc);

    // Configura PA5 como saída push-pull — pino do LED nas placas Nucleo.
    let mut led = gpioa.pa5.into_push_pull_output();

    // Loop principal infinito — alterna o estado do LED com delays simples.
    loop {
        // Registra que vamos colocar o LED em nível baixo (apagado/ligado
        // dependendo do circuito da placa).
        info!("Set Led low");

        // Pequeno delay ativo: laço vazio para criar tempo visível.
        for _ in 0..10_000_000 {
            led.set_low().expect("deu errado o led low");
        }

        // Agora registamos que vamos colocar o LED em nível alto.
        info!("Set Led High");

        // Outro delay ativo para manter o LED no estado alto por um tempo.
        for _ in 0..10_000_000 {
            led.set_high().expect("deu errado o led low");
        }
    }
}