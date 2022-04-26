#[doc = "Register `pg_cfg1` reader"]
pub struct R(crate::R<PG_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PG_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PG_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PG_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pg_cfg1` writer"]
pub struct W(crate::W<PG_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PG_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PG_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PG_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PG15 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PG15_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    I2S1_DOUT0 = 2,
    #[doc = "4: `100`"]
    MDIO = 4,
    #[doc = "6: `110`"]
    SPI0_HOLD = 6,
    #[doc = "14: `1110`"]
    PG_EINT15 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    TWI2_SDA = 3,
    #[doc = "5: `101`"]
    I2S1_DIN1 = 5,
    #[doc = "7: `111`"]
    UART1_CTS = 7,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PG15_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PG15_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PG15_SELECT` reader - PG15 Select"]
pub struct PG15_SELECT_R(crate::FieldReader<u8>);
impl PG15_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PG15_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PG15_SELECT_A> {
        match self.bits {
            0 => Some(PG15_SELECT_A::INPUT),
            2 => Some(PG15_SELECT_A::I2S1_DOUT0),
            4 => Some(PG15_SELECT_A::MDIO),
            6 => Some(PG15_SELECT_A::SPI0_HOLD),
            14 => Some(PG15_SELECT_A::PG_EINT15),
            1 => Some(PG15_SELECT_A::OUTPUT),
            3 => Some(PG15_SELECT_A::TWI2_SDA),
            5 => Some(PG15_SELECT_A::I2S1_DIN1),
            7 => Some(PG15_SELECT_A::UART1_CTS),
            15 => Some(PG15_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == PG15_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `I2S1_DOUT0`"]
    #[inline(always)]
    pub fn is_i2s1_dout0(&self) -> bool {
        **self == PG15_SELECT_A::I2S1_DOUT0
    }
    #[doc = "Checks if the value of the field is `MDIO`"]
    #[inline(always)]
    pub fn is_mdio(&self) -> bool {
        **self == PG15_SELECT_A::MDIO
    }
    #[doc = "Checks if the value of the field is `SPI0_HOLD`"]
    #[inline(always)]
    pub fn is_spi0_hold(&self) -> bool {
        **self == PG15_SELECT_A::SPI0_HOLD
    }
    #[doc = "Checks if the value of the field is `PG_EINT15`"]
    #[inline(always)]
    pub fn is_pg_eint15(&self) -> bool {
        **self == PG15_SELECT_A::PG_EINT15
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == PG15_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `TWI2_SDA`"]
    #[inline(always)]
    pub fn is_twi2_sda(&self) -> bool {
        **self == PG15_SELECT_A::TWI2_SDA
    }
    #[doc = "Checks if the value of the field is `I2S1_DIN1`"]
    #[inline(always)]
    pub fn is_i2s1_din1(&self) -> bool {
        **self == PG15_SELECT_A::I2S1_DIN1
    }
    #[doc = "Checks if the value of the field is `UART1_CTS`"]
    #[inline(always)]
    pub fn is_uart1_cts(&self) -> bool {
        **self == PG15_SELECT_A::UART1_CTS
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        **self == PG15_SELECT_A::IO_DISABLE
    }
}
impl core::ops::Deref for PG15_SELECT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG15_SELECT` writer - PG15 Select"]
pub struct PG15_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PG15_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PG15_SELECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PG15_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn i2s1_dout0(self) -> &'a mut W {
        self.variant(PG15_SELECT_A::I2S1_DOUT0)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn mdio(self) -> &'a mut W {
        self.variant(PG15_SELECT_A::MDIO)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn spi0_hold(self) -> &'a mut W {
        self.variant(PG15_SELECT_A::SPI0_HOLD)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pg_eint15(self) -> &'a mut W {
        self.variant(PG15_SELECT_A::PG_EINT15)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PG15_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn twi2_sda(self) -> &'a mut W {
        self.variant(PG15_SELECT_A::TWI2_SDA)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn i2s1_din1(self) -> &'a mut W {
        self.variant(PG15_SELECT_A::I2S1_DIN1)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart1_cts(self) -> &'a mut W {
        self.variant(PG15_SELECT_A::UART1_CTS)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PG15_SELECT_A::IO_DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "PG14 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PG14_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    I2S1_DIN0 = 2,
    #[doc = "4: `100`"]
    MDC = 4,
    #[doc = "6: `110`"]
    SPI0_WP = 6,
    #[doc = "14: `1110`"]
    PG_EINT14 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    TWI2_SCK = 3,
    #[doc = "5: `101`"]
    I2S1_DOUT1 = 5,
    #[doc = "7: `111`"]
    UART1_RTS = 7,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PG14_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PG14_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PG14_SELECT` reader - PG14 Select"]
pub struct PG14_SELECT_R(crate::FieldReader<u8>);
impl PG14_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PG14_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PG14_SELECT_A> {
        match self.bits {
            0 => Some(PG14_SELECT_A::INPUT),
            2 => Some(PG14_SELECT_A::I2S1_DIN0),
            4 => Some(PG14_SELECT_A::MDC),
            6 => Some(PG14_SELECT_A::SPI0_WP),
            14 => Some(PG14_SELECT_A::PG_EINT14),
            1 => Some(PG14_SELECT_A::OUTPUT),
            3 => Some(PG14_SELECT_A::TWI2_SCK),
            5 => Some(PG14_SELECT_A::I2S1_DOUT1),
            7 => Some(PG14_SELECT_A::UART1_RTS),
            15 => Some(PG14_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == PG14_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `I2S1_DIN0`"]
    #[inline(always)]
    pub fn is_i2s1_din0(&self) -> bool {
        **self == PG14_SELECT_A::I2S1_DIN0
    }
    #[doc = "Checks if the value of the field is `MDC`"]
    #[inline(always)]
    pub fn is_mdc(&self) -> bool {
        **self == PG14_SELECT_A::MDC
    }
    #[doc = "Checks if the value of the field is `SPI0_WP`"]
    #[inline(always)]
    pub fn is_spi0_wp(&self) -> bool {
        **self == PG14_SELECT_A::SPI0_WP
    }
    #[doc = "Checks if the value of the field is `PG_EINT14`"]
    #[inline(always)]
    pub fn is_pg_eint14(&self) -> bool {
        **self == PG14_SELECT_A::PG_EINT14
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == PG14_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `TWI2_SCK`"]
    #[inline(always)]
    pub fn is_twi2_sck(&self) -> bool {
        **self == PG14_SELECT_A::TWI2_SCK
    }
    #[doc = "Checks if the value of the field is `I2S1_DOUT1`"]
    #[inline(always)]
    pub fn is_i2s1_dout1(&self) -> bool {
        **self == PG14_SELECT_A::I2S1_DOUT1
    }
    #[doc = "Checks if the value of the field is `UART1_RTS`"]
    #[inline(always)]
    pub fn is_uart1_rts(&self) -> bool {
        **self == PG14_SELECT_A::UART1_RTS
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        **self == PG14_SELECT_A::IO_DISABLE
    }
}
impl core::ops::Deref for PG14_SELECT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG14_SELECT` writer - PG14 Select"]
pub struct PG14_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PG14_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PG14_SELECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PG14_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn i2s1_din0(self) -> &'a mut W {
        self.variant(PG14_SELECT_A::I2S1_DIN0)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn mdc(self) -> &'a mut W {
        self.variant(PG14_SELECT_A::MDC)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn spi0_wp(self) -> &'a mut W {
        self.variant(PG14_SELECT_A::SPI0_WP)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pg_eint14(self) -> &'a mut W {
        self.variant(PG14_SELECT_A::PG_EINT14)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PG14_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn twi2_sck(self) -> &'a mut W {
        self.variant(PG14_SELECT_A::TWI2_SCK)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn i2s1_dout1(self) -> &'a mut W {
        self.variant(PG14_SELECT_A::I2S1_DOUT1)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart1_rts(self) -> &'a mut W {
        self.variant(PG14_SELECT_A::UART1_RTS)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PG14_SELECT_A::IO_DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "PG13 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PG13_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    I2S1_BCLK = 2,
    #[doc = "4: `100`"]
    RGMII_CLKIN_RMII_RXER = 4,
    #[doc = "6: `110`"]
    LEDC_DO = 6,
    #[doc = "14: `1110`"]
    PG_EINT13 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    TWI0_SDA = 3,
    #[doc = "5: `101`"]
    PWM2 = 5,
    #[doc = "7: `111`"]
    UART1_RX = 7,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PG13_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PG13_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PG13_SELECT` reader - PG13 Select"]
pub struct PG13_SELECT_R(crate::FieldReader<u8>);
impl PG13_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PG13_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PG13_SELECT_A> {
        match self.bits {
            0 => Some(PG13_SELECT_A::INPUT),
            2 => Some(PG13_SELECT_A::I2S1_BCLK),
            4 => Some(PG13_SELECT_A::RGMII_CLKIN_RMII_RXER),
            6 => Some(PG13_SELECT_A::LEDC_DO),
            14 => Some(PG13_SELECT_A::PG_EINT13),
            1 => Some(PG13_SELECT_A::OUTPUT),
            3 => Some(PG13_SELECT_A::TWI0_SDA),
            5 => Some(PG13_SELECT_A::PWM2),
            7 => Some(PG13_SELECT_A::UART1_RX),
            15 => Some(PG13_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == PG13_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `I2S1_BCLK`"]
    #[inline(always)]
    pub fn is_i2s1_bclk(&self) -> bool {
        **self == PG13_SELECT_A::I2S1_BCLK
    }
    #[doc = "Checks if the value of the field is `RGMII_CLKIN_RMII_RXER`"]
    #[inline(always)]
    pub fn is_rgmii_clkin_rmii_rxer(&self) -> bool {
        **self == PG13_SELECT_A::RGMII_CLKIN_RMII_RXER
    }
    #[doc = "Checks if the value of the field is `LEDC_DO`"]
    #[inline(always)]
    pub fn is_ledc_do(&self) -> bool {
        **self == PG13_SELECT_A::LEDC_DO
    }
    #[doc = "Checks if the value of the field is `PG_EINT13`"]
    #[inline(always)]
    pub fn is_pg_eint13(&self) -> bool {
        **self == PG13_SELECT_A::PG_EINT13
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == PG13_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `TWI0_SDA`"]
    #[inline(always)]
    pub fn is_twi0_sda(&self) -> bool {
        **self == PG13_SELECT_A::TWI0_SDA
    }
    #[doc = "Checks if the value of the field is `PWM2`"]
    #[inline(always)]
    pub fn is_pwm2(&self) -> bool {
        **self == PG13_SELECT_A::PWM2
    }
    #[doc = "Checks if the value of the field is `UART1_RX`"]
    #[inline(always)]
    pub fn is_uart1_rx(&self) -> bool {
        **self == PG13_SELECT_A::UART1_RX
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        **self == PG13_SELECT_A::IO_DISABLE
    }
}
impl core::ops::Deref for PG13_SELECT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG13_SELECT` writer - PG13 Select"]
pub struct PG13_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PG13_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PG13_SELECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PG13_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn i2s1_bclk(self) -> &'a mut W {
        self.variant(PG13_SELECT_A::I2S1_BCLK)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn rgmii_clkin_rmii_rxer(self) -> &'a mut W {
        self.variant(PG13_SELECT_A::RGMII_CLKIN_RMII_RXER)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn ledc_do(self) -> &'a mut W {
        self.variant(PG13_SELECT_A::LEDC_DO)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pg_eint13(self) -> &'a mut W {
        self.variant(PG13_SELECT_A::PG_EINT13)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PG13_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn twi0_sda(self) -> &'a mut W {
        self.variant(PG13_SELECT_A::TWI0_SDA)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pwm2(self) -> &'a mut W {
        self.variant(PG13_SELECT_A::PWM2)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart1_rx(self) -> &'a mut W {
        self.variant(PG13_SELECT_A::UART1_RX)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PG13_SELECT_A::IO_DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "PG12 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PG12_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "2: `10`"]
    I2S1_LRCK = 2,
    #[doc = "3: `11`"]
    TWI0_SCK = 3,
    #[doc = "4: `100`"]
    RGMII_TXCTRL_RMII_TXEN = 4,
    #[doc = "5: `101`"]
    CLK_FANOUT2 = 5,
    #[doc = "6: `110`"]
    PWM0 = 6,
    #[doc = "14: `1110`"]
    PG_EINT12 = 14,
    #[doc = "7: `111`"]
    UART1_TX = 7,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PG12_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PG12_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PG12_SELECT` reader - PG12 Select"]
pub struct PG12_SELECT_R(crate::FieldReader<u8>);
impl PG12_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PG12_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PG12_SELECT_A> {
        match self.bits {
            0 => Some(PG12_SELECT_A::INPUT),
            1 => Some(PG12_SELECT_A::OUTPUT),
            2 => Some(PG12_SELECT_A::I2S1_LRCK),
            3 => Some(PG12_SELECT_A::TWI0_SCK),
            4 => Some(PG12_SELECT_A::RGMII_TXCTRL_RMII_TXEN),
            5 => Some(PG12_SELECT_A::CLK_FANOUT2),
            6 => Some(PG12_SELECT_A::PWM0),
            14 => Some(PG12_SELECT_A::PG_EINT12),
            7 => Some(PG12_SELECT_A::UART1_TX),
            15 => Some(PG12_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == PG12_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == PG12_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `I2S1_LRCK`"]
    #[inline(always)]
    pub fn is_i2s1_lrck(&self) -> bool {
        **self == PG12_SELECT_A::I2S1_LRCK
    }
    #[doc = "Checks if the value of the field is `TWI0_SCK`"]
    #[inline(always)]
    pub fn is_twi0_sck(&self) -> bool {
        **self == PG12_SELECT_A::TWI0_SCK
    }
    #[doc = "Checks if the value of the field is `RGMII_TXCTRL_RMII_TXEN`"]
    #[inline(always)]
    pub fn is_rgmii_txctrl_rmii_txen(&self) -> bool {
        **self == PG12_SELECT_A::RGMII_TXCTRL_RMII_TXEN
    }
    #[doc = "Checks if the value of the field is `CLK_FANOUT2`"]
    #[inline(always)]
    pub fn is_clk_fanout2(&self) -> bool {
        **self == PG12_SELECT_A::CLK_FANOUT2
    }
    #[doc = "Checks if the value of the field is `PWM0`"]
    #[inline(always)]
    pub fn is_pwm0(&self) -> bool {
        **self == PG12_SELECT_A::PWM0
    }
    #[doc = "Checks if the value of the field is `PG_EINT12`"]
    #[inline(always)]
    pub fn is_pg_eint12(&self) -> bool {
        **self == PG12_SELECT_A::PG_EINT12
    }
    #[doc = "Checks if the value of the field is `UART1_TX`"]
    #[inline(always)]
    pub fn is_uart1_tx(&self) -> bool {
        **self == PG12_SELECT_A::UART1_TX
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        **self == PG12_SELECT_A::IO_DISABLE
    }
}
impl core::ops::Deref for PG12_SELECT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG12_SELECT` writer - PG12 Select"]
pub struct PG12_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PG12_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PG12_SELECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PG12_SELECT_A::INPUT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PG12_SELECT_A::OUTPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn i2s1_lrck(self) -> &'a mut W {
        self.variant(PG12_SELECT_A::I2S1_LRCK)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn twi0_sck(self) -> &'a mut W {
        self.variant(PG12_SELECT_A::TWI0_SCK)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn rgmii_txctrl_rmii_txen(self) -> &'a mut W {
        self.variant(PG12_SELECT_A::RGMII_TXCTRL_RMII_TXEN)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn clk_fanout2(self) -> &'a mut W {
        self.variant(PG12_SELECT_A::CLK_FANOUT2)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn pwm0(self) -> &'a mut W {
        self.variant(PG12_SELECT_A::PWM0)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pg_eint12(self) -> &'a mut W {
        self.variant(PG12_SELECT_A::PG_EINT12)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart1_tx(self) -> &'a mut W {
        self.variant(PG12_SELECT_A::UART1_TX)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PG12_SELECT_A::IO_DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "PG11 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PG11_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    I2S1_MCLK = 2,
    #[doc = "4: `100`"]
    EPHY_25M = 4,
    #[doc = "6: `110`"]
    TCON_TRIG = 6,
    #[doc = "14: `1110`"]
    PG_EINT11 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    TWI3_SDA = 3,
    #[doc = "5: `101`"]
    CLK_FANOUT1 = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PG11_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PG11_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PG11_SELECT` reader - PG11 Select"]
pub struct PG11_SELECT_R(crate::FieldReader<u8>);
impl PG11_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PG11_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PG11_SELECT_A> {
        match self.bits {
            0 => Some(PG11_SELECT_A::INPUT),
            2 => Some(PG11_SELECT_A::I2S1_MCLK),
            4 => Some(PG11_SELECT_A::EPHY_25M),
            6 => Some(PG11_SELECT_A::TCON_TRIG),
            14 => Some(PG11_SELECT_A::PG_EINT11),
            1 => Some(PG11_SELECT_A::OUTPUT),
            3 => Some(PG11_SELECT_A::TWI3_SDA),
            5 => Some(PG11_SELECT_A::CLK_FANOUT1),
            15 => Some(PG11_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == PG11_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `I2S1_MCLK`"]
    #[inline(always)]
    pub fn is_i2s1_mclk(&self) -> bool {
        **self == PG11_SELECT_A::I2S1_MCLK
    }
    #[doc = "Checks if the value of the field is `EPHY_25M`"]
    #[inline(always)]
    pub fn is_ephy_25m(&self) -> bool {
        **self == PG11_SELECT_A::EPHY_25M
    }
    #[doc = "Checks if the value of the field is `TCON_TRIG`"]
    #[inline(always)]
    pub fn is_tcon_trig(&self) -> bool {
        **self == PG11_SELECT_A::TCON_TRIG
    }
    #[doc = "Checks if the value of the field is `PG_EINT11`"]
    #[inline(always)]
    pub fn is_pg_eint11(&self) -> bool {
        **self == PG11_SELECT_A::PG_EINT11
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == PG11_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `TWI3_SDA`"]
    #[inline(always)]
    pub fn is_twi3_sda(&self) -> bool {
        **self == PG11_SELECT_A::TWI3_SDA
    }
    #[doc = "Checks if the value of the field is `CLK_FANOUT1`"]
    #[inline(always)]
    pub fn is_clk_fanout1(&self) -> bool {
        **self == PG11_SELECT_A::CLK_FANOUT1
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        **self == PG11_SELECT_A::IO_DISABLE
    }
}
impl core::ops::Deref for PG11_SELECT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG11_SELECT` writer - PG11 Select"]
pub struct PG11_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PG11_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PG11_SELECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PG11_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn i2s1_mclk(self) -> &'a mut W {
        self.variant(PG11_SELECT_A::I2S1_MCLK)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn ephy_25m(self) -> &'a mut W {
        self.variant(PG11_SELECT_A::EPHY_25M)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn tcon_trig(self) -> &'a mut W {
        self.variant(PG11_SELECT_A::TCON_TRIG)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pg_eint11(self) -> &'a mut W {
        self.variant(PG11_SELECT_A::PG_EINT11)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PG11_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn twi3_sda(self) -> &'a mut W {
        self.variant(PG11_SELECT_A::TWI3_SDA)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn clk_fanout1(self) -> &'a mut W {
        self.variant(PG11_SELECT_A::CLK_FANOUT1)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PG11_SELECT_A::IO_DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "PG10 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PG10_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    PWM3 = 2,
    #[doc = "4: `100`"]
    RGMII_RXCK = 4,
    #[doc = "6: `110`"]
    IR_RX = 6,
    #[doc = "14: `1110`"]
    PG_EINT10 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    TWI3_SCK = 3,
    #[doc = "5: `101`"]
    CLK_FANOUT0 = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PG10_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PG10_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PG10_SELECT` reader - PG10 Select"]
pub struct PG10_SELECT_R(crate::FieldReader<u8>);
impl PG10_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PG10_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PG10_SELECT_A> {
        match self.bits {
            0 => Some(PG10_SELECT_A::INPUT),
            2 => Some(PG10_SELECT_A::PWM3),
            4 => Some(PG10_SELECT_A::RGMII_RXCK),
            6 => Some(PG10_SELECT_A::IR_RX),
            14 => Some(PG10_SELECT_A::PG_EINT10),
            1 => Some(PG10_SELECT_A::OUTPUT),
            3 => Some(PG10_SELECT_A::TWI3_SCK),
            5 => Some(PG10_SELECT_A::CLK_FANOUT0),
            15 => Some(PG10_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == PG10_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `PWM3`"]
    #[inline(always)]
    pub fn is_pwm3(&self) -> bool {
        **self == PG10_SELECT_A::PWM3
    }
    #[doc = "Checks if the value of the field is `RGMII_RXCK`"]
    #[inline(always)]
    pub fn is_rgmii_rxck(&self) -> bool {
        **self == PG10_SELECT_A::RGMII_RXCK
    }
    #[doc = "Checks if the value of the field is `IR_RX`"]
    #[inline(always)]
    pub fn is_ir_rx(&self) -> bool {
        **self == PG10_SELECT_A::IR_RX
    }
    #[doc = "Checks if the value of the field is `PG_EINT10`"]
    #[inline(always)]
    pub fn is_pg_eint10(&self) -> bool {
        **self == PG10_SELECT_A::PG_EINT10
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == PG10_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `TWI3_SCK`"]
    #[inline(always)]
    pub fn is_twi3_sck(&self) -> bool {
        **self == PG10_SELECT_A::TWI3_SCK
    }
    #[doc = "Checks if the value of the field is `CLK_FANOUT0`"]
    #[inline(always)]
    pub fn is_clk_fanout0(&self) -> bool {
        **self == PG10_SELECT_A::CLK_FANOUT0
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        **self == PG10_SELECT_A::IO_DISABLE
    }
}
impl core::ops::Deref for PG10_SELECT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG10_SELECT` writer - PG10 Select"]
pub struct PG10_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PG10_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PG10_SELECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PG10_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pwm3(self) -> &'a mut W {
        self.variant(PG10_SELECT_A::PWM3)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn rgmii_rxck(self) -> &'a mut W {
        self.variant(PG10_SELECT_A::RGMII_RXCK)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn ir_rx(self) -> &'a mut W {
        self.variant(PG10_SELECT_A::IR_RX)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pg_eint10(self) -> &'a mut W {
        self.variant(PG10_SELECT_A::PG_EINT10)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PG10_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn twi3_sck(self) -> &'a mut W {
        self.variant(PG10_SELECT_A::TWI3_SCK)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn clk_fanout0(self) -> &'a mut W {
        self.variant(PG10_SELECT_A::CLK_FANOUT0)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PG10_SELECT_A::IO_DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "PG9 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PG9_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    UART1_CTS = 2,
    #[doc = "4: `100`"]
    RGMII_RXD3 = 4,
    #[doc = "14: `1110`"]
    PG_EINT9 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    TWI1_SDA = 3,
    #[doc = "5: `101`"]
    UART3_RX = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PG9_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PG9_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PG9_SELECT` reader - PG9 Select"]
pub struct PG9_SELECT_R(crate::FieldReader<u8>);
impl PG9_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PG9_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PG9_SELECT_A> {
        match self.bits {
            0 => Some(PG9_SELECT_A::INPUT),
            2 => Some(PG9_SELECT_A::UART1_CTS),
            4 => Some(PG9_SELECT_A::RGMII_RXD3),
            14 => Some(PG9_SELECT_A::PG_EINT9),
            1 => Some(PG9_SELECT_A::OUTPUT),
            3 => Some(PG9_SELECT_A::TWI1_SDA),
            5 => Some(PG9_SELECT_A::UART3_RX),
            15 => Some(PG9_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == PG9_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `UART1_CTS`"]
    #[inline(always)]
    pub fn is_uart1_cts(&self) -> bool {
        **self == PG9_SELECT_A::UART1_CTS
    }
    #[doc = "Checks if the value of the field is `RGMII_RXD3`"]
    #[inline(always)]
    pub fn is_rgmii_rxd3(&self) -> bool {
        **self == PG9_SELECT_A::RGMII_RXD3
    }
    #[doc = "Checks if the value of the field is `PG_EINT9`"]
    #[inline(always)]
    pub fn is_pg_eint9(&self) -> bool {
        **self == PG9_SELECT_A::PG_EINT9
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == PG9_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `TWI1_SDA`"]
    #[inline(always)]
    pub fn is_twi1_sda(&self) -> bool {
        **self == PG9_SELECT_A::TWI1_SDA
    }
    #[doc = "Checks if the value of the field is `UART3_RX`"]
    #[inline(always)]
    pub fn is_uart3_rx(&self) -> bool {
        **self == PG9_SELECT_A::UART3_RX
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        **self == PG9_SELECT_A::IO_DISABLE
    }
}
impl core::ops::Deref for PG9_SELECT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG9_SELECT` writer - PG9 Select"]
pub struct PG9_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PG9_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PG9_SELECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PG9_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn uart1_cts(self) -> &'a mut W {
        self.variant(PG9_SELECT_A::UART1_CTS)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn rgmii_rxd3(self) -> &'a mut W {
        self.variant(PG9_SELECT_A::RGMII_RXD3)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pg_eint9(self) -> &'a mut W {
        self.variant(PG9_SELECT_A::PG_EINT9)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PG9_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn twi1_sda(self) -> &'a mut W {
        self.variant(PG9_SELECT_A::TWI1_SDA)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn uart3_rx(self) -> &'a mut W {
        self.variant(PG9_SELECT_A::UART3_RX)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PG9_SELECT_A::IO_DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "PG8 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PG8_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    UART1_RTS = 2,
    #[doc = "4: `100`"]
    RGMII_RXD2 = 4,
    #[doc = "14: `1110`"]
    PG_EINT8 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    TWI1_SCK = 3,
    #[doc = "5: `101`"]
    UART3_TX = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PG8_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PG8_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PG8_SELECT` reader - PG8 Select"]
pub struct PG8_SELECT_R(crate::FieldReader<u8>);
impl PG8_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PG8_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PG8_SELECT_A> {
        match self.bits {
            0 => Some(PG8_SELECT_A::INPUT),
            2 => Some(PG8_SELECT_A::UART1_RTS),
            4 => Some(PG8_SELECT_A::RGMII_RXD2),
            14 => Some(PG8_SELECT_A::PG_EINT8),
            1 => Some(PG8_SELECT_A::OUTPUT),
            3 => Some(PG8_SELECT_A::TWI1_SCK),
            5 => Some(PG8_SELECT_A::UART3_TX),
            15 => Some(PG8_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == PG8_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `UART1_RTS`"]
    #[inline(always)]
    pub fn is_uart1_rts(&self) -> bool {
        **self == PG8_SELECT_A::UART1_RTS
    }
    #[doc = "Checks if the value of the field is `RGMII_RXD2`"]
    #[inline(always)]
    pub fn is_rgmii_rxd2(&self) -> bool {
        **self == PG8_SELECT_A::RGMII_RXD2
    }
    #[doc = "Checks if the value of the field is `PG_EINT8`"]
    #[inline(always)]
    pub fn is_pg_eint8(&self) -> bool {
        **self == PG8_SELECT_A::PG_EINT8
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == PG8_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `TWI1_SCK`"]
    #[inline(always)]
    pub fn is_twi1_sck(&self) -> bool {
        **self == PG8_SELECT_A::TWI1_SCK
    }
    #[doc = "Checks if the value of the field is `UART3_TX`"]
    #[inline(always)]
    pub fn is_uart3_tx(&self) -> bool {
        **self == PG8_SELECT_A::UART3_TX
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        **self == PG8_SELECT_A::IO_DISABLE
    }
}
impl core::ops::Deref for PG8_SELECT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PG8_SELECT` writer - PG8 Select"]
pub struct PG8_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PG8_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PG8_SELECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PG8_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn uart1_rts(self) -> &'a mut W {
        self.variant(PG8_SELECT_A::UART1_RTS)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn rgmii_rxd2(self) -> &'a mut W {
        self.variant(PG8_SELECT_A::RGMII_RXD2)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pg_eint8(self) -> &'a mut W {
        self.variant(PG8_SELECT_A::PG_EINT8)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PG8_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn twi1_sck(self) -> &'a mut W {
        self.variant(PG8_SELECT_A::TWI1_SCK)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn uart3_tx(self) -> &'a mut W {
        self.variant(PG8_SELECT_A::UART3_TX)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PG8_SELECT_A::IO_DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - PG15 Select"]
    #[inline(always)]
    pub fn pg15_select(&self) -> PG15_SELECT_R {
        PG15_SELECT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PG14 Select"]
    #[inline(always)]
    pub fn pg14_select(&self) -> PG14_SELECT_R {
        PG14_SELECT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PG13 Select"]
    #[inline(always)]
    pub fn pg13_select(&self) -> PG13_SELECT_R {
        PG13_SELECT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PG12 Select"]
    #[inline(always)]
    pub fn pg12_select(&self) -> PG12_SELECT_R {
        PG12_SELECT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PG11 Select"]
    #[inline(always)]
    pub fn pg11_select(&self) -> PG11_SELECT_R {
        PG11_SELECT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PG10 Select"]
    #[inline(always)]
    pub fn pg10_select(&self) -> PG10_SELECT_R {
        PG10_SELECT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PG9 Select"]
    #[inline(always)]
    pub fn pg9_select(&self) -> PG9_SELECT_R {
        PG9_SELECT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - PG8 Select"]
    #[inline(always)]
    pub fn pg8_select(&self) -> PG8_SELECT_R {
        PG8_SELECT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - PG15 Select"]
    #[inline(always)]
    pub fn pg15_select(&mut self) -> PG15_SELECT_W {
        PG15_SELECT_W { w: self }
    }
    #[doc = "Bits 24:27 - PG14 Select"]
    #[inline(always)]
    pub fn pg14_select(&mut self) -> PG14_SELECT_W {
        PG14_SELECT_W { w: self }
    }
    #[doc = "Bits 20:23 - PG13 Select"]
    #[inline(always)]
    pub fn pg13_select(&mut self) -> PG13_SELECT_W {
        PG13_SELECT_W { w: self }
    }
    #[doc = "Bits 16:19 - PG12 Select"]
    #[inline(always)]
    pub fn pg12_select(&mut self) -> PG12_SELECT_W {
        PG12_SELECT_W { w: self }
    }
    #[doc = "Bits 12:15 - PG11 Select"]
    #[inline(always)]
    pub fn pg11_select(&mut self) -> PG11_SELECT_W {
        PG11_SELECT_W { w: self }
    }
    #[doc = "Bits 8:11 - PG10 Select"]
    #[inline(always)]
    pub fn pg10_select(&mut self) -> PG10_SELECT_W {
        PG10_SELECT_W { w: self }
    }
    #[doc = "Bits 4:7 - PG9 Select"]
    #[inline(always)]
    pub fn pg9_select(&mut self) -> PG9_SELECT_W {
        PG9_SELECT_W { w: self }
    }
    #[doc = "Bits 0:3 - PG8 Select"]
    #[inline(always)]
    pub fn pg8_select(&mut self) -> PG8_SELECT_W {
        PG8_SELECT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PG Configure Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pg_cfg1](index.html) module"]
pub struct PG_CFG1_SPEC;
impl crate::RegisterSpec for PG_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pg_cfg1::R](R) reader structure"]
impl crate::Readable for PG_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pg_cfg1::W](W) writer structure"]
impl crate::Writable for PG_CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pg_cfg1 to value 0"]
impl crate::Resettable for PG_CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
