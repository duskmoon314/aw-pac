#[doc = "Register `GP_FIFO_INTS` reader"]
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
#[doc = "Register `GP_FIFO_INTS` writer"]
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
#[doc = "ADC FIFO Overrun IRQ Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `FIFO_OVERRUN_PENDING` reader - ADC FIFO Overrun IRQ Pending"]
pub struct FIFO_OVERRUN_PENDING_R(crate::FieldReader<bool>);
impl FIFO_OVERRUN_PENDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_OVERRUN_PENDING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == FIFO_OVERRUN_PENDING_A::NP_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == FIFO_OVERRUN_PENDING_A::PENDING
    }
}
impl core::ops::Deref for FIFO_OVERRUN_PENDING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_OVERRUN_PENDING` writer - ADC FIFO Overrun IRQ Pending"]
pub struct FIFO_OVERRUN_PENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_OVERRUN_PENDING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_OVERRUN_PENDING_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "ADC FIFO Data Available Pending Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `FIFO_DATA_PENDING` reader - ADC FIFO Data Available Pending Bit"]
pub struct FIFO_DATA_PENDING_R(crate::FieldReader<bool>);
impl FIFO_DATA_PENDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_DATA_PENDING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == FIFO_DATA_PENDING_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == FIFO_DATA_PENDING_A::PENDING
    }
}
impl core::ops::Deref for FIFO_DATA_PENDING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_DATA_PENDING` writer - ADC FIFO Data Available Pending Bit"]
pub struct FIFO_DATA_PENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_DATA_PENDING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_DATA_PENDING_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `RXA_CNT` reader - ADC FIFO available sample word counter"]
pub struct RXA_CNT_R(crate::FieldReader<u8>);
impl RXA_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXA_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXA_CNT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 17 - ADC FIFO Overrun IRQ Pending"]
    #[inline(always)]
    pub fn fifo_overrun_pending(&self) -> FIFO_OVERRUN_PENDING_R {
        FIFO_OVERRUN_PENDING_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC FIFO Data Available Pending Bit"]
    #[inline(always)]
    pub fn fifo_data_pending(&self) -> FIFO_DATA_PENDING_R {
        FIFO_DATA_PENDING_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 8:13 - ADC FIFO available sample word counter"]
    #[inline(always)]
    pub fn rxa_cnt(&self) -> RXA_CNT_R {
        RXA_CNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 17 - ADC FIFO Overrun IRQ Pending"]
    #[inline(always)]
    pub fn fifo_overrun_pending(&mut self) -> FIFO_OVERRUN_PENDING_W {
        FIFO_OVERRUN_PENDING_W { w: self }
    }
    #[doc = "Bit 16 - ADC FIFO Data Available Pending Bit"]
    #[inline(always)]
    pub fn fifo_data_pending(&mut self) -> FIFO_DATA_PENDING_W {
        FIFO_DATA_PENDING_W { w: self }
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
}
#[doc = "`reset()` method sets GP_FIFO_INTS to value 0"]
impl crate::Resettable for GP_FIFO_INTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
