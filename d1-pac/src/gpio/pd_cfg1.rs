#[doc = "Register `pd_cfg1` reader"]
pub struct R(crate::R<PD_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pd_cfg1` writer"]
pub struct W(crate::W<PD_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD_CFG1_SPEC>;
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
impl From<crate::W<PD_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pd8_select` reader - PD8 Select"]
pub type PD8_SELECT_R = crate::FieldReader<u8, PD8_SELECT_A>;
#[doc = "PD8 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD8_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    LCD0_D12 = 2,
    #[doc = "4: `100`"]
    DSI_D3P = 4,
    #[doc = "14: `1110`"]
    PD_EINT8 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    LVDS0_V3P = 3,
    #[doc = "5: `101`"]
    UART4_RX = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PD8_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PD8_SELECT_A) -> Self {
        variant as _
    }
}
impl PD8_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PD8_SELECT_A> {
        match self.bits {
            0 => Some(PD8_SELECT_A::INPUT),
            2 => Some(PD8_SELECT_A::LCD0_D12),
            4 => Some(PD8_SELECT_A::DSI_D3P),
            14 => Some(PD8_SELECT_A::PD_EINT8),
            1 => Some(PD8_SELECT_A::OUTPUT),
            3 => Some(PD8_SELECT_A::LVDS0_V3P),
            5 => Some(PD8_SELECT_A::UART4_RX),
            15 => Some(PD8_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PD8_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `LCD0_D12`"]
    #[inline(always)]
    pub fn is_lcd0_d12(&self) -> bool {
        *self == PD8_SELECT_A::LCD0_D12
    }
    #[doc = "Checks if the value of the field is `DSI_D3P`"]
    #[inline(always)]
    pub fn is_dsi_d3p(&self) -> bool {
        *self == PD8_SELECT_A::DSI_D3P
    }
    #[doc = "Checks if the value of the field is `PD_EINT8`"]
    #[inline(always)]
    pub fn is_pd_eint8(&self) -> bool {
        *self == PD8_SELECT_A::PD_EINT8
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PD8_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `LVDS0_V3P`"]
    #[inline(always)]
    pub fn is_lvds0_v3p(&self) -> bool {
        *self == PD8_SELECT_A::LVDS0_V3P
    }
    #[doc = "Checks if the value of the field is `UART4_RX`"]
    #[inline(always)]
    pub fn is_uart4_rx(&self) -> bool {
        *self == PD8_SELECT_A::UART4_RX
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PD8_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pd8_select` writer - PD8 Select"]
pub type PD8_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PD_CFG1_SPEC, u8, PD8_SELECT_A, 4, O>;
impl<'a, const O: u8> PD8_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PD8_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_d12(self) -> &'a mut W {
        self.variant(PD8_SELECT_A::LCD0_D12)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn dsi_d3p(self) -> &'a mut W {
        self.variant(PD8_SELECT_A::DSI_D3P)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pd_eint8(self) -> &'a mut W {
        self.variant(PD8_SELECT_A::PD_EINT8)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PD8_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn lvds0_v3p(self) -> &'a mut W {
        self.variant(PD8_SELECT_A::LVDS0_V3P)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn uart4_rx(self) -> &'a mut W {
        self.variant(PD8_SELECT_A::UART4_RX)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PD8_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pd9_select` reader - PD9 Select"]
pub type PD9_SELECT_R = crate::FieldReader<u8, PD9_SELECT_A>;
#[doc = "PD9 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD9_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    LCD0_D13 = 2,
    #[doc = "4: `100`"]
    DSI_D3N = 4,
    #[doc = "14: `1110`"]
    PD_EINT9 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    LVDS0_V3N = 3,
    #[doc = "5: `101`"]
    PWM6 = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PD9_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PD9_SELECT_A) -> Self {
        variant as _
    }
}
impl PD9_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PD9_SELECT_A> {
        match self.bits {
            0 => Some(PD9_SELECT_A::INPUT),
            2 => Some(PD9_SELECT_A::LCD0_D13),
            4 => Some(PD9_SELECT_A::DSI_D3N),
            14 => Some(PD9_SELECT_A::PD_EINT9),
            1 => Some(PD9_SELECT_A::OUTPUT),
            3 => Some(PD9_SELECT_A::LVDS0_V3N),
            5 => Some(PD9_SELECT_A::PWM6),
            15 => Some(PD9_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PD9_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `LCD0_D13`"]
    #[inline(always)]
    pub fn is_lcd0_d13(&self) -> bool {
        *self == PD9_SELECT_A::LCD0_D13
    }
    #[doc = "Checks if the value of the field is `DSI_D3N`"]
    #[inline(always)]
    pub fn is_dsi_d3n(&self) -> bool {
        *self == PD9_SELECT_A::DSI_D3N
    }
    #[doc = "Checks if the value of the field is `PD_EINT9`"]
    #[inline(always)]
    pub fn is_pd_eint9(&self) -> bool {
        *self == PD9_SELECT_A::PD_EINT9
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PD9_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `LVDS0_V3N`"]
    #[inline(always)]
    pub fn is_lvds0_v3n(&self) -> bool {
        *self == PD9_SELECT_A::LVDS0_V3N
    }
    #[doc = "Checks if the value of the field is `PWM6`"]
    #[inline(always)]
    pub fn is_pwm6(&self) -> bool {
        *self == PD9_SELECT_A::PWM6
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PD9_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pd9_select` writer - PD9 Select"]
pub type PD9_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PD_CFG1_SPEC, u8, PD9_SELECT_A, 4, O>;
impl<'a, const O: u8> PD9_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PD9_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_d13(self) -> &'a mut W {
        self.variant(PD9_SELECT_A::LCD0_D13)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn dsi_d3n(self) -> &'a mut W {
        self.variant(PD9_SELECT_A::DSI_D3N)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pd_eint9(self) -> &'a mut W {
        self.variant(PD9_SELECT_A::PD_EINT9)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PD9_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn lvds0_v3n(self) -> &'a mut W {
        self.variant(PD9_SELECT_A::LVDS0_V3N)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pwm6(self) -> &'a mut W {
        self.variant(PD9_SELECT_A::PWM6)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PD9_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pd10_select` reader - PD10 Select"]
pub type PD10_SELECT_R = crate::FieldReader<u8, PD10_SELECT_A>;
#[doc = "PD10 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD10_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    LCD0_D14 = 2,
    #[doc = "4: `100`"]
    SPI1_CS_DBI_CSX = 4,
    #[doc = "14: `1110`"]
    PD_EINT10 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    LVDS1_V0P = 3,
    #[doc = "5: `101`"]
    UART3_TX = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PD10_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PD10_SELECT_A) -> Self {
        variant as _
    }
}
impl PD10_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PD10_SELECT_A> {
        match self.bits {
            0 => Some(PD10_SELECT_A::INPUT),
            2 => Some(PD10_SELECT_A::LCD0_D14),
            4 => Some(PD10_SELECT_A::SPI1_CS_DBI_CSX),
            14 => Some(PD10_SELECT_A::PD_EINT10),
            1 => Some(PD10_SELECT_A::OUTPUT),
            3 => Some(PD10_SELECT_A::LVDS1_V0P),
            5 => Some(PD10_SELECT_A::UART3_TX),
            15 => Some(PD10_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PD10_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `LCD0_D14`"]
    #[inline(always)]
    pub fn is_lcd0_d14(&self) -> bool {
        *self == PD10_SELECT_A::LCD0_D14
    }
    #[doc = "Checks if the value of the field is `SPI1_CS_DBI_CSX`"]
    #[inline(always)]
    pub fn is_spi1_cs_dbi_csx(&self) -> bool {
        *self == PD10_SELECT_A::SPI1_CS_DBI_CSX
    }
    #[doc = "Checks if the value of the field is `PD_EINT10`"]
    #[inline(always)]
    pub fn is_pd_eint10(&self) -> bool {
        *self == PD10_SELECT_A::PD_EINT10
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PD10_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `LVDS1_V0P`"]
    #[inline(always)]
    pub fn is_lvds1_v0p(&self) -> bool {
        *self == PD10_SELECT_A::LVDS1_V0P
    }
    #[doc = "Checks if the value of the field is `UART3_TX`"]
    #[inline(always)]
    pub fn is_uart3_tx(&self) -> bool {
        *self == PD10_SELECT_A::UART3_TX
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PD10_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pd10_select` writer - PD10 Select"]
pub type PD10_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PD_CFG1_SPEC, u8, PD10_SELECT_A, 4, O>;
impl<'a, const O: u8> PD10_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PD10_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_d14(self) -> &'a mut W {
        self.variant(PD10_SELECT_A::LCD0_D14)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi1_cs_dbi_csx(self) -> &'a mut W {
        self.variant(PD10_SELECT_A::SPI1_CS_DBI_CSX)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pd_eint10(self) -> &'a mut W {
        self.variant(PD10_SELECT_A::PD_EINT10)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PD10_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn lvds1_v0p(self) -> &'a mut W {
        self.variant(PD10_SELECT_A::LVDS1_V0P)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn uart3_tx(self) -> &'a mut W {
        self.variant(PD10_SELECT_A::UART3_TX)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PD10_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pd11_select` reader - PD11 Select"]
pub type PD11_SELECT_R = crate::FieldReader<u8, PD11_SELECT_A>;
#[doc = "PD11 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD11_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    LCD0_D15 = 2,
    #[doc = "4: `100`"]
    SPI1_CLK_DBI_SCLK = 4,
    #[doc = "14: `1110`"]
    PD_EINT11 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    LVDS1_V0N = 3,
    #[doc = "5: `101`"]
    UART3_RX = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PD11_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PD11_SELECT_A) -> Self {
        variant as _
    }
}
impl PD11_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PD11_SELECT_A> {
        match self.bits {
            0 => Some(PD11_SELECT_A::INPUT),
            2 => Some(PD11_SELECT_A::LCD0_D15),
            4 => Some(PD11_SELECT_A::SPI1_CLK_DBI_SCLK),
            14 => Some(PD11_SELECT_A::PD_EINT11),
            1 => Some(PD11_SELECT_A::OUTPUT),
            3 => Some(PD11_SELECT_A::LVDS1_V0N),
            5 => Some(PD11_SELECT_A::UART3_RX),
            15 => Some(PD11_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PD11_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `LCD0_D15`"]
    #[inline(always)]
    pub fn is_lcd0_d15(&self) -> bool {
        *self == PD11_SELECT_A::LCD0_D15
    }
    #[doc = "Checks if the value of the field is `SPI1_CLK_DBI_SCLK`"]
    #[inline(always)]
    pub fn is_spi1_clk_dbi_sclk(&self) -> bool {
        *self == PD11_SELECT_A::SPI1_CLK_DBI_SCLK
    }
    #[doc = "Checks if the value of the field is `PD_EINT11`"]
    #[inline(always)]
    pub fn is_pd_eint11(&self) -> bool {
        *self == PD11_SELECT_A::PD_EINT11
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PD11_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `LVDS1_V0N`"]
    #[inline(always)]
    pub fn is_lvds1_v0n(&self) -> bool {
        *self == PD11_SELECT_A::LVDS1_V0N
    }
    #[doc = "Checks if the value of the field is `UART3_RX`"]
    #[inline(always)]
    pub fn is_uart3_rx(&self) -> bool {
        *self == PD11_SELECT_A::UART3_RX
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PD11_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pd11_select` writer - PD11 Select"]
pub type PD11_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PD_CFG1_SPEC, u8, PD11_SELECT_A, 4, O>;
impl<'a, const O: u8> PD11_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PD11_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_d15(self) -> &'a mut W {
        self.variant(PD11_SELECT_A::LCD0_D15)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi1_clk_dbi_sclk(self) -> &'a mut W {
        self.variant(PD11_SELECT_A::SPI1_CLK_DBI_SCLK)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pd_eint11(self) -> &'a mut W {
        self.variant(PD11_SELECT_A::PD_EINT11)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PD11_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn lvds1_v0n(self) -> &'a mut W {
        self.variant(PD11_SELECT_A::LVDS1_V0N)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn uart3_rx(self) -> &'a mut W {
        self.variant(PD11_SELECT_A::UART3_RX)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PD11_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pd12_select` reader - PD12 Select"]
pub type PD12_SELECT_R = crate::FieldReader<u8, PD12_SELECT_A>;
#[doc = "PD12 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD12_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    LCD0_D18 = 2,
    #[doc = "4: `100`"]
    SPI1_MOSI_DBI_SDO = 4,
    #[doc = "14: `1110`"]
    PD_EINT12 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    LVDS1_V1P = 3,
    #[doc = "5: `101`"]
    TWI0_SDA = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PD12_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PD12_SELECT_A) -> Self {
        variant as _
    }
}
impl PD12_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PD12_SELECT_A> {
        match self.bits {
            0 => Some(PD12_SELECT_A::INPUT),
            2 => Some(PD12_SELECT_A::LCD0_D18),
            4 => Some(PD12_SELECT_A::SPI1_MOSI_DBI_SDO),
            14 => Some(PD12_SELECT_A::PD_EINT12),
            1 => Some(PD12_SELECT_A::OUTPUT),
            3 => Some(PD12_SELECT_A::LVDS1_V1P),
            5 => Some(PD12_SELECT_A::TWI0_SDA),
            15 => Some(PD12_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PD12_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `LCD0_D18`"]
    #[inline(always)]
    pub fn is_lcd0_d18(&self) -> bool {
        *self == PD12_SELECT_A::LCD0_D18
    }
    #[doc = "Checks if the value of the field is `SPI1_MOSI_DBI_SDO`"]
    #[inline(always)]
    pub fn is_spi1_mosi_dbi_sdo(&self) -> bool {
        *self == PD12_SELECT_A::SPI1_MOSI_DBI_SDO
    }
    #[doc = "Checks if the value of the field is `PD_EINT12`"]
    #[inline(always)]
    pub fn is_pd_eint12(&self) -> bool {
        *self == PD12_SELECT_A::PD_EINT12
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PD12_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `LVDS1_V1P`"]
    #[inline(always)]
    pub fn is_lvds1_v1p(&self) -> bool {
        *self == PD12_SELECT_A::LVDS1_V1P
    }
    #[doc = "Checks if the value of the field is `TWI0_SDA`"]
    #[inline(always)]
    pub fn is_twi0_sda(&self) -> bool {
        *self == PD12_SELECT_A::TWI0_SDA
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PD12_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pd12_select` writer - PD12 Select"]
pub type PD12_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PD_CFG1_SPEC, u8, PD12_SELECT_A, 4, O>;
impl<'a, const O: u8> PD12_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PD12_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_d18(self) -> &'a mut W {
        self.variant(PD12_SELECT_A::LCD0_D18)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi1_mosi_dbi_sdo(self) -> &'a mut W {
        self.variant(PD12_SELECT_A::SPI1_MOSI_DBI_SDO)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pd_eint12(self) -> &'a mut W {
        self.variant(PD12_SELECT_A::PD_EINT12)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PD12_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn lvds1_v1p(self) -> &'a mut W {
        self.variant(PD12_SELECT_A::LVDS1_V1P)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn twi0_sda(self) -> &'a mut W {
        self.variant(PD12_SELECT_A::TWI0_SDA)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PD12_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pd13_select` reader - PD13 Select"]
pub type PD13_SELECT_R = crate::FieldReader<u8, PD13_SELECT_A>;
#[doc = "PD13 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD13_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "2: `10`"]
    LCD0_D19 = 2,
    #[doc = "3: `11`"]
    LVDS1_V1N = 3,
    #[doc = "4: `100`"]
    SPI1_MISO_DBI_SDI_DBI_TE_DBI_DCX = 4,
    #[doc = "5: `101`"]
    UART3_RTS = 5,
    #[doc = "14: `1110`"]
    PD_EINT13 = 14,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PD13_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PD13_SELECT_A) -> Self {
        variant as _
    }
}
impl PD13_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PD13_SELECT_A> {
        match self.bits {
            0 => Some(PD13_SELECT_A::INPUT),
            1 => Some(PD13_SELECT_A::OUTPUT),
            2 => Some(PD13_SELECT_A::LCD0_D19),
            3 => Some(PD13_SELECT_A::LVDS1_V1N),
            4 => Some(PD13_SELECT_A::SPI1_MISO_DBI_SDI_DBI_TE_DBI_DCX),
            5 => Some(PD13_SELECT_A::UART3_RTS),
            14 => Some(PD13_SELECT_A::PD_EINT13),
            15 => Some(PD13_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PD13_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PD13_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `LCD0_D19`"]
    #[inline(always)]
    pub fn is_lcd0_d19(&self) -> bool {
        *self == PD13_SELECT_A::LCD0_D19
    }
    #[doc = "Checks if the value of the field is `LVDS1_V1N`"]
    #[inline(always)]
    pub fn is_lvds1_v1n(&self) -> bool {
        *self == PD13_SELECT_A::LVDS1_V1N
    }
    #[doc = "Checks if the value of the field is `SPI1_MISO_DBI_SDI_DBI_TE_DBI_DCX`"]
    #[inline(always)]
    pub fn is_spi1_miso_dbi_sdi_dbi_te_dbi_dcx(&self) -> bool {
        *self == PD13_SELECT_A::SPI1_MISO_DBI_SDI_DBI_TE_DBI_DCX
    }
    #[doc = "Checks if the value of the field is `UART3_RTS`"]
    #[inline(always)]
    pub fn is_uart3_rts(&self) -> bool {
        *self == PD13_SELECT_A::UART3_RTS
    }
    #[doc = "Checks if the value of the field is `PD_EINT13`"]
    #[inline(always)]
    pub fn is_pd_eint13(&self) -> bool {
        *self == PD13_SELECT_A::PD_EINT13
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PD13_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pd13_select` writer - PD13 Select"]
pub type PD13_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PD_CFG1_SPEC, u8, PD13_SELECT_A, 4, O>;
impl<'a, const O: u8> PD13_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PD13_SELECT_A::INPUT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PD13_SELECT_A::OUTPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_d19(self) -> &'a mut W {
        self.variant(PD13_SELECT_A::LCD0_D19)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn lvds1_v1n(self) -> &'a mut W {
        self.variant(PD13_SELECT_A::LVDS1_V1N)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi1_miso_dbi_sdi_dbi_te_dbi_dcx(self) -> &'a mut W {
        self.variant(PD13_SELECT_A::SPI1_MISO_DBI_SDI_DBI_TE_DBI_DCX)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn uart3_rts(self) -> &'a mut W {
        self.variant(PD13_SELECT_A::UART3_RTS)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pd_eint13(self) -> &'a mut W {
        self.variant(PD13_SELECT_A::PD_EINT13)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PD13_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pd14_select` reader - PD14 Select"]
pub type PD14_SELECT_R = crate::FieldReader<u8, PD14_SELECT_A>;
#[doc = "PD14 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD14_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    LCD0_D20 = 2,
    #[doc = "4: `100`"]
    SPI1_HOLD_DBI_DCX_DBI_WRX = 4,
    #[doc = "14: `1110`"]
    PD_EINT14 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    LVDS1_V2P = 3,
    #[doc = "5: `101`"]
    UART3_CTS = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PD14_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PD14_SELECT_A) -> Self {
        variant as _
    }
}
impl PD14_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PD14_SELECT_A> {
        match self.bits {
            0 => Some(PD14_SELECT_A::INPUT),
            2 => Some(PD14_SELECT_A::LCD0_D20),
            4 => Some(PD14_SELECT_A::SPI1_HOLD_DBI_DCX_DBI_WRX),
            14 => Some(PD14_SELECT_A::PD_EINT14),
            1 => Some(PD14_SELECT_A::OUTPUT),
            3 => Some(PD14_SELECT_A::LVDS1_V2P),
            5 => Some(PD14_SELECT_A::UART3_CTS),
            15 => Some(PD14_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PD14_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `LCD0_D20`"]
    #[inline(always)]
    pub fn is_lcd0_d20(&self) -> bool {
        *self == PD14_SELECT_A::LCD0_D20
    }
    #[doc = "Checks if the value of the field is `SPI1_HOLD_DBI_DCX_DBI_WRX`"]
    #[inline(always)]
    pub fn is_spi1_hold_dbi_dcx_dbi_wrx(&self) -> bool {
        *self == PD14_SELECT_A::SPI1_HOLD_DBI_DCX_DBI_WRX
    }
    #[doc = "Checks if the value of the field is `PD_EINT14`"]
    #[inline(always)]
    pub fn is_pd_eint14(&self) -> bool {
        *self == PD14_SELECT_A::PD_EINT14
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PD14_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `LVDS1_V2P`"]
    #[inline(always)]
    pub fn is_lvds1_v2p(&self) -> bool {
        *self == PD14_SELECT_A::LVDS1_V2P
    }
    #[doc = "Checks if the value of the field is `UART3_CTS`"]
    #[inline(always)]
    pub fn is_uart3_cts(&self) -> bool {
        *self == PD14_SELECT_A::UART3_CTS
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PD14_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pd14_select` writer - PD14 Select"]
pub type PD14_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PD_CFG1_SPEC, u8, PD14_SELECT_A, 4, O>;
impl<'a, const O: u8> PD14_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PD14_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_d20(self) -> &'a mut W {
        self.variant(PD14_SELECT_A::LCD0_D20)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi1_hold_dbi_dcx_dbi_wrx(self) -> &'a mut W {
        self.variant(PD14_SELECT_A::SPI1_HOLD_DBI_DCX_DBI_WRX)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pd_eint14(self) -> &'a mut W {
        self.variant(PD14_SELECT_A::PD_EINT14)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PD14_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn lvds1_v2p(self) -> &'a mut W {
        self.variant(PD14_SELECT_A::LVDS1_V2P)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn uart3_cts(self) -> &'a mut W {
        self.variant(PD14_SELECT_A::UART3_CTS)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PD14_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pd15_select` reader - PD15 Select"]
pub type PD15_SELECT_R = crate::FieldReader<u8, PD15_SELECT_A>;
#[doc = "PD15 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD15_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    LCD0_D21 = 2,
    #[doc = "4: `100`"]
    SPI1_WP_DBI_TE = 4,
    #[doc = "14: `1110`"]
    PD_EINT15 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    LVDS1_V2N = 3,
    #[doc = "5: `101`"]
    IR_RX = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PD15_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PD15_SELECT_A) -> Self {
        variant as _
    }
}
impl PD15_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PD15_SELECT_A> {
        match self.bits {
            0 => Some(PD15_SELECT_A::INPUT),
            2 => Some(PD15_SELECT_A::LCD0_D21),
            4 => Some(PD15_SELECT_A::SPI1_WP_DBI_TE),
            14 => Some(PD15_SELECT_A::PD_EINT15),
            1 => Some(PD15_SELECT_A::OUTPUT),
            3 => Some(PD15_SELECT_A::LVDS1_V2N),
            5 => Some(PD15_SELECT_A::IR_RX),
            15 => Some(PD15_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PD15_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `LCD0_D21`"]
    #[inline(always)]
    pub fn is_lcd0_d21(&self) -> bool {
        *self == PD15_SELECT_A::LCD0_D21
    }
    #[doc = "Checks if the value of the field is `SPI1_WP_DBI_TE`"]
    #[inline(always)]
    pub fn is_spi1_wp_dbi_te(&self) -> bool {
        *self == PD15_SELECT_A::SPI1_WP_DBI_TE
    }
    #[doc = "Checks if the value of the field is `PD_EINT15`"]
    #[inline(always)]
    pub fn is_pd_eint15(&self) -> bool {
        *self == PD15_SELECT_A::PD_EINT15
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PD15_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `LVDS1_V2N`"]
    #[inline(always)]
    pub fn is_lvds1_v2n(&self) -> bool {
        *self == PD15_SELECT_A::LVDS1_V2N
    }
    #[doc = "Checks if the value of the field is `IR_RX`"]
    #[inline(always)]
    pub fn is_ir_rx(&self) -> bool {
        *self == PD15_SELECT_A::IR_RX
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PD15_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pd15_select` writer - PD15 Select"]
pub type PD15_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PD_CFG1_SPEC, u8, PD15_SELECT_A, 4, O>;
impl<'a, const O: u8> PD15_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PD15_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn lcd0_d21(self) -> &'a mut W {
        self.variant(PD15_SELECT_A::LCD0_D21)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi1_wp_dbi_te(self) -> &'a mut W {
        self.variant(PD15_SELECT_A::SPI1_WP_DBI_TE)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pd_eint15(self) -> &'a mut W {
        self.variant(PD15_SELECT_A::PD_EINT15)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PD15_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn lvds1_v2n(self) -> &'a mut W {
        self.variant(PD15_SELECT_A::LVDS1_V2N)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn ir_rx(self) -> &'a mut W {
        self.variant(PD15_SELECT_A::IR_RX)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PD15_SELECT_A::IO_DISABLE)
    }
}
impl R {
    #[doc = "Bits 0:3 - PD8 Select"]
    #[inline(always)]
    pub fn pd8_select(&self) -> PD8_SELECT_R {
        PD8_SELECT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PD9 Select"]
    #[inline(always)]
    pub fn pd9_select(&self) -> PD9_SELECT_R {
        PD9_SELECT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PD10 Select"]
    #[inline(always)]
    pub fn pd10_select(&self) -> PD10_SELECT_R {
        PD10_SELECT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PD11 Select"]
    #[inline(always)]
    pub fn pd11_select(&self) -> PD11_SELECT_R {
        PD11_SELECT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PD12 Select"]
    #[inline(always)]
    pub fn pd12_select(&self) -> PD12_SELECT_R {
        PD12_SELECT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PD13 Select"]
    #[inline(always)]
    pub fn pd13_select(&self) -> PD13_SELECT_R {
        PD13_SELECT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PD14 Select"]
    #[inline(always)]
    pub fn pd14_select(&self) -> PD14_SELECT_R {
        PD14_SELECT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PD15 Select"]
    #[inline(always)]
    pub fn pd15_select(&self) -> PD15_SELECT_R {
        PD15_SELECT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PD8 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd8_select(&mut self) -> PD8_SELECT_W<0> {
        PD8_SELECT_W::new(self)
    }
    #[doc = "Bits 4:7 - PD9 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd9_select(&mut self) -> PD9_SELECT_W<4> {
        PD9_SELECT_W::new(self)
    }
    #[doc = "Bits 8:11 - PD10 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd10_select(&mut self) -> PD10_SELECT_W<8> {
        PD10_SELECT_W::new(self)
    }
    #[doc = "Bits 12:15 - PD11 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd11_select(&mut self) -> PD11_SELECT_W<12> {
        PD11_SELECT_W::new(self)
    }
    #[doc = "Bits 16:19 - PD12 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd12_select(&mut self) -> PD12_SELECT_W<16> {
        PD12_SELECT_W::new(self)
    }
    #[doc = "Bits 20:23 - PD13 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd13_select(&mut self) -> PD13_SELECT_W<20> {
        PD13_SELECT_W::new(self)
    }
    #[doc = "Bits 24:27 - PD14 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd14_select(&mut self) -> PD14_SELECT_W<24> {
        PD14_SELECT_W::new(self)
    }
    #[doc = "Bits 28:31 - PD15 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd15_select(&mut self) -> PD15_SELECT_W<28> {
        PD15_SELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PD Configure Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_cfg1](index.html) module"]
pub struct PD_CFG1_SPEC;
impl crate::RegisterSpec for PD_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd_cfg1::R](R) reader structure"]
impl crate::Readable for PD_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd_cfg1::W](W) writer structure"]
impl crate::Writable for PD_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pd_cfg1 to value 0"]
impl crate::Resettable for PD_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
