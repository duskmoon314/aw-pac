#[doc = "Register `pe_cfg1` reader"]
pub type R = crate::R<PE_CFG1_SPEC>;
#[doc = "Register `pe_cfg1` writer"]
pub type W = crate::W<PE_CFG1_SPEC>;
#[doc = "Field `pe8_select` reader - PE8 Select"]
pub type PE8_SELECT_R = crate::FieldReader<PE8_SELECT_A>;
#[doc = "PE8 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PE8_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    NCSI0_D4 = 2,
    #[doc = "4: `100`"]
    PWM2 = 4,
    #[doc = "6: `110`"]
    JTAG_MS = 6,
    #[doc = "8: `1000`"]
    MDC = 8,
    #[doc = "14: `1110`"]
    PE_EINT8 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    UART1_RTS = 3,
    #[doc = "5: `101`"]
    UART3_TX = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PE8_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PE8_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PE8_SELECT_A {
    type Ux = u8;
}
impl PE8_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PE8_SELECT_A> {
        match self.bits {
            0 => Some(PE8_SELECT_A::INPUT),
            2 => Some(PE8_SELECT_A::NCSI0_D4),
            4 => Some(PE8_SELECT_A::PWM2),
            6 => Some(PE8_SELECT_A::JTAG_MS),
            8 => Some(PE8_SELECT_A::MDC),
            14 => Some(PE8_SELECT_A::PE_EINT8),
            1 => Some(PE8_SELECT_A::OUTPUT),
            3 => Some(PE8_SELECT_A::UART1_RTS),
            5 => Some(PE8_SELECT_A::UART3_TX),
            15 => Some(PE8_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PE8_SELECT_A::INPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_ncsi0_d4(&self) -> bool {
        *self == PE8_SELECT_A::NCSI0_D4
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_pwm2(&self) -> bool {
        *self == PE8_SELECT_A::PWM2
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_jtag_ms(&self) -> bool {
        *self == PE8_SELECT_A::JTAG_MS
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_mdc(&self) -> bool {
        *self == PE8_SELECT_A::MDC
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pe_eint8(&self) -> bool {
        *self == PE8_SELECT_A::PE_EINT8
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PE8_SELECT_A::OUTPUT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_uart1_rts(&self) -> bool {
        *self == PE8_SELECT_A::UART1_RTS
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_uart3_tx(&self) -> bool {
        *self == PE8_SELECT_A::UART3_TX
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PE8_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pe8_select` writer - PE8 Select"]
pub type PE8_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PE8_SELECT_A>;
impl<'a, REG> PE8_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PE8_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ncsi0_d4(self) -> &'a mut crate::W<REG> {
        self.variant(PE8_SELECT_A::NCSI0_D4)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn pwm2(self) -> &'a mut crate::W<REG> {
        self.variant(PE8_SELECT_A::PWM2)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn jtag_ms(self) -> &'a mut crate::W<REG> {
        self.variant(PE8_SELECT_A::JTAG_MS)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn mdc(self) -> &'a mut crate::W<REG> {
        self.variant(PE8_SELECT_A::MDC)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pe_eint8(self) -> &'a mut crate::W<REG> {
        self.variant(PE8_SELECT_A::PE_EINT8)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PE8_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn uart1_rts(self) -> &'a mut crate::W<REG> {
        self.variant(PE8_SELECT_A::UART1_RTS)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn uart3_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PE8_SELECT_A::UART3_TX)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PE8_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pe9_select` reader - PE9 Select"]
pub type PE9_SELECT_R = crate::FieldReader<PE9_SELECT_A>;
#[doc = "PE9 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PE9_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    NCSI0_D5 = 2,
    #[doc = "4: `100`"]
    PWM3 = 4,
    #[doc = "6: `110`"]
    JTAG_DI = 6,
    #[doc = "8: `1000`"]
    MDIO = 8,
    #[doc = "14: `1110`"]
    PE_EINT9 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    UART1_CTS = 3,
    #[doc = "5: `101`"]
    UART3_RX = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PE9_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PE9_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PE9_SELECT_A {
    type Ux = u8;
}
impl PE9_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PE9_SELECT_A> {
        match self.bits {
            0 => Some(PE9_SELECT_A::INPUT),
            2 => Some(PE9_SELECT_A::NCSI0_D5),
            4 => Some(PE9_SELECT_A::PWM3),
            6 => Some(PE9_SELECT_A::JTAG_DI),
            8 => Some(PE9_SELECT_A::MDIO),
            14 => Some(PE9_SELECT_A::PE_EINT9),
            1 => Some(PE9_SELECT_A::OUTPUT),
            3 => Some(PE9_SELECT_A::UART1_CTS),
            5 => Some(PE9_SELECT_A::UART3_RX),
            15 => Some(PE9_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PE9_SELECT_A::INPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_ncsi0_d5(&self) -> bool {
        *self == PE9_SELECT_A::NCSI0_D5
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_pwm3(&self) -> bool {
        *self == PE9_SELECT_A::PWM3
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_jtag_di(&self) -> bool {
        *self == PE9_SELECT_A::JTAG_DI
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_mdio(&self) -> bool {
        *self == PE9_SELECT_A::MDIO
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pe_eint9(&self) -> bool {
        *self == PE9_SELECT_A::PE_EINT9
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PE9_SELECT_A::OUTPUT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_uart1_cts(&self) -> bool {
        *self == PE9_SELECT_A::UART1_CTS
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_uart3_rx(&self) -> bool {
        *self == PE9_SELECT_A::UART3_RX
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PE9_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pe9_select` writer - PE9 Select"]
pub type PE9_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PE9_SELECT_A>;
impl<'a, REG> PE9_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PE9_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ncsi0_d5(self) -> &'a mut crate::W<REG> {
        self.variant(PE9_SELECT_A::NCSI0_D5)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn pwm3(self) -> &'a mut crate::W<REG> {
        self.variant(PE9_SELECT_A::PWM3)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn jtag_di(self) -> &'a mut crate::W<REG> {
        self.variant(PE9_SELECT_A::JTAG_DI)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn mdio(self) -> &'a mut crate::W<REG> {
        self.variant(PE9_SELECT_A::MDIO)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pe_eint9(self) -> &'a mut crate::W<REG> {
        self.variant(PE9_SELECT_A::PE_EINT9)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PE9_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn uart1_cts(self) -> &'a mut crate::W<REG> {
        self.variant(PE9_SELECT_A::UART1_CTS)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn uart3_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PE9_SELECT_A::UART3_RX)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PE9_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pe10_select` reader - PE10 Select"]
pub type PE10_SELECT_R = crate::FieldReader<PE10_SELECT_A>;
#[doc = "PE10 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PE10_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    NCSI0_D6 = 2,
    #[doc = "4: `100`"]
    PWM4 = 4,
    #[doc = "6: `110`"]
    JTAG_DO = 6,
    #[doc = "8: `1000`"]
    EPHY_25M = 8,
    #[doc = "14: `1110`"]
    PE_EINT10 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    UART1_TX = 3,
    #[doc = "5: `101`"]
    IR_RX = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PE10_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PE10_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PE10_SELECT_A {
    type Ux = u8;
}
impl PE10_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PE10_SELECT_A> {
        match self.bits {
            0 => Some(PE10_SELECT_A::INPUT),
            2 => Some(PE10_SELECT_A::NCSI0_D6),
            4 => Some(PE10_SELECT_A::PWM4),
            6 => Some(PE10_SELECT_A::JTAG_DO),
            8 => Some(PE10_SELECT_A::EPHY_25M),
            14 => Some(PE10_SELECT_A::PE_EINT10),
            1 => Some(PE10_SELECT_A::OUTPUT),
            3 => Some(PE10_SELECT_A::UART1_TX),
            5 => Some(PE10_SELECT_A::IR_RX),
            15 => Some(PE10_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PE10_SELECT_A::INPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_ncsi0_d6(&self) -> bool {
        *self == PE10_SELECT_A::NCSI0_D6
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_pwm4(&self) -> bool {
        *self == PE10_SELECT_A::PWM4
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_jtag_do(&self) -> bool {
        *self == PE10_SELECT_A::JTAG_DO
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_ephy_25m(&self) -> bool {
        *self == PE10_SELECT_A::EPHY_25M
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pe_eint10(&self) -> bool {
        *self == PE10_SELECT_A::PE_EINT10
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PE10_SELECT_A::OUTPUT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_uart1_tx(&self) -> bool {
        *self == PE10_SELECT_A::UART1_TX
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_ir_rx(&self) -> bool {
        *self == PE10_SELECT_A::IR_RX
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PE10_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pe10_select` writer - PE10 Select"]
pub type PE10_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PE10_SELECT_A>;
impl<'a, REG> PE10_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PE10_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ncsi0_d6(self) -> &'a mut crate::W<REG> {
        self.variant(PE10_SELECT_A::NCSI0_D6)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn pwm4(self) -> &'a mut crate::W<REG> {
        self.variant(PE10_SELECT_A::PWM4)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn jtag_do(self) -> &'a mut crate::W<REG> {
        self.variant(PE10_SELECT_A::JTAG_DO)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn ephy_25m(self) -> &'a mut crate::W<REG> {
        self.variant(PE10_SELECT_A::EPHY_25M)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pe_eint10(self) -> &'a mut crate::W<REG> {
        self.variant(PE10_SELECT_A::PE_EINT10)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PE10_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn uart1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PE10_SELECT_A::UART1_TX)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn ir_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PE10_SELECT_A::IR_RX)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PE10_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pe11_select` reader - PE11 Select"]
pub type PE11_SELECT_R = crate::FieldReader<PE11_SELECT_A>;
#[doc = "PE11 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PE11_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    NCSI0_D7 = 2,
    #[doc = "4: `100`"]
    I2S0_DOUT3 = 4,
    #[doc = "6: `110`"]
    JTAG_CK = 6,
    #[doc = "8: `1000`"]
    RGMII_TXD2 = 8,
    #[doc = "14: `1110`"]
    PE_EINT11 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    UART1_RX = 3,
    #[doc = "5: `101`"]
    I2S0_DIN3 = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PE11_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PE11_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PE11_SELECT_A {
    type Ux = u8;
}
impl PE11_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PE11_SELECT_A> {
        match self.bits {
            0 => Some(PE11_SELECT_A::INPUT),
            2 => Some(PE11_SELECT_A::NCSI0_D7),
            4 => Some(PE11_SELECT_A::I2S0_DOUT3),
            6 => Some(PE11_SELECT_A::JTAG_CK),
            8 => Some(PE11_SELECT_A::RGMII_TXD2),
            14 => Some(PE11_SELECT_A::PE_EINT11),
            1 => Some(PE11_SELECT_A::OUTPUT),
            3 => Some(PE11_SELECT_A::UART1_RX),
            5 => Some(PE11_SELECT_A::I2S0_DIN3),
            15 => Some(PE11_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PE11_SELECT_A::INPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_ncsi0_d7(&self) -> bool {
        *self == PE11_SELECT_A::NCSI0_D7
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_i2s0_dout3(&self) -> bool {
        *self == PE11_SELECT_A::I2S0_DOUT3
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_jtag_ck(&self) -> bool {
        *self == PE11_SELECT_A::JTAG_CK
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_rgmii_txd2(&self) -> bool {
        *self == PE11_SELECT_A::RGMII_TXD2
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pe_eint11(&self) -> bool {
        *self == PE11_SELECT_A::PE_EINT11
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PE11_SELECT_A::OUTPUT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_uart1_rx(&self) -> bool {
        *self == PE11_SELECT_A::UART1_RX
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_i2s0_din3(&self) -> bool {
        *self == PE11_SELECT_A::I2S0_DIN3
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PE11_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pe11_select` writer - PE11 Select"]
pub type PE11_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PE11_SELECT_A>;
impl<'a, REG> PE11_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PE11_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ncsi0_d7(self) -> &'a mut crate::W<REG> {
        self.variant(PE11_SELECT_A::NCSI0_D7)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn i2s0_dout3(self) -> &'a mut crate::W<REG> {
        self.variant(PE11_SELECT_A::I2S0_DOUT3)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn jtag_ck(self) -> &'a mut crate::W<REG> {
        self.variant(PE11_SELECT_A::JTAG_CK)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn rgmii_txd2(self) -> &'a mut crate::W<REG> {
        self.variant(PE11_SELECT_A::RGMII_TXD2)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pe_eint11(self) -> &'a mut crate::W<REG> {
        self.variant(PE11_SELECT_A::PE_EINT11)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PE11_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn uart1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PE11_SELECT_A::UART1_RX)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn i2s0_din3(self) -> &'a mut crate::W<REG> {
        self.variant(PE11_SELECT_A::I2S0_DIN3)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PE11_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pe12_select` reader - PE12 Select"]
pub type PE12_SELECT_R = crate::FieldReader<PE12_SELECT_A>;
#[doc = "PE12 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PE12_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    TWI2_SCK = 2,
    #[doc = "4: `100`"]
    I2S0_DOUT2 = 4,
    #[doc = "8: `1000`"]
    RGMII_TXD3 = 8,
    #[doc = "14: `1110`"]
    PE_EINT12 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    NCSI0_FIELD = 3,
    #[doc = "5: `101`"]
    I2S0_DIN2 = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PE12_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PE12_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PE12_SELECT_A {
    type Ux = u8;
}
impl PE12_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PE12_SELECT_A> {
        match self.bits {
            0 => Some(PE12_SELECT_A::INPUT),
            2 => Some(PE12_SELECT_A::TWI2_SCK),
            4 => Some(PE12_SELECT_A::I2S0_DOUT2),
            8 => Some(PE12_SELECT_A::RGMII_TXD3),
            14 => Some(PE12_SELECT_A::PE_EINT12),
            1 => Some(PE12_SELECT_A::OUTPUT),
            3 => Some(PE12_SELECT_A::NCSI0_FIELD),
            5 => Some(PE12_SELECT_A::I2S0_DIN2),
            15 => Some(PE12_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PE12_SELECT_A::INPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_twi2_sck(&self) -> bool {
        *self == PE12_SELECT_A::TWI2_SCK
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_i2s0_dout2(&self) -> bool {
        *self == PE12_SELECT_A::I2S0_DOUT2
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_rgmii_txd3(&self) -> bool {
        *self == PE12_SELECT_A::RGMII_TXD3
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pe_eint12(&self) -> bool {
        *self == PE12_SELECT_A::PE_EINT12
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PE12_SELECT_A::OUTPUT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_ncsi0_field(&self) -> bool {
        *self == PE12_SELECT_A::NCSI0_FIELD
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_i2s0_din2(&self) -> bool {
        *self == PE12_SELECT_A::I2S0_DIN2
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PE12_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pe12_select` writer - PE12 Select"]
pub type PE12_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PE12_SELECT_A>;
impl<'a, REG> PE12_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PE12_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn twi2_sck(self) -> &'a mut crate::W<REG> {
        self.variant(PE12_SELECT_A::TWI2_SCK)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn i2s0_dout2(self) -> &'a mut crate::W<REG> {
        self.variant(PE12_SELECT_A::I2S0_DOUT2)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn rgmii_txd3(self) -> &'a mut crate::W<REG> {
        self.variant(PE12_SELECT_A::RGMII_TXD3)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pe_eint12(self) -> &'a mut crate::W<REG> {
        self.variant(PE12_SELECT_A::PE_EINT12)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PE12_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn ncsi0_field(self) -> &'a mut crate::W<REG> {
        self.variant(PE12_SELECT_A::NCSI0_FIELD)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn i2s0_din2(self) -> &'a mut crate::W<REG> {
        self.variant(PE12_SELECT_A::I2S0_DIN2)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PE12_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pe13_select` reader - PE13 Select"]
pub type PE13_SELECT_R = crate::FieldReader<PE13_SELECT_A>;
#[doc = "PE13 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PE13_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    TWI2_SDA = 2,
    #[doc = "4: `100`"]
    I2S0_DOUT0 = 4,
    #[doc = "6: `110`"]
    DMIC_DATA3 = 6,
    #[doc = "8: `1000`"]
    RGMII_RXD2 = 8,
    #[doc = "14: `1110`"]
    PE_EINT13 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    PWM5 = 3,
    #[doc = "5: `101`"]
    I2S0_DIN1 = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PE13_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PE13_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PE13_SELECT_A {
    type Ux = u8;
}
impl PE13_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PE13_SELECT_A> {
        match self.bits {
            0 => Some(PE13_SELECT_A::INPUT),
            2 => Some(PE13_SELECT_A::TWI2_SDA),
            4 => Some(PE13_SELECT_A::I2S0_DOUT0),
            6 => Some(PE13_SELECT_A::DMIC_DATA3),
            8 => Some(PE13_SELECT_A::RGMII_RXD2),
            14 => Some(PE13_SELECT_A::PE_EINT13),
            1 => Some(PE13_SELECT_A::OUTPUT),
            3 => Some(PE13_SELECT_A::PWM5),
            5 => Some(PE13_SELECT_A::I2S0_DIN1),
            15 => Some(PE13_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PE13_SELECT_A::INPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_twi2_sda(&self) -> bool {
        *self == PE13_SELECT_A::TWI2_SDA
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_i2s0_dout0(&self) -> bool {
        *self == PE13_SELECT_A::I2S0_DOUT0
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_dmic_data3(&self) -> bool {
        *self == PE13_SELECT_A::DMIC_DATA3
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_rgmii_rxd2(&self) -> bool {
        *self == PE13_SELECT_A::RGMII_RXD2
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pe_eint13(&self) -> bool {
        *self == PE13_SELECT_A::PE_EINT13
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PE13_SELECT_A::OUTPUT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_pwm5(&self) -> bool {
        *self == PE13_SELECT_A::PWM5
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_i2s0_din1(&self) -> bool {
        *self == PE13_SELECT_A::I2S0_DIN1
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PE13_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pe13_select` writer - PE13 Select"]
pub type PE13_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PE13_SELECT_A>;
impl<'a, REG> PE13_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PE13_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn twi2_sda(self) -> &'a mut crate::W<REG> {
        self.variant(PE13_SELECT_A::TWI2_SDA)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn i2s0_dout0(self) -> &'a mut crate::W<REG> {
        self.variant(PE13_SELECT_A::I2S0_DOUT0)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn dmic_data3(self) -> &'a mut crate::W<REG> {
        self.variant(PE13_SELECT_A::DMIC_DATA3)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn rgmii_rxd2(self) -> &'a mut crate::W<REG> {
        self.variant(PE13_SELECT_A::RGMII_RXD2)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pe_eint13(self) -> &'a mut crate::W<REG> {
        self.variant(PE13_SELECT_A::PE_EINT13)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PE13_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pwm5(self) -> &'a mut crate::W<REG> {
        self.variant(PE13_SELECT_A::PWM5)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn i2s0_din1(self) -> &'a mut crate::W<REG> {
        self.variant(PE13_SELECT_A::I2S0_DIN1)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PE13_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pe14_select` reader - PE14 Select"]
pub type PE14_SELECT_R = crate::FieldReader<PE14_SELECT_A>;
#[doc = "PE14 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PE14_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    TWI1_SCK = 2,
    #[doc = "4: `100`"]
    I2S0_DOUT1 = 4,
    #[doc = "6: `110`"]
    DMIC_DATA2 = 6,
    #[doc = "8: `1000`"]
    RGMII_RXD3 = 8,
    #[doc = "14: `1110`"]
    PE_EINT14 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    D_JTAG_MS = 3,
    #[doc = "5: `101`"]
    I2S0_DIN0 = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PE14_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PE14_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PE14_SELECT_A {
    type Ux = u8;
}
impl PE14_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PE14_SELECT_A> {
        match self.bits {
            0 => Some(PE14_SELECT_A::INPUT),
            2 => Some(PE14_SELECT_A::TWI1_SCK),
            4 => Some(PE14_SELECT_A::I2S0_DOUT1),
            6 => Some(PE14_SELECT_A::DMIC_DATA2),
            8 => Some(PE14_SELECT_A::RGMII_RXD3),
            14 => Some(PE14_SELECT_A::PE_EINT14),
            1 => Some(PE14_SELECT_A::OUTPUT),
            3 => Some(PE14_SELECT_A::D_JTAG_MS),
            5 => Some(PE14_SELECT_A::I2S0_DIN0),
            15 => Some(PE14_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PE14_SELECT_A::INPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_twi1_sck(&self) -> bool {
        *self == PE14_SELECT_A::TWI1_SCK
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_i2s0_dout1(&self) -> bool {
        *self == PE14_SELECT_A::I2S0_DOUT1
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_dmic_data2(&self) -> bool {
        *self == PE14_SELECT_A::DMIC_DATA2
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_rgmii_rxd3(&self) -> bool {
        *self == PE14_SELECT_A::RGMII_RXD3
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pe_eint14(&self) -> bool {
        *self == PE14_SELECT_A::PE_EINT14
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PE14_SELECT_A::OUTPUT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_d_jtag_ms(&self) -> bool {
        *self == PE14_SELECT_A::D_JTAG_MS
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_i2s0_din0(&self) -> bool {
        *self == PE14_SELECT_A::I2S0_DIN0
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PE14_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pe14_select` writer - PE14 Select"]
pub type PE14_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PE14_SELECT_A>;
impl<'a, REG> PE14_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PE14_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn twi1_sck(self) -> &'a mut crate::W<REG> {
        self.variant(PE14_SELECT_A::TWI1_SCK)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn i2s0_dout1(self) -> &'a mut crate::W<REG> {
        self.variant(PE14_SELECT_A::I2S0_DOUT1)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn dmic_data2(self) -> &'a mut crate::W<REG> {
        self.variant(PE14_SELECT_A::DMIC_DATA2)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn rgmii_rxd3(self) -> &'a mut crate::W<REG> {
        self.variant(PE14_SELECT_A::RGMII_RXD3)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pe_eint14(self) -> &'a mut crate::W<REG> {
        self.variant(PE14_SELECT_A::PE_EINT14)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PE14_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn d_jtag_ms(self) -> &'a mut crate::W<REG> {
        self.variant(PE14_SELECT_A::D_JTAG_MS)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn i2s0_din0(self) -> &'a mut crate::W<REG> {
        self.variant(PE14_SELECT_A::I2S0_DIN0)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PE14_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pe15_select` reader - PE15 Select"]
pub type PE15_SELECT_R = crate::FieldReader<PE15_SELECT_A>;
#[doc = "PE15 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PE15_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    TWI1_SDA = 2,
    #[doc = "4: `100`"]
    PWM6 = 4,
    #[doc = "6: `110`"]
    DMIC_DATA1 = 6,
    #[doc = "8: `1000`"]
    RGMII_RXCK = 8,
    #[doc = "14: `1110`"]
    PE_EINT15 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    D_JTAG_DI = 3,
    #[doc = "5: `101`"]
    I2S0_LRCK = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PE15_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PE15_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PE15_SELECT_A {
    type Ux = u8;
}
impl PE15_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PE15_SELECT_A> {
        match self.bits {
            0 => Some(PE15_SELECT_A::INPUT),
            2 => Some(PE15_SELECT_A::TWI1_SDA),
            4 => Some(PE15_SELECT_A::PWM6),
            6 => Some(PE15_SELECT_A::DMIC_DATA1),
            8 => Some(PE15_SELECT_A::RGMII_RXCK),
            14 => Some(PE15_SELECT_A::PE_EINT15),
            1 => Some(PE15_SELECT_A::OUTPUT),
            3 => Some(PE15_SELECT_A::D_JTAG_DI),
            5 => Some(PE15_SELECT_A::I2S0_LRCK),
            15 => Some(PE15_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PE15_SELECT_A::INPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_twi1_sda(&self) -> bool {
        *self == PE15_SELECT_A::TWI1_SDA
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_pwm6(&self) -> bool {
        *self == PE15_SELECT_A::PWM6
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_dmic_data1(&self) -> bool {
        *self == PE15_SELECT_A::DMIC_DATA1
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_rgmii_rxck(&self) -> bool {
        *self == PE15_SELECT_A::RGMII_RXCK
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pe_eint15(&self) -> bool {
        *self == PE15_SELECT_A::PE_EINT15
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PE15_SELECT_A::OUTPUT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_d_jtag_di(&self) -> bool {
        *self == PE15_SELECT_A::D_JTAG_DI
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_i2s0_lrck(&self) -> bool {
        *self == PE15_SELECT_A::I2S0_LRCK
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PE15_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pe15_select` writer - PE15 Select"]
pub type PE15_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PE15_SELECT_A>;
impl<'a, REG> PE15_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PE15_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn twi1_sda(self) -> &'a mut crate::W<REG> {
        self.variant(PE15_SELECT_A::TWI1_SDA)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn pwm6(self) -> &'a mut crate::W<REG> {
        self.variant(PE15_SELECT_A::PWM6)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn dmic_data1(self) -> &'a mut crate::W<REG> {
        self.variant(PE15_SELECT_A::DMIC_DATA1)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn rgmii_rxck(self) -> &'a mut crate::W<REG> {
        self.variant(PE15_SELECT_A::RGMII_RXCK)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pe_eint15(self) -> &'a mut crate::W<REG> {
        self.variant(PE15_SELECT_A::PE_EINT15)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PE15_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn d_jtag_di(self) -> &'a mut crate::W<REG> {
        self.variant(PE15_SELECT_A::D_JTAG_DI)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn i2s0_lrck(self) -> &'a mut crate::W<REG> {
        self.variant(PE15_SELECT_A::I2S0_LRCK)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PE15_SELECT_A::IO_DISABLE)
    }
}
impl R {
    #[doc = "Bits 0:3 - PE8 Select"]
    #[inline(always)]
    pub fn pe8_select(&self) -> PE8_SELECT_R {
        PE8_SELECT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PE9 Select"]
    #[inline(always)]
    pub fn pe9_select(&self) -> PE9_SELECT_R {
        PE9_SELECT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PE10 Select"]
    #[inline(always)]
    pub fn pe10_select(&self) -> PE10_SELECT_R {
        PE10_SELECT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PE11 Select"]
    #[inline(always)]
    pub fn pe11_select(&self) -> PE11_SELECT_R {
        PE11_SELECT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PE12 Select"]
    #[inline(always)]
    pub fn pe12_select(&self) -> PE12_SELECT_R {
        PE12_SELECT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PE13 Select"]
    #[inline(always)]
    pub fn pe13_select(&self) -> PE13_SELECT_R {
        PE13_SELECT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PE14 Select"]
    #[inline(always)]
    pub fn pe14_select(&self) -> PE14_SELECT_R {
        PE14_SELECT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PE15 Select"]
    #[inline(always)]
    pub fn pe15_select(&self) -> PE15_SELECT_R {
        PE15_SELECT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PE8 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe8_select(&mut self) -> PE8_SELECT_W<PE_CFG1_SPEC> {
        PE8_SELECT_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - PE9 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe9_select(&mut self) -> PE9_SELECT_W<PE_CFG1_SPEC> {
        PE9_SELECT_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - PE10 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe10_select(&mut self) -> PE10_SELECT_W<PE_CFG1_SPEC> {
        PE10_SELECT_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - PE11 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe11_select(&mut self) -> PE11_SELECT_W<PE_CFG1_SPEC> {
        PE11_SELECT_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - PE12 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe12_select(&mut self) -> PE12_SELECT_W<PE_CFG1_SPEC> {
        PE12_SELECT_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - PE13 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe13_select(&mut self) -> PE13_SELECT_W<PE_CFG1_SPEC> {
        PE13_SELECT_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - PE14 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe14_select(&mut self) -> PE14_SELECT_W<PE_CFG1_SPEC> {
        PE14_SELECT_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - PE15 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe15_select(&mut self) -> PE15_SELECT_W<PE_CFG1_SPEC> {
        PE15_SELECT_W::new(self, 28)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PE Configure Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PE_CFG1_SPEC;
impl crate::RegisterSpec for PE_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pe_cfg1::R`](R) reader structure"]
impl crate::Readable for PE_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pe_cfg1::W`](W) writer structure"]
impl crate::Writable for PE_CFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pe_cfg1 to value 0"]
impl crate::Resettable for PE_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
