#[doc = "Register `gp_fifo_ints` reader"]
pub struct R(crate::R<GP_FIFO_INTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_FIFO_INTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_FIFO_INTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_FIFO_INTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gp_fifo_ints` writer"]
pub struct W(crate::W<GP_FIFO_INTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_FIFO_INTS_SPEC>;
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
impl From<crate::W<GP_FIFO_INTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_FIFO_INTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rxa_cnt` reader - ADC FIFO available sample word counter"]
pub type RXA_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `fifo_data_pending` reader - ADC FIFO Data Available Pending Bit"]
pub type FIFO_DATA_PENDING_R = crate::BitReader<FIFO_DATA_PENDING_A>;
#[doc = "ADC FIFO Data Available Pending Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFO_DATA_PENDING_A {
    #[doc = "0: NO Pending IRQ"]
    NO_PENDING = 0,
    #[doc = "1: FIFO Available Pending IRQ"]
    PENDING = 1,
}
impl From<FIFO_DATA_PENDING_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_DATA_PENDING_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_DATA_PENDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_DATA_PENDING_A {
        match self.bits {
            false => FIFO_DATA_PENDING_A::NO_PENDING,
            true => FIFO_DATA_PENDING_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == FIFO_DATA_PENDING_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == FIFO_DATA_PENDING_A::PENDING
    }
}
#[doc = "Field `fifo_data_pending` writer - ADC FIFO Data Available Pending Bit"]
pub type FIFO_DATA_PENDING_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, GP_FIFO_INTS_SPEC, FIFO_DATA_PENDING_A, O>;
impl<'a, const O: u8> FIFO_DATA_PENDING_W<'a, O> {
    #[doc = "NO Pending IRQ"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(FIFO_DATA_PENDING_A::NO_PENDING)
    }
    #[doc = "FIFO Available Pending IRQ"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(FIFO_DATA_PENDING_A::PENDING)
    }
}
#[doc = "Field `fifo_overrun_pending` reader - ADC FIFO Overrun IRQ Pending"]
pub type FIFO_OVERRUN_PENDING_R = crate::BitReader<FIFO_OVERRUN_PENDING_A>;
#[doc = "ADC FIFO Overrun IRQ Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFO_OVERRUN_PENDING_A {
    #[doc = "0: No Pending IRQ"]
    NP_PENDING = 0,
    #[doc = "1: FIFO Overrun Pending IRQ"]
    PENDING = 1,
}
impl From<FIFO_OVERRUN_PENDING_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_OVERRUN_PENDING_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_OVERRUN_PENDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_OVERRUN_PENDING_A {
        match self.bits {
            false => FIFO_OVERRUN_PENDING_A::NP_PENDING,
            true => FIFO_OVERRUN_PENDING_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NP_PENDING`"]
    #[inline(always)]
    pub fn is_np_pending(&self) -> bool {
        *self == FIFO_OVERRUN_PENDING_A::NP_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == FIFO_OVERRUN_PENDING_A::PENDING
    }
}
#[doc = "Field `fifo_overrun_pending` writer - ADC FIFO Overrun IRQ Pending"]
pub type FIFO_OVERRUN_PENDING_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, GP_FIFO_INTS_SPEC, FIFO_OVERRUN_PENDING_A, O>;
impl<'a, const O: u8> FIFO_OVERRUN_PENDING_W<'a, O> {
    #[doc = "No Pending IRQ"]
    #[inline(always)]
    pub fn np_pending(self) -> &'a mut W {
        self.variant(FIFO_OVERRUN_PENDING_A::NP_PENDING)
    }
    #[doc = "FIFO Overrun Pending IRQ"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(FIFO_OVERRUN_PENDING_A::PENDING)
    }
}
impl R {
    #[doc = "Bits 8:13 - ADC FIFO available sample word counter"]
    #[inline(always)]
    pub fn rxa_cnt(&self) -> RXA_CNT_R {
        RXA_CNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - ADC FIFO Data Available Pending Bit"]
    #[inline(always)]
    pub fn fifo_data_pending(&self) -> FIFO_DATA_PENDING_R {
        FIFO_DATA_PENDING_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC FIFO Overrun IRQ Pending"]
    #[inline(always)]
    pub fn fifo_overrun_pending(&self) -> FIFO_OVERRUN_PENDING_R {
        FIFO_OVERRUN_PENDING_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - ADC FIFO Data Available Pending Bit"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_data_pending(&mut self) -> FIFO_DATA_PENDING_W<16> {
        FIFO_DATA_PENDING_W::new(self)
    }
    #[doc = "Bit 17 - ADC FIFO Overrun IRQ Pending"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_overrun_pending(&mut self) -> FIFO_OVERRUN_PENDING_W<17> {
        FIFO_OVERRUN_PENDING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPADC FIFO Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_fifo_ints](index.html) module"]
pub struct GP_FIFO_INTS_SPEC;
impl crate::RegisterSpec for GP_FIFO_INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gp_fifo_ints::R](R) reader structure"]
impl crate::Readable for GP_FIFO_INTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_fifo_ints::W](W) writer structure"]
impl crate::Writable for GP_FIFO_INTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0003_0000;
}
#[doc = "`reset()` method sets gp_fifo_ints to value 0"]
impl crate::Resettable for GP_FIFO_INTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
