#[doc = "Register `pe_cfg2` reader"]
pub struct R(crate::R<PE_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PE_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PE_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PE_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pe_cfg2` writer"]
pub struct W(crate::W<PE_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PE_CFG2_SPEC>;
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
impl From<crate::W<PE_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PE_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pe16_select` reader - PE16 Select"]
pub type PE16_SELECT_R = crate::FieldReader<u8, PE16_SELECT_A>;
#[doc = "PE16 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PE16_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    TWI3_SCK = 2,
    #[doc = "4: `100`"]
    PWM7 = 4,
    #[doc = "6: `110`"]
    DMIC_DATA0 = 6,
    #[doc = "14: `1110`"]
    PE_EINT16 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    D_JTAG_DO = 3,
    #[doc = "5: `101`"]
    I2S0_BCLK = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PE16_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PE16_SELECT_A) -> Self {
        variant as _
    }
}
impl PE16_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PE16_SELECT_A> {
        match self.bits {
            0 => Some(PE16_SELECT_A::INPUT),
            2 => Some(PE16_SELECT_A::TWI3_SCK),
            4 => Some(PE16_SELECT_A::PWM7),
            6 => Some(PE16_SELECT_A::DMIC_DATA0),
            14 => Some(PE16_SELECT_A::PE_EINT16),
            1 => Some(PE16_SELECT_A::OUTPUT),
            3 => Some(PE16_SELECT_A::D_JTAG_DO),
            5 => Some(PE16_SELECT_A::I2S0_BCLK),
            15 => Some(PE16_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PE16_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `TWI3_SCK`"]
    #[inline(always)]
    pub fn is_twi3_sck(&self) -> bool {
        *self == PE16_SELECT_A::TWI3_SCK
    }
    #[doc = "Checks if the value of the field is `PWM7`"]
    #[inline(always)]
    pub fn is_pwm7(&self) -> bool {
        *self == PE16_SELECT_A::PWM7
    }
    #[doc = "Checks if the value of the field is `DMIC_DATA0`"]
    #[inline(always)]
    pub fn is_dmic_data0(&self) -> bool {
        *self == PE16_SELECT_A::DMIC_DATA0
    }
    #[doc = "Checks if the value of the field is `PE_EINT16`"]
    #[inline(always)]
    pub fn is_pe_eint16(&self) -> bool {
        *self == PE16_SELECT_A::PE_EINT16
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PE16_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `D_JTAG_DO`"]
    #[inline(always)]
    pub fn is_d_jtag_do(&self) -> bool {
        *self == PE16_SELECT_A::D_JTAG_DO
    }
    #[doc = "Checks if the value of the field is `I2S0_BCLK`"]
    #[inline(always)]
    pub fn is_i2s0_bclk(&self) -> bool {
        *self == PE16_SELECT_A::I2S0_BCLK
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PE16_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pe16_select` writer - PE16 Select"]
pub type PE16_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PE_CFG2_SPEC, u8, PE16_SELECT_A, 4, O>;
impl<'a, const O: u8> PE16_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PE16_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn twi3_sck(self) -> &'a mut W {
        self.variant(PE16_SELECT_A::TWI3_SCK)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn pwm7(self) -> &'a mut W {
        self.variant(PE16_SELECT_A::PWM7)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn dmic_data0(self) -> &'a mut W {
        self.variant(PE16_SELECT_A::DMIC_DATA0)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pe_eint16(self) -> &'a mut W {
        self.variant(PE16_SELECT_A::PE_EINT16)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PE16_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn d_jtag_do(self) -> &'a mut W {
        self.variant(PE16_SELECT_A::D_JTAG_DO)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn i2s0_bclk(self) -> &'a mut W {
        self.variant(PE16_SELECT_A::I2S0_BCLK)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PE16_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pe17_select` reader - PE17 Select"]
pub type PE17_SELECT_R = crate::FieldReader<u8, PE17_SELECT_A>;
#[doc = "PE17 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PE17_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    TWI3_SDA = 2,
    #[doc = "4: `100`"]
    IR_TX = 4,
    #[doc = "6: `110`"]
    DMIC_CLK = 6,
    #[doc = "14: `1110`"]
    PE_EINT17 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    D_JTAG_CK = 3,
    #[doc = "5: `101`"]
    I2S0_MCLK = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PE17_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PE17_SELECT_A) -> Self {
        variant as _
    }
}
impl PE17_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PE17_SELECT_A> {
        match self.bits {
            0 => Some(PE17_SELECT_A::INPUT),
            2 => Some(PE17_SELECT_A::TWI3_SDA),
            4 => Some(PE17_SELECT_A::IR_TX),
            6 => Some(PE17_SELECT_A::DMIC_CLK),
            14 => Some(PE17_SELECT_A::PE_EINT17),
            1 => Some(PE17_SELECT_A::OUTPUT),
            3 => Some(PE17_SELECT_A::D_JTAG_CK),
            5 => Some(PE17_SELECT_A::I2S0_MCLK),
            15 => Some(PE17_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PE17_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `TWI3_SDA`"]
    #[inline(always)]
    pub fn is_twi3_sda(&self) -> bool {
        *self == PE17_SELECT_A::TWI3_SDA
    }
    #[doc = "Checks if the value of the field is `IR_TX`"]
    #[inline(always)]
    pub fn is_ir_tx(&self) -> bool {
        *self == PE17_SELECT_A::IR_TX
    }
    #[doc = "Checks if the value of the field is `DMIC_CLK`"]
    #[inline(always)]
    pub fn is_dmic_clk(&self) -> bool {
        *self == PE17_SELECT_A::DMIC_CLK
    }
    #[doc = "Checks if the value of the field is `PE_EINT17`"]
    #[inline(always)]
    pub fn is_pe_eint17(&self) -> bool {
        *self == PE17_SELECT_A::PE_EINT17
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PE17_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `D_JTAG_CK`"]
    #[inline(always)]
    pub fn is_d_jtag_ck(&self) -> bool {
        *self == PE17_SELECT_A::D_JTAG_CK
    }
    #[doc = "Checks if the value of the field is `I2S0_MCLK`"]
    #[inline(always)]
    pub fn is_i2s0_mclk(&self) -> bool {
        *self == PE17_SELECT_A::I2S0_MCLK
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PE17_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pe17_select` writer - PE17 Select"]
pub type PE17_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PE_CFG2_SPEC, u8, PE17_SELECT_A, 4, O>;
impl<'a, const O: u8> PE17_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PE17_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn twi3_sda(self) -> &'a mut W {
        self.variant(PE17_SELECT_A::TWI3_SDA)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn ir_tx(self) -> &'a mut W {
        self.variant(PE17_SELECT_A::IR_TX)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn dmic_clk(self) -> &'a mut W {
        self.variant(PE17_SELECT_A::DMIC_CLK)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pe_eint17(self) -> &'a mut W {
        self.variant(PE17_SELECT_A::PE_EINT17)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PE17_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn d_jtag_ck(self) -> &'a mut W {
        self.variant(PE17_SELECT_A::D_JTAG_CK)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn i2s0_mclk(self) -> &'a mut W {
        self.variant(PE17_SELECT_A::I2S0_MCLK)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PE17_SELECT_A::IO_DISABLE)
    }
}
impl R {
    #[doc = "Bits 0:3 - PE16 Select"]
    #[inline(always)]
    pub fn pe16_select(&self) -> PE16_SELECT_R {
        PE16_SELECT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PE17 Select"]
    #[inline(always)]
    pub fn pe17_select(&self) -> PE17_SELECT_R {
        PE17_SELECT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PE16 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe16_select(&mut self) -> PE16_SELECT_W<0> {
        PE16_SELECT_W::new(self)
    }
    #[doc = "Bits 4:7 - PE17 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe17_select(&mut self) -> PE17_SELECT_W<4> {
        PE17_SELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PE Configure Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe_cfg2](index.html) module"]
pub struct PE_CFG2_SPEC;
impl crate::RegisterSpec for PE_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pe_cfg2::R](R) reader structure"]
impl crate::Readable for PE_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pe_cfg2::W](W) writer structure"]
impl crate::Writable for PE_CFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pe_cfg2 to value 0"]
impl crate::Resettable for PE_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
