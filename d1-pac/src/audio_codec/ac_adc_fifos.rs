#[doc = "Register `ac_adc_fifos` reader"]
pub struct R(crate::R<AC_ADC_FIFOS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AC_ADC_FIFOS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AC_ADC_FIFOS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AC_ADC_FIFOS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ac_adc_fifos` writer"]
pub struct W(crate::W<AC_ADC_FIFOS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AC_ADC_FIFOS_SPEC>;
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
impl From<crate::W<AC_ADC_FIFOS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AC_ADC_FIFOS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rxo_int` reader - RX FIFO Overrun Pending Interrupt"]
pub type RXO_INT_R = crate::BitReader<RXO_INT_A>;
#[doc = "RX FIFO Overrun Pending Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXO_INT_A {
    #[doc = "0: No Pending IRQ"]
    NO_PENDING = 0,
    #[doc = "1: FIFO Overrun Pending IRQ\n\nWrite '1' to clear this interrupt."]
    PENDING = 1,
}
impl From<RXO_INT_A> for bool {
    #[inline(always)]
    fn from(variant: RXO_INT_A) -> Self {
        variant as u8 != 0
    }
}
impl RXO_INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXO_INT_A {
        match self.bits {
            false => RXO_INT_A::NO_PENDING,
            true => RXO_INT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == RXO_INT_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RXO_INT_A::PENDING
    }
}
#[doc = "Field `rxo_int` writer - RX FIFO Overrun Pending Interrupt"]
pub type RXO_INT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, AC_ADC_FIFOS_SPEC, RXO_INT_A, O>;
impl<'a, const O: u8> RXO_INT_W<'a, O> {
    #[doc = "No Pending IRQ"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(RXO_INT_A::NO_PENDING)
    }
    #[doc = "FIFO Overrun Pending IRQ\n\nWrite '1' to clear this interrupt."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(RXO_INT_A::PENDING)
    }
}
#[doc = "Field `rxa_int` reader - RX FIFO Data Available Pending Interrupt"]
pub type RXA_INT_R = crate::BitReader<RXA_INT_A>;
#[doc = "RX FIFO Data Available Pending Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXA_INT_A {
    #[doc = "0: No Pending IRQ"]
    NO_PENDING = 0,
    #[doc = "1: Data Available Pending IRQ\n\nWrite '1' to clear this interrupt or automatic clear if the interrupt condition fails."]
    PENDING = 1,
}
impl From<RXA_INT_A> for bool {
    #[inline(always)]
    fn from(variant: RXA_INT_A) -> Self {
        variant as u8 != 0
    }
}
impl RXA_INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXA_INT_A {
        match self.bits {
            false => RXA_INT_A::NO_PENDING,
            true => RXA_INT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == RXA_INT_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RXA_INT_A::PENDING
    }
}
#[doc = "Field `rxa_int` writer - RX FIFO Data Available Pending Interrupt"]
pub type RXA_INT_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, AC_ADC_FIFOS_SPEC, RXA_INT_A, O>;
impl<'a, const O: u8> RXA_INT_W<'a, O> {
    #[doc = "No Pending IRQ"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(RXA_INT_A::NO_PENDING)
    }
    #[doc = "Data Available Pending IRQ\n\nWrite '1' to clear this interrupt or automatic clear if the interrupt condition fails."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(RXA_INT_A::PENDING)
    }
}
#[doc = "Field `rxa_cnt` reader - RX FIFO Available Sample Word Counter"]
pub type RXA_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rxa` reader - RX FIFO Available"]
pub type RXA_R = crate::BitReader<RXA_A>;
#[doc = "RX FIFO Available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXA_A {
    #[doc = "0: No available data in RX FIFO"]
    NO_AVAILABLE = 0,
    #[doc = "1: More than one sample in RX FIFO (>= 1 word)"]
    MORE = 1,
}
impl From<RXA_A> for bool {
    #[inline(always)]
    fn from(variant: RXA_A) -> Self {
        variant as u8 != 0
    }
}
impl RXA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXA_A {
        match self.bits {
            false => RXA_A::NO_AVAILABLE,
            true => RXA_A::MORE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_AVAILABLE`"]
    #[inline(always)]
    pub fn is_no_available(&self) -> bool {
        *self == RXA_A::NO_AVAILABLE
    }
    #[doc = "Checks if the value of the field is `MORE`"]
    #[inline(always)]
    pub fn is_more(&self) -> bool {
        *self == RXA_A::MORE
    }
}
impl R {
    #[doc = "Bit 1 - RX FIFO Overrun Pending Interrupt"]
    #[inline(always)]
    pub fn rxo_int(&self) -> RXO_INT_R {
        RXO_INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO Data Available Pending Interrupt"]
    #[inline(always)]
    pub fn rxa_int(&self) -> RXA_INT_R {
        RXA_INT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:16 - RX FIFO Available Sample Word Counter"]
    #[inline(always)]
    pub fn rxa_cnt(&self) -> RXA_CNT_R {
        RXA_CNT_R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bit 23 - RX FIFO Available"]
    #[inline(always)]
    pub fn rxa(&self) -> RXA_R {
        RXA_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - RX FIFO Overrun Pending Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxo_int(&mut self) -> RXO_INT_W<1> {
        RXO_INT_W::new(self)
    }
    #[doc = "Bit 3 - RX FIFO Data Available Pending Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxa_int(&mut self) -> RXA_INT_W<3> {
        RXA_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ac_adc_fifos](index.html) module"]
pub struct AC_ADC_FIFOS_SPEC;
impl crate::RegisterSpec for AC_ADC_FIFOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ac_adc_fifos::R](R) reader structure"]
impl crate::Readable for AC_ADC_FIFOS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ac_adc_fifos::W](W) writer structure"]
impl crate::Writable for AC_ADC_FIFOS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0a;
}
#[doc = "`reset()` method sets ac_adc_fifos to value 0x01"]
impl crate::Resettable for AC_ADC_FIFOS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
