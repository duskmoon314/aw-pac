#[doc = "Register `pf_cfg0` reader"]
pub struct R(crate::R<PF_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PF_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PF_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PF_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pf_cfg0` writer"]
pub struct W(crate::W<PF_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PF_CFG0_SPEC>;
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
impl From<crate::W<PF_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PF_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pf0_select` reader - PF0 Select"]
pub type PF0_SELECT_R = crate::FieldReader<u8, PF0_SELECT_A>;
#[doc = "PF0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PF0_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    SDC0_D1 = 2,
    #[doc = "4: `100`"]
    R_JTAG_MS = 4,
    #[doc = "6: `110`"]
    I2S2_DIN0 = 6,
    #[doc = "14: `1110`"]
    PF_EINT0 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    JTAG_MS = 3,
    #[doc = "5: `101`"]
    I2S2_DOUT1 = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PF0_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PF0_SELECT_A) -> Self {
        variant as _
    }
}
impl PF0_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PF0_SELECT_A> {
        match self.bits {
            0 => Some(PF0_SELECT_A::INPUT),
            2 => Some(PF0_SELECT_A::SDC0_D1),
            4 => Some(PF0_SELECT_A::R_JTAG_MS),
            6 => Some(PF0_SELECT_A::I2S2_DIN0),
            14 => Some(PF0_SELECT_A::PF_EINT0),
            1 => Some(PF0_SELECT_A::OUTPUT),
            3 => Some(PF0_SELECT_A::JTAG_MS),
            5 => Some(PF0_SELECT_A::I2S2_DOUT1),
            15 => Some(PF0_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PF0_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `SDC0_D1`"]
    #[inline(always)]
    pub fn is_sdc0_d1(&self) -> bool {
        *self == PF0_SELECT_A::SDC0_D1
    }
    #[doc = "Checks if the value of the field is `R_JTAG_MS`"]
    #[inline(always)]
    pub fn is_r_jtag_ms(&self) -> bool {
        *self == PF0_SELECT_A::R_JTAG_MS
    }
    #[doc = "Checks if the value of the field is `I2S2_DIN0`"]
    #[inline(always)]
    pub fn is_i2s2_din0(&self) -> bool {
        *self == PF0_SELECT_A::I2S2_DIN0
    }
    #[doc = "Checks if the value of the field is `PF_EINT0`"]
    #[inline(always)]
    pub fn is_pf_eint0(&self) -> bool {
        *self == PF0_SELECT_A::PF_EINT0
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PF0_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `JTAG_MS`"]
    #[inline(always)]
    pub fn is_jtag_ms(&self) -> bool {
        *self == PF0_SELECT_A::JTAG_MS
    }
    #[doc = "Checks if the value of the field is `I2S2_DOUT1`"]
    #[inline(always)]
    pub fn is_i2s2_dout1(&self) -> bool {
        *self == PF0_SELECT_A::I2S2_DOUT1
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PF0_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pf0_select` writer - PF0 Select"]
pub type PF0_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PF_CFG0_SPEC, u8, PF0_SELECT_A, 4, O>;
impl<'a, const O: u8> PF0_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PF0_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sdc0_d1(self) -> &'a mut W {
        self.variant(PF0_SELECT_A::SDC0_D1)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn r_jtag_ms(self) -> &'a mut W {
        self.variant(PF0_SELECT_A::R_JTAG_MS)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2s2_din0(self) -> &'a mut W {
        self.variant(PF0_SELECT_A::I2S2_DIN0)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pf_eint0(self) -> &'a mut W {
        self.variant(PF0_SELECT_A::PF_EINT0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PF0_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn jtag_ms(self) -> &'a mut W {
        self.variant(PF0_SELECT_A::JTAG_MS)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn i2s2_dout1(self) -> &'a mut W {
        self.variant(PF0_SELECT_A::I2S2_DOUT1)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PF0_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pf1_select` reader - PF1 Select"]
pub type PF1_SELECT_R = crate::FieldReader<u8, PF1_SELECT_A>;
#[doc = "PF1 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PF1_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    SDC0_D0 = 2,
    #[doc = "4: `100`"]
    R_JTAG_DI = 4,
    #[doc = "6: `110`"]
    I2S2_DIN1 = 6,
    #[doc = "14: `1110`"]
    PF_EINT1 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    JTAG_DI = 3,
    #[doc = "5: `101`"]
    I2S2_DOUT0 = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PF1_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PF1_SELECT_A) -> Self {
        variant as _
    }
}
impl PF1_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PF1_SELECT_A> {
        match self.bits {
            0 => Some(PF1_SELECT_A::INPUT),
            2 => Some(PF1_SELECT_A::SDC0_D0),
            4 => Some(PF1_SELECT_A::R_JTAG_DI),
            6 => Some(PF1_SELECT_A::I2S2_DIN1),
            14 => Some(PF1_SELECT_A::PF_EINT1),
            1 => Some(PF1_SELECT_A::OUTPUT),
            3 => Some(PF1_SELECT_A::JTAG_DI),
            5 => Some(PF1_SELECT_A::I2S2_DOUT0),
            15 => Some(PF1_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PF1_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `SDC0_D0`"]
    #[inline(always)]
    pub fn is_sdc0_d0(&self) -> bool {
        *self == PF1_SELECT_A::SDC0_D0
    }
    #[doc = "Checks if the value of the field is `R_JTAG_DI`"]
    #[inline(always)]
    pub fn is_r_jtag_di(&self) -> bool {
        *self == PF1_SELECT_A::R_JTAG_DI
    }
    #[doc = "Checks if the value of the field is `I2S2_DIN1`"]
    #[inline(always)]
    pub fn is_i2s2_din1(&self) -> bool {
        *self == PF1_SELECT_A::I2S2_DIN1
    }
    #[doc = "Checks if the value of the field is `PF_EINT1`"]
    #[inline(always)]
    pub fn is_pf_eint1(&self) -> bool {
        *self == PF1_SELECT_A::PF_EINT1
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PF1_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `JTAG_DI`"]
    #[inline(always)]
    pub fn is_jtag_di(&self) -> bool {
        *self == PF1_SELECT_A::JTAG_DI
    }
    #[doc = "Checks if the value of the field is `I2S2_DOUT0`"]
    #[inline(always)]
    pub fn is_i2s2_dout0(&self) -> bool {
        *self == PF1_SELECT_A::I2S2_DOUT0
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PF1_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pf1_select` writer - PF1 Select"]
pub type PF1_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PF_CFG0_SPEC, u8, PF1_SELECT_A, 4, O>;
impl<'a, const O: u8> PF1_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PF1_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sdc0_d0(self) -> &'a mut W {
        self.variant(PF1_SELECT_A::SDC0_D0)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn r_jtag_di(self) -> &'a mut W {
        self.variant(PF1_SELECT_A::R_JTAG_DI)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2s2_din1(self) -> &'a mut W {
        self.variant(PF1_SELECT_A::I2S2_DIN1)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pf_eint1(self) -> &'a mut W {
        self.variant(PF1_SELECT_A::PF_EINT1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PF1_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn jtag_di(self) -> &'a mut W {
        self.variant(PF1_SELECT_A::JTAG_DI)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn i2s2_dout0(self) -> &'a mut W {
        self.variant(PF1_SELECT_A::I2S2_DOUT0)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PF1_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pf2_select` reader - PF2 Select"]
pub type PF2_SELECT_R = crate::FieldReader<u8, PF2_SELECT_A>;
#[doc = "PF2 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PF2_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    SDC0_CLK = 2,
    #[doc = "4: `100`"]
    TWI0_SCK = 4,
    #[doc = "6: `110`"]
    OWA_IN = 6,
    #[doc = "14: `1110`"]
    PF_EINT2 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    UART0_TX = 3,
    #[doc = "5: `101`"]
    LEDC_DO = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PF2_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PF2_SELECT_A) -> Self {
        variant as _
    }
}
impl PF2_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PF2_SELECT_A> {
        match self.bits {
            0 => Some(PF2_SELECT_A::INPUT),
            2 => Some(PF2_SELECT_A::SDC0_CLK),
            4 => Some(PF2_SELECT_A::TWI0_SCK),
            6 => Some(PF2_SELECT_A::OWA_IN),
            14 => Some(PF2_SELECT_A::PF_EINT2),
            1 => Some(PF2_SELECT_A::OUTPUT),
            3 => Some(PF2_SELECT_A::UART0_TX),
            5 => Some(PF2_SELECT_A::LEDC_DO),
            15 => Some(PF2_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PF2_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `SDC0_CLK`"]
    #[inline(always)]
    pub fn is_sdc0_clk(&self) -> bool {
        *self == PF2_SELECT_A::SDC0_CLK
    }
    #[doc = "Checks if the value of the field is `TWI0_SCK`"]
    #[inline(always)]
    pub fn is_twi0_sck(&self) -> bool {
        *self == PF2_SELECT_A::TWI0_SCK
    }
    #[doc = "Checks if the value of the field is `OWA_IN`"]
    #[inline(always)]
    pub fn is_owa_in(&self) -> bool {
        *self == PF2_SELECT_A::OWA_IN
    }
    #[doc = "Checks if the value of the field is `PF_EINT2`"]
    #[inline(always)]
    pub fn is_pf_eint2(&self) -> bool {
        *self == PF2_SELECT_A::PF_EINT2
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PF2_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `UART0_TX`"]
    #[inline(always)]
    pub fn is_uart0_tx(&self) -> bool {
        *self == PF2_SELECT_A::UART0_TX
    }
    #[doc = "Checks if the value of the field is `LEDC_DO`"]
    #[inline(always)]
    pub fn is_ledc_do(&self) -> bool {
        *self == PF2_SELECT_A::LEDC_DO
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PF2_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pf2_select` writer - PF2 Select"]
pub type PF2_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PF_CFG0_SPEC, u8, PF2_SELECT_A, 4, O>;
impl<'a, const O: u8> PF2_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PF2_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sdc0_clk(self) -> &'a mut W {
        self.variant(PF2_SELECT_A::SDC0_CLK)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn twi0_sck(self) -> &'a mut W {
        self.variant(PF2_SELECT_A::TWI0_SCK)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn owa_in(self) -> &'a mut W {
        self.variant(PF2_SELECT_A::OWA_IN)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pf_eint2(self) -> &'a mut W {
        self.variant(PF2_SELECT_A::PF_EINT2)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PF2_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn uart0_tx(self) -> &'a mut W {
        self.variant(PF2_SELECT_A::UART0_TX)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn ledc_do(self) -> &'a mut W {
        self.variant(PF2_SELECT_A::LEDC_DO)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PF2_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pf3_select` reader - PF3 Select"]
pub type PF3_SELECT_R = crate::FieldReader<u8, PF3_SELECT_A>;
#[doc = "PF3 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PF3_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    SDC0_CMD = 2,
    #[doc = "4: `100`"]
    R_JTAG_DO = 4,
    #[doc = "14: `1110`"]
    PF_EINT3 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    JTAG_DO = 3,
    #[doc = "5: `101`"]
    I2S2_BCLK = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PF3_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PF3_SELECT_A) -> Self {
        variant as _
    }
}
impl PF3_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PF3_SELECT_A> {
        match self.bits {
            0 => Some(PF3_SELECT_A::INPUT),
            2 => Some(PF3_SELECT_A::SDC0_CMD),
            4 => Some(PF3_SELECT_A::R_JTAG_DO),
            14 => Some(PF3_SELECT_A::PF_EINT3),
            1 => Some(PF3_SELECT_A::OUTPUT),
            3 => Some(PF3_SELECT_A::JTAG_DO),
            5 => Some(PF3_SELECT_A::I2S2_BCLK),
            15 => Some(PF3_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PF3_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `SDC0_CMD`"]
    #[inline(always)]
    pub fn is_sdc0_cmd(&self) -> bool {
        *self == PF3_SELECT_A::SDC0_CMD
    }
    #[doc = "Checks if the value of the field is `R_JTAG_DO`"]
    #[inline(always)]
    pub fn is_r_jtag_do(&self) -> bool {
        *self == PF3_SELECT_A::R_JTAG_DO
    }
    #[doc = "Checks if the value of the field is `PF_EINT3`"]
    #[inline(always)]
    pub fn is_pf_eint3(&self) -> bool {
        *self == PF3_SELECT_A::PF_EINT3
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PF3_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `JTAG_DO`"]
    #[inline(always)]
    pub fn is_jtag_do(&self) -> bool {
        *self == PF3_SELECT_A::JTAG_DO
    }
    #[doc = "Checks if the value of the field is `I2S2_BCLK`"]
    #[inline(always)]
    pub fn is_i2s2_bclk(&self) -> bool {
        *self == PF3_SELECT_A::I2S2_BCLK
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PF3_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pf3_select` writer - PF3 Select"]
pub type PF3_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PF_CFG0_SPEC, u8, PF3_SELECT_A, 4, O>;
impl<'a, const O: u8> PF3_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PF3_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sdc0_cmd(self) -> &'a mut W {
        self.variant(PF3_SELECT_A::SDC0_CMD)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn r_jtag_do(self) -> &'a mut W {
        self.variant(PF3_SELECT_A::R_JTAG_DO)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pf_eint3(self) -> &'a mut W {
        self.variant(PF3_SELECT_A::PF_EINT3)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PF3_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn jtag_do(self) -> &'a mut W {
        self.variant(PF3_SELECT_A::JTAG_DO)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn i2s2_bclk(self) -> &'a mut W {
        self.variant(PF3_SELECT_A::I2S2_BCLK)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PF3_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pf4_select` reader - PF4 Select"]
pub type PF4_SELECT_R = crate::FieldReader<u8, PF4_SELECT_A>;
#[doc = "PF4 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PF4_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    SDC0_D3 = 2,
    #[doc = "4: `100`"]
    TWI0_SDA = 4,
    #[doc = "6: `110`"]
    IR_TX = 6,
    #[doc = "14: `1110`"]
    PF_EINT4 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    UART0_RX = 3,
    #[doc = "5: `101`"]
    PWM6 = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PF4_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PF4_SELECT_A) -> Self {
        variant as _
    }
}
impl PF4_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PF4_SELECT_A> {
        match self.bits {
            0 => Some(PF4_SELECT_A::INPUT),
            2 => Some(PF4_SELECT_A::SDC0_D3),
            4 => Some(PF4_SELECT_A::TWI0_SDA),
            6 => Some(PF4_SELECT_A::IR_TX),
            14 => Some(PF4_SELECT_A::PF_EINT4),
            1 => Some(PF4_SELECT_A::OUTPUT),
            3 => Some(PF4_SELECT_A::UART0_RX),
            5 => Some(PF4_SELECT_A::PWM6),
            15 => Some(PF4_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PF4_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `SDC0_D3`"]
    #[inline(always)]
    pub fn is_sdc0_d3(&self) -> bool {
        *self == PF4_SELECT_A::SDC0_D3
    }
    #[doc = "Checks if the value of the field is `TWI0_SDA`"]
    #[inline(always)]
    pub fn is_twi0_sda(&self) -> bool {
        *self == PF4_SELECT_A::TWI0_SDA
    }
    #[doc = "Checks if the value of the field is `IR_TX`"]
    #[inline(always)]
    pub fn is_ir_tx(&self) -> bool {
        *self == PF4_SELECT_A::IR_TX
    }
    #[doc = "Checks if the value of the field is `PF_EINT4`"]
    #[inline(always)]
    pub fn is_pf_eint4(&self) -> bool {
        *self == PF4_SELECT_A::PF_EINT4
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PF4_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `UART0_RX`"]
    #[inline(always)]
    pub fn is_uart0_rx(&self) -> bool {
        *self == PF4_SELECT_A::UART0_RX
    }
    #[doc = "Checks if the value of the field is `PWM6`"]
    #[inline(always)]
    pub fn is_pwm6(&self) -> bool {
        *self == PF4_SELECT_A::PWM6
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PF4_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pf4_select` writer - PF4 Select"]
pub type PF4_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PF_CFG0_SPEC, u8, PF4_SELECT_A, 4, O>;
impl<'a, const O: u8> PF4_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PF4_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sdc0_d3(self) -> &'a mut W {
        self.variant(PF4_SELECT_A::SDC0_D3)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn twi0_sda(self) -> &'a mut W {
        self.variant(PF4_SELECT_A::TWI0_SDA)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn ir_tx(self) -> &'a mut W {
        self.variant(PF4_SELECT_A::IR_TX)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pf_eint4(self) -> &'a mut W {
        self.variant(PF4_SELECT_A::PF_EINT4)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PF4_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn uart0_rx(self) -> &'a mut W {
        self.variant(PF4_SELECT_A::UART0_RX)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pwm6(self) -> &'a mut W {
        self.variant(PF4_SELECT_A::PWM6)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PF4_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pf5_select` reader - PF5 Select"]
pub type PF5_SELECT_R = crate::FieldReader<u8, PF5_SELECT_A>;
#[doc = "PF5 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PF5_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    SDC0_D2 = 2,
    #[doc = "4: `100`"]
    R_JTAG_CK = 4,
    #[doc = "14: `1110`"]
    PF_EINT5 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    JTAG_CK = 3,
    #[doc = "5: `101`"]
    I2S2_LRCK = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PF5_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PF5_SELECT_A) -> Self {
        variant as _
    }
}
impl PF5_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PF5_SELECT_A> {
        match self.bits {
            0 => Some(PF5_SELECT_A::INPUT),
            2 => Some(PF5_SELECT_A::SDC0_D2),
            4 => Some(PF5_SELECT_A::R_JTAG_CK),
            14 => Some(PF5_SELECT_A::PF_EINT5),
            1 => Some(PF5_SELECT_A::OUTPUT),
            3 => Some(PF5_SELECT_A::JTAG_CK),
            5 => Some(PF5_SELECT_A::I2S2_LRCK),
            15 => Some(PF5_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PF5_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `SDC0_D2`"]
    #[inline(always)]
    pub fn is_sdc0_d2(&self) -> bool {
        *self == PF5_SELECT_A::SDC0_D2
    }
    #[doc = "Checks if the value of the field is `R_JTAG_CK`"]
    #[inline(always)]
    pub fn is_r_jtag_ck(&self) -> bool {
        *self == PF5_SELECT_A::R_JTAG_CK
    }
    #[doc = "Checks if the value of the field is `PF_EINT5`"]
    #[inline(always)]
    pub fn is_pf_eint5(&self) -> bool {
        *self == PF5_SELECT_A::PF_EINT5
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PF5_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `JTAG_CK`"]
    #[inline(always)]
    pub fn is_jtag_ck(&self) -> bool {
        *self == PF5_SELECT_A::JTAG_CK
    }
    #[doc = "Checks if the value of the field is `I2S2_LRCK`"]
    #[inline(always)]
    pub fn is_i2s2_lrck(&self) -> bool {
        *self == PF5_SELECT_A::I2S2_LRCK
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PF5_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pf5_select` writer - PF5 Select"]
pub type PF5_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PF_CFG0_SPEC, u8, PF5_SELECT_A, 4, O>;
impl<'a, const O: u8> PF5_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PF5_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sdc0_d2(self) -> &'a mut W {
        self.variant(PF5_SELECT_A::SDC0_D2)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn r_jtag_ck(self) -> &'a mut W {
        self.variant(PF5_SELECT_A::R_JTAG_CK)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pf_eint5(self) -> &'a mut W {
        self.variant(PF5_SELECT_A::PF_EINT5)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PF5_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn jtag_ck(self) -> &'a mut W {
        self.variant(PF5_SELECT_A::JTAG_CK)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn i2s2_lrck(self) -> &'a mut W {
        self.variant(PF5_SELECT_A::I2S2_LRCK)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PF5_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pf6_select` reader - PF6 Select"]
pub type PF6_SELECT_R = crate::FieldReader<u8, PF6_SELECT_A>;
#[doc = "PF6 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PF6_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "4: `100`"]
    IR_RX = 4,
    #[doc = "6: `110`"]
    PWM5 = 6,
    #[doc = "14: `1110`"]
    PF_EINT6 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    OWA_OUT = 3,
    #[doc = "5: `101`"]
    I2S2_MCLK = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PF6_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PF6_SELECT_A) -> Self {
        variant as _
    }
}
impl PF6_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PF6_SELECT_A> {
        match self.bits {
            0 => Some(PF6_SELECT_A::INPUT),
            4 => Some(PF6_SELECT_A::IR_RX),
            6 => Some(PF6_SELECT_A::PWM5),
            14 => Some(PF6_SELECT_A::PF_EINT6),
            1 => Some(PF6_SELECT_A::OUTPUT),
            3 => Some(PF6_SELECT_A::OWA_OUT),
            5 => Some(PF6_SELECT_A::I2S2_MCLK),
            15 => Some(PF6_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PF6_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `IR_RX`"]
    #[inline(always)]
    pub fn is_ir_rx(&self) -> bool {
        *self == PF6_SELECT_A::IR_RX
    }
    #[doc = "Checks if the value of the field is `PWM5`"]
    #[inline(always)]
    pub fn is_pwm5(&self) -> bool {
        *self == PF6_SELECT_A::PWM5
    }
    #[doc = "Checks if the value of the field is `PF_EINT6`"]
    #[inline(always)]
    pub fn is_pf_eint6(&self) -> bool {
        *self == PF6_SELECT_A::PF_EINT6
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PF6_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `OWA_OUT`"]
    #[inline(always)]
    pub fn is_owa_out(&self) -> bool {
        *self == PF6_SELECT_A::OWA_OUT
    }
    #[doc = "Checks if the value of the field is `I2S2_MCLK`"]
    #[inline(always)]
    pub fn is_i2s2_mclk(&self) -> bool {
        *self == PF6_SELECT_A::I2S2_MCLK
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PF6_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pf6_select` writer - PF6 Select"]
pub type PF6_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PF_CFG0_SPEC, u8, PF6_SELECT_A, 4, O>;
impl<'a, const O: u8> PF6_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PF6_SELECT_A::INPUT)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn ir_rx(self) -> &'a mut W {
        self.variant(PF6_SELECT_A::IR_RX)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn pwm5(self) -> &'a mut W {
        self.variant(PF6_SELECT_A::PWM5)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pf_eint6(self) -> &'a mut W {
        self.variant(PF6_SELECT_A::PF_EINT6)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PF6_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn owa_out(self) -> &'a mut W {
        self.variant(PF6_SELECT_A::OWA_OUT)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn i2s2_mclk(self) -> &'a mut W {
        self.variant(PF6_SELECT_A::I2S2_MCLK)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PF6_SELECT_A::IO_DISABLE)
    }
}
impl R {
    #[doc = "Bits 0:3 - PF0 Select"]
    #[inline(always)]
    pub fn pf0_select(&self) -> PF0_SELECT_R {
        PF0_SELECT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PF1 Select"]
    #[inline(always)]
    pub fn pf1_select(&self) -> PF1_SELECT_R {
        PF1_SELECT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PF2 Select"]
    #[inline(always)]
    pub fn pf2_select(&self) -> PF2_SELECT_R {
        PF2_SELECT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PF3 Select"]
    #[inline(always)]
    pub fn pf3_select(&self) -> PF3_SELECT_R {
        PF3_SELECT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PF4 Select"]
    #[inline(always)]
    pub fn pf4_select(&self) -> PF4_SELECT_R {
        PF4_SELECT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PF5 Select"]
    #[inline(always)]
    pub fn pf5_select(&self) -> PF5_SELECT_R {
        PF5_SELECT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PF6 Select"]
    #[inline(always)]
    pub fn pf6_select(&self) -> PF6_SELECT_R {
        PF6_SELECT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PF0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pf0_select(&mut self) -> PF0_SELECT_W<0> {
        PF0_SELECT_W::new(self)
    }
    #[doc = "Bits 4:7 - PF1 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pf1_select(&mut self) -> PF1_SELECT_W<4> {
        PF1_SELECT_W::new(self)
    }
    #[doc = "Bits 8:11 - PF2 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pf2_select(&mut self) -> PF2_SELECT_W<8> {
        PF2_SELECT_W::new(self)
    }
    #[doc = "Bits 12:15 - PF3 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pf3_select(&mut self) -> PF3_SELECT_W<12> {
        PF3_SELECT_W::new(self)
    }
    #[doc = "Bits 16:19 - PF4 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pf4_select(&mut self) -> PF4_SELECT_W<16> {
        PF4_SELECT_W::new(self)
    }
    #[doc = "Bits 20:23 - PF5 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pf5_select(&mut self) -> PF5_SELECT_W<20> {
        PF5_SELECT_W::new(self)
    }
    #[doc = "Bits 24:27 - PF6 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pf6_select(&mut self) -> PF6_SELECT_W<24> {
        PF6_SELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PF Configure Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_cfg0](index.html) module"]
pub struct PF_CFG0_SPEC;
impl crate::RegisterSpec for PF_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pf_cfg0::R](R) reader structure"]
impl crate::Readable for PF_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pf_cfg0::W](W) writer structure"]
impl crate::Writable for PF_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pf_cfg0 to value 0"]
impl crate::Resettable for PF_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
