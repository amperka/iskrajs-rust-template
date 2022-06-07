#![no_main]
#![no_std]
#![allow(unused_imports)]

{% if rtt == "Yes" %}use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};{% else %}
use panic_halt as _;{% endif %}
{% if rtic == "Yes" %}use systick_monotonic::{fugit::Duration, Systick};{% else %}{% if board == "IskraJS" %}use cortex_m_rt::{entry, pre_init};{% else %}use cortex_m_rt::entry;{% endif %}{% endif %}
use stm32f4xx_hal::{
    gpio::{Pin, Output, PushPull},
    pac,
    prelude::*,
};

{% if board == "IskraJS" %}// Example of using CCMRAM
#[link_section = ".ccmram"]
#[inline(never)]
fn example_function() -> u32 {
    16 + 16
}

#[link_section = ".ccmram"]
static mut EXAMPLE_DATA: u32 = 0;{% endif %}

{% if rtic == "Yes" %}#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true, dispatchers = [SPI3])]
mod app {

    use super::*;
    
    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        {% if board == "IskraJS" %}led: Pin<'B', 6, Output<PushPull>>,{% else %}led: Pin<'B', 2, Output<PushPull>>,{% endif %}
    }

    #[monotonic(binds = SysTick, default = true)]
    type MonoTimer = Systick<1000>;

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        {% if rtt == "Yes" %}rtt_init_print!();{% endif %}
        {% if board == "IskraJS" %}// CCMRAM initialization
        unsafe {
            extern "C" {
                static mut _ccmram_start: u32;
                static mut _ccmram_end: u32;
                static mut _siccmram: u32;
            }
            
            r0::init_data(&mut _ccmram_start, &mut _ccmram_end, &_siccmram);
        }{% endif %}
        // Configuration clocks
        let rcc = ctx.device.RCC.constrain();
        {% if board == "IskraJS" %}let _clocks = rcc
            .cfgr
            .use_hse(8.MHz())
            .sysclk(168.MHz())
            .hclk(168.MHz())
            .pclk1(42.MHz())
            .pclk2(84.MHz())
            .require_pll48clk()
            .freeze();
        {% else %}let _clocks = rcc
            .cfgr
            .use_hse(8.MHz())
            .sysclk(96.MHz())
            .hclk(96.MHz())
            .pclk1(48.MHz())
            .pclk2(96.MHz())
            .require_pll48clk()
            .freeze();{% endif %}
        let mono = Systick::new(ctx.core.SYST, 96_000_000);

        let gpiob = ctx.device.GPIOB.split();
        {% if board == "IskraJS" %}let led = gpiob.pb6.into_push_pull_output();{% else %}let led = gpiob.pb2.into_push_pull_output();{% endif %}

        {% if rtt == "Yes" %}rprintln!("It works 🎉");{% endif %}// Schedule the blinking task
        blink::spawn_after(Duration::<u64, 1, 1000>::from_ticks(1000)).unwrap();

        (Shared {}, Local { led }, init::Monotonics(mono))
    }

    #[task(local = [led])]
    fn blink(ctx: blink::Context) {
        ctx.local.led.toggle();
        
        blink::spawn_after(Duration::<u64, 1, 1000>::from_ticks(1000)).unwrap();
    }
}{% else %}{% if board == "IskraJS" %}// CCMRAM initialization
#[pre_init]
unsafe fn ccmram_init() {
    extern "C" {
        static mut _ccmram_start: u32;
        static mut _ccmram_end: u32;
        static mut _siccmram: u32;
    }

    r0::init_data(&mut _ccmram_start, &mut _ccmram_end, &_siccmram);
}{% endif %}

#[entry]
fn main() -> ! {
    {% if rtt == "Yes" %}rtt_init_print!();{% endif %}if let (Some(device_periph), Some(core_periph)) =
        (pac::Peripherals::take(), cortex_m::peripheral::Peripherals::take())
    {
        // Configuration clocks
        let rcc = device_periph.RCC.constrain();
        {% if board == "IskraJS" %}let clocks = rcc
            .cfgr
            .use_hse(8.MHz())
            .sysclk(168.MHz())
            .hclk(168.MHz())
            .pclk1(42.MHz())
            .pclk2(84.MHz())
            .require_pll48clk()
            .freeze();
        {% else %}let clocks = rcc
            .cfgr
            .use_hse(8.MHz())
            .sysclk(96.MHz())
            .hclk(96.MHz())
            .pclk1(48.MHz())
            .pclk2(96.MHz())
            .require_pll48clk()
            .freeze();{% endif %}
        let gpiob = device_periph.GPIOB.split();
        {% if board == "IskraJS" %}let mut led = gpiob.pb6.into_push_pull_output();{% else %}let mut led = gpiob.pb2.into_push_pull_output();{% endif %}
        let mut delay = core_periph.SYST.delay(&clocks);

        {% if rtt == "Yes" %}rprintln!("It works 🎉");{% endif %}
        loop {
            led.set_high();
            delay.delay_ms(1000_u32);
            led.set_low();
            delay.delay_ms(1000_u32);
        }
    }

    loop {}
}
{% endif %}
