#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "34 - Uart0"]
    UART0 = 34,
    #[doc = "35 - Uart1"]
    UART1 = 35,
    #[doc = "36 - Uart2"]
    UART2 = 36,
    #[doc = "37 - Uart3"]
    UART3 = 37,
}
#[doc = r" TryFromInterruptError"]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            34 => Ok(Interrupt::UART0),
            35 => Ok(Interrupt::UART1),
            36 => Ok(Interrupt::UART2),
            37 => Ok(Interrupt::UART3),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
