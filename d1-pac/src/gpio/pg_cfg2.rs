#[doc = "Register `pg_cfg2` reader"]
pub struct R(crate::R<PG_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PG_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PG_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PG_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pg_cfg2` writer"]
pub struct W(crate::W<PG_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PG_CFG2_SPEC>;
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
impl From<crate::W<PG_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PG_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pg16_select` reader - PG16 Select"]
pub type PG16_SELECT_R = crate::FieldReader<u8, PG16_SELECT_A>;
#[doc = "PG16 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PG16_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    IR_RX = 2,
    #[doc = "4: `100`"]
    PWM5 = 4,
    #[doc = "6: `110`"]
    OWA_IN = 6,
    #[doc = "14: `1110`"]
    PG_EINT16 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    TCON_TRIG = 3,
    #[doc = "5: `101`"]
    CLK_FANOUT2 = 5,
    #[doc = "7: `111`"]
    LEDC_DO = 7,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PG16_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PG16_SELECT_A) -> Self {
        variant as _
    }
}
impl PG16_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PG16_SELECT_A> {
        match self.bits {
            0 => Some(PG16_SELECT_A::INPUT),
            2 => Some(PG16_SELECT_A::IR_RX),
            4 => Some(PG16_SELECT_A::PWM5),
            6 => Some(PG16_SELECT_A::OWA_IN),
            14 => Some(PG16_SELECT_A::PG_EINT16),
            1 => Some(PG16_SELECT_A::OUTPUT),
            3 => Some(PG16_SELECT_A::TCON_TRIG),
            5 => Some(PG16_SELECT_A::CLK_FANOUT2),
            7 => Some(PG16_SELECT_A::LEDC_DO),
            15 => Some(PG16_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PG16_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `IR_RX`"]
    #[inline(always)]
    pub fn is_ir_rx(&self) -> bool {
        *self == PG16_SELECT_A::IR_RX
    }
    #[doc = "Checks if the value of the field is `PWM5`"]
    #[inline(always)]
    pub fn is_pwm5(&self) -> bool {
        *self == PG16_SELECT_A::PWM5
    }
    #[doc = "Checks if the value of the field is `OWA_IN`"]
    #[inline(always)]
    pub fn is_owa_in(&self) -> bool {
        *self == PG16_SELECT_A::OWA_IN
    }
    #[doc = "Checks if the value of the field is `PG_EINT16`"]
    #[inline(always)]
    pub fn is_pg_eint16(&self) -> bool {
        *self == PG16_SELECT_A::PG_EINT16
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PG16_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `TCON_TRIG`"]
    #[inline(always)]
    pub fn is_tcon_trig(&self) -> bool {
        *self == PG16_SELECT_A::TCON_TRIG
    }
    #[doc = "Checks if the value of the field is `CLK_FANOUT2`"]
    #[inline(always)]
    pub fn is_clk_fanout2(&self) -> bool {
        *self == PG16_SELECT_A::CLK_FANOUT2
    }
    #[doc = "Checks if the value of the field is `LEDC_DO`"]
    #[inline(always)]
    pub fn is_ledc_do(&self) -> bool {
        *self == PG16_SELECT_A::LEDC_DO
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PG16_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pg16_select` writer - PG16 Select"]
pub type PG16_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PG_CFG2_SPEC, u8, PG16_SELECT_A, 4, O>;
impl<'a, const O: u8> PG16_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PG16_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ir_rx(self) -> &'a mut W {
        self.variant(PG16_SELECT_A::IR_RX)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn pwm5(self) -> &'a mut W {
        self.variant(PG16_SELECT_A::PWM5)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn owa_in(self) -> &'a mut W {
        self.variant(PG16_SELECT_A::OWA_IN)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pg_eint16(self) -> &'a mut W {
        self.variant(PG16_SELECT_A::PG_EINT16)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PG16_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn tcon_trig(self) -> &'a mut W {
        self.variant(PG16_SELECT_A::TCON_TRIG)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn clk_fanout2(self) -> &'a mut W {
        self.variant(PG16_SELECT_A::CLK_FANOUT2)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn ledc_do(self) -> &'a mut W {
        self.variant(PG16_SELECT_A::LEDC_DO)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PG16_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pg17_select` reader - PG17 Select"]
pub type PG17_SELECT_R = crate::FieldReader<u8, PG17_SELECT_A>;
#[doc = "PG17 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PG17_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    UART2_TX = 2,
    #[doc = "4: `100`"]
    PWM7 = 4,
    #[doc = "6: `110`"]
    IR_TX = 6,
    #[doc = "14: `1110`"]
    PG_EINT17 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    TWI3_SCK = 3,
    #[doc = "5: `101`"]
    CLK_FANOUT0 = 5,
    #[doc = "7: `111`"]
    UART0_TX = 7,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PG17_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PG17_SELECT_A) -> Self {
        variant as _
    }
}
impl PG17_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PG17_SELECT_A> {
        match self.bits {
            0 => Some(PG17_SELECT_A::INPUT),
            2 => Some(PG17_SELECT_A::UART2_TX),
            4 => Some(PG17_SELECT_A::PWM7),
            6 => Some(PG17_SELECT_A::IR_TX),
            14 => Some(PG17_SELECT_A::PG_EINT17),
            1 => Some(PG17_SELECT_A::OUTPUT),
            3 => Some(PG17_SELECT_A::TWI3_SCK),
            5 => Some(PG17_SELECT_A::CLK_FANOUT0),
            7 => Some(PG17_SELECT_A::UART0_TX),
            15 => Some(PG17_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PG17_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `UART2_TX`"]
    #[inline(always)]
    pub fn is_uart2_tx(&self) -> bool {
        *self == PG17_SELECT_A::UART2_TX
    }
    #[doc = "Checks if the value of the field is `PWM7`"]
    #[inline(always)]
    pub fn is_pwm7(&self) -> bool {
        *self == PG17_SELECT_A::PWM7
    }
    #[doc = "Checks if the value of the field is `IR_TX`"]
    #[inline(always)]
    pub fn is_ir_tx(&self) -> bool {
        *self == PG17_SELECT_A::IR_TX
    }
    #[doc = "Checks if the value of the field is `PG_EINT17`"]
    #[inline(always)]
    pub fn is_pg_eint17(&self) -> bool {
        *self == PG17_SELECT_A::PG_EINT17
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PG17_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `TWI3_SCK`"]
    #[inline(always)]
    pub fn is_twi3_sck(&self) -> bool {
        *self == PG17_SELECT_A::TWI3_SCK
    }
    #[doc = "Checks if the value of the field is `CLK_FANOUT0`"]
    #[inline(always)]
    pub fn is_clk_fanout0(&self) -> bool {
        *self == PG17_SELECT_A::CLK_FANOUT0
    }
    #[doc = "Checks if the value of the field is `UART0_TX`"]
    #[inline(always)]
    pub fn is_uart0_tx(&self) -> bool {
        *self == PG17_SELECT_A::UART0_TX
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PG17_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pg17_select` writer - PG17 Select"]
pub type PG17_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PG_CFG2_SPEC, u8, PG17_SELECT_A, 4, O>;
impl<'a, const O: u8> PG17_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PG17_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn uart2_tx(self) -> &'a mut W {
        self.variant(PG17_SELECT_A::UART2_TX)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn pwm7(self) -> &'a mut W {
        self.variant(PG17_SELECT_A::PWM7)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn ir_tx(self) -> &'a mut W {
        self.variant(PG17_SELECT_A::IR_TX)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pg_eint17(self) -> &'a mut W {
        self.variant(PG17_SELECT_A::PG_EINT17)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PG17_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn twi3_sck(self) -> &'a mut W {
        self.variant(PG17_SELECT_A::TWI3_SCK)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn clk_fanout0(self) -> &'a mut W {
        self.variant(PG17_SELECT_A::CLK_FANOUT0)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart0_tx(self) -> &'a mut W {
        self.variant(PG17_SELECT_A::UART0_TX)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PG17_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pg18_select` reader - PG18 Select"]
pub type PG18_SELECT_R = crate::FieldReader<u8, PG18_SELECT_A>;
#[doc = "PG18 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PG18_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    UART2_RX = 2,
    #[doc = "4: `100`"]
    PWM6 = 4,
    #[doc = "6: `110`"]
    OWA_OUT = 6,
    #[doc = "14: `1110`"]
    PG_EINT18 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    TWI3_SDA = 3,
    #[doc = "5: `101`"]
    CLK_FANOUT1 = 5,
    #[doc = "7: `111`"]
    UART0_RX = 7,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PG18_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PG18_SELECT_A) -> Self {
        variant as _
    }
}
impl PG18_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PG18_SELECT_A> {
        match self.bits {
            0 => Some(PG18_SELECT_A::INPUT),
            2 => Some(PG18_SELECT_A::UART2_RX),
            4 => Some(PG18_SELECT_A::PWM6),
            6 => Some(PG18_SELECT_A::OWA_OUT),
            14 => Some(PG18_SELECT_A::PG_EINT18),
            1 => Some(PG18_SELECT_A::OUTPUT),
            3 => Some(PG18_SELECT_A::TWI3_SDA),
            5 => Some(PG18_SELECT_A::CLK_FANOUT1),
            7 => Some(PG18_SELECT_A::UART0_RX),
            15 => Some(PG18_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PG18_SELECT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `UART2_RX`"]
    #[inline(always)]
    pub fn is_uart2_rx(&self) -> bool {
        *self == PG18_SELECT_A::UART2_RX
    }
    #[doc = "Checks if the value of the field is `PWM6`"]
    #[inline(always)]
    pub fn is_pwm6(&self) -> bool {
        *self == PG18_SELECT_A::PWM6
    }
    #[doc = "Checks if the value of the field is `OWA_OUT`"]
    #[inline(always)]
    pub fn is_owa_out(&self) -> bool {
        *self == PG18_SELECT_A::OWA_OUT
    }
    #[doc = "Checks if the value of the field is `PG_EINT18`"]
    #[inline(always)]
    pub fn is_pg_eint18(&self) -> bool {
        *self == PG18_SELECT_A::PG_EINT18
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PG18_SELECT_A::OUTPUT
    }
    #[doc = "Checks if the value of the field is `TWI3_SDA`"]
    #[inline(always)]
    pub fn is_twi3_sda(&self) -> bool {
        *self == PG18_SELECT_A::TWI3_SDA
    }
    #[doc = "Checks if the value of the field is `CLK_FANOUT1`"]
    #[inline(always)]
    pub fn is_clk_fanout1(&self) -> bool {
        *self == PG18_SELECT_A::CLK_FANOUT1
    }
    #[doc = "Checks if the value of the field is `UART0_RX`"]
    #[inline(always)]
    pub fn is_uart0_rx(&self) -> bool {
        *self == PG18_SELECT_A::UART0_RX
    }
    #[doc = "Checks if the value of the field is `IO_DISABLE`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PG18_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pg18_select` writer - PG18 Select"]
pub type PG18_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PG_CFG2_SPEC, u8, PG18_SELECT_A, 4, O>;
impl<'a, const O: u8> PG18_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PG18_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn uart2_rx(self) -> &'a mut W {
        self.variant(PG18_SELECT_A::UART2_RX)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn pwm6(self) -> &'a mut W {
        self.variant(PG18_SELECT_A::PWM6)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn owa_out(self) -> &'a mut W {
        self.variant(PG18_SELECT_A::OWA_OUT)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pg_eint18(self) -> &'a mut W {
        self.variant(PG18_SELECT_A::PG_EINT18)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PG18_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn twi3_sda(self) -> &'a mut W {
        self.variant(PG18_SELECT_A::TWI3_SDA)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn clk_fanout1(self) -> &'a mut W {
        self.variant(PG18_SELECT_A::CLK_FANOUT1)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart0_rx(self) -> &'a mut W {
        self.variant(PG18_SELECT_A::UART0_RX)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut W {
        self.variant(PG18_SELECT_A::IO_DISABLE)
    }
}
impl R {
    #[doc = "Bits 0:3 - PG16 Select"]
    #[inline(always)]
    pub fn pg16_select(&self) -> PG16_SELECT_R {
        PG16_SELECT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PG17 Select"]
    #[inline(always)]
    pub fn pg17_select(&self) -> PG17_SELECT_R {
        PG17_SELECT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PG18 Select"]
    #[inline(always)]
    pub fn pg18_select(&self) -> PG18_SELECT_R {
        PG18_SELECT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PG16 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg16_select(&mut self) -> PG16_SELECT_W<0> {
        PG16_SELECT_W::new(self)
    }
    #[doc = "Bits 4:7 - PG17 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg17_select(&mut self) -> PG17_SELECT_W<4> {
        PG17_SELECT_W::new(self)
    }
    #[doc = "Bits 8:11 - PG18 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg18_select(&mut self) -> PG18_SELECT_W<8> {
        PG18_SELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PG Configure Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pg_cfg2](index.html) module"]
pub struct PG_CFG2_SPEC;
impl crate::RegisterSpec for PG_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pg_cfg2::R](R) reader structure"]
impl crate::Readable for PG_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pg_cfg2::W](W) writer structure"]
impl crate::Writable for PG_CFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pg_cfg2 to value 0"]
impl crate::Resettable for PG_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
