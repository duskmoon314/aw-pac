#[doc = "Register `CE_ESR` reader"]
pub struct R(crate::R<CE_ESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CE_ESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CE_ESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CE_ESR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CE_ESR` writer"]
pub struct W(crate::W<CE_ESR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CE_ESR_SPEC>;
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
impl From<crate::W<CE_ESR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CE_ESR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Task Channel Error Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TASK_CHANNEL_ERROR_TYPE_A {
    #[doc = "1: Algorithm not support"]
    ALGORITHM_NOT_SUPPORT = 1,
    #[doc = "2: Data length error"]
    DATA_LENGTH_ERROR = 2,
    #[doc = "4: keysram access error for AES"]
    KEYSRAM_ACCESS_ERROR = 4,
}
impl From<TASK_CHANNEL_ERROR_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: TASK_CHANNEL_ERROR_TYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Fields `TASK_CHANNEL(0-3)_ERROR_TYPE` reader - Task Channel Error Type"]
pub struct TASK_CHANNEL_ERROR_TYPE_R(crate::FieldReader<u8>);
impl TASK_CHANNEL_ERROR_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TASK_CHANNEL_ERROR_TYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TASK_CHANNEL_ERROR_TYPE_A> {
        match self.bits {
            1 => Some(TASK_CHANNEL_ERROR_TYPE_A::ALGORITHM_NOT_SUPPORT),
            2 => Some(TASK_CHANNEL_ERROR_TYPE_A::DATA_LENGTH_ERROR),
            4 => Some(TASK_CHANNEL_ERROR_TYPE_A::KEYSRAM_ACCESS_ERROR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ALGORITHM_NOT_SUPPORT`"]
    #[inline(always)]
    pub fn is_algorithm_not_support(&self) -> bool {
        **self == TASK_CHANNEL_ERROR_TYPE_A::ALGORITHM_NOT_SUPPORT
    }
    #[doc = "Checks if the value of the field is `DATA_LENGTH_ERROR`"]
    #[inline(always)]
    pub fn is_data_length_error(&self) -> bool {
        **self == TASK_CHANNEL_ERROR_TYPE_A::DATA_LENGTH_ERROR
    }
    #[doc = "Checks if the value of the field is `KEYSRAM_ACCESS_ERROR`"]
    #[inline(always)]
    pub fn is_keysram_access_error(&self) -> bool {
        **self == TASK_CHANNEL_ERROR_TYPE_A::KEYSRAM_ACCESS_ERROR
    }
}
impl core::ops::Deref for TASK_CHANNEL_ERROR_TYPE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `TASK_CHANNEL(0-3)_ERROR_TYPE` const generic writer - Task Channel Error Type"]
pub struct TASK_CHANNEL_ERROR_TYPE_W<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> TASK_CHANNEL_ERROR_TYPE_W<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TASK_CHANNEL_ERROR_TYPE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Algorithm not support"]
    #[inline(always)]
    pub fn algorithm_not_support(self) -> &'a mut W {
        self.variant(TASK_CHANNEL_ERROR_TYPE_A::ALGORITHM_NOT_SUPPORT)
    }
    #[doc = "Data length error"]
    #[inline(always)]
    pub fn data_length_error(self) -> &'a mut W {
        self.variant(TASK_CHANNEL_ERROR_TYPE_A::DATA_LENGTH_ERROR)
    }
    #[doc = "keysram access error for AES"]
    #[inline(always)]
    pub fn keysram_access_error(self) -> &'a mut W {
        self.variant(TASK_CHANNEL_ERROR_TYPE_A::KEYSRAM_ACCESS_ERROR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << O)) | ((value as u32 & 0x0f) << O);
        self.w
    }
}
impl R {
    #[doc = "Task Channel Error Type"]
    #[inline(always)]
    pub unsafe fn task_channel_error_type(&self, n: usize) -> TASK_CHANNEL_ERROR_TYPE_R {
        TASK_CHANNEL_ERROR_TYPE_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Task Channel Error Type"]
    #[inline(always)]
    pub fn task_channel0_error_type(&self) -> TASK_CHANNEL_ERROR_TYPE_R {
        TASK_CHANNEL_ERROR_TYPE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Task Channel Error Type"]
    #[inline(always)]
    pub fn task_channel1_error_type(&self) -> TASK_CHANNEL_ERROR_TYPE_R {
        TASK_CHANNEL_ERROR_TYPE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Task Channel Error Type"]
    #[inline(always)]
    pub fn task_channel2_error_type(&self) -> TASK_CHANNEL_ERROR_TYPE_R {
        TASK_CHANNEL_ERROR_TYPE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Task Channel Error Type"]
    #[inline(always)]
    pub fn task_channel3_error_type(&self) -> TASK_CHANNEL_ERROR_TYPE_R {
        TASK_CHANNEL_ERROR_TYPE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Task Channel Error Type"]
    #[inline(always)]
    pub unsafe fn task_channel_error_type<const O: usize>(
        &mut self,
    ) -> TASK_CHANNEL_ERROR_TYPE_W<O> {
        TASK_CHANNEL_ERROR_TYPE_W { w: self }
    }
    #[doc = "Bits 0:3 - Task Channel Error Type"]
    #[inline(always)]
    pub fn task_channel0_error_type(&mut self) -> TASK_CHANNEL_ERROR_TYPE_W<0> {
        TASK_CHANNEL_ERROR_TYPE_W { w: self }
    }
    #[doc = "Bits 4:7 - Task Channel Error Type"]
    #[inline(always)]
    pub fn task_channel1_error_type(&mut self) -> TASK_CHANNEL_ERROR_TYPE_W<4> {
        TASK_CHANNEL_ERROR_TYPE_W { w: self }
    }
    #[doc = "Bits 8:11 - Task Channel Error Type"]
    #[inline(always)]
    pub fn task_channel2_error_type(&mut self) -> TASK_CHANNEL_ERROR_TYPE_W<8> {
        TASK_CHANNEL_ERROR_TYPE_W { w: self }
    }
    #[doc = "Bits 12:15 - Task Channel Error Type"]
    #[inline(always)]
    pub fn task_channel3_error_type(&mut self) -> TASK_CHANNEL_ERROR_TYPE_W<12> {
        TASK_CHANNEL_ERROR_TYPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ce_esr](index.html) module"]
pub struct CE_ESR_SPEC;
impl crate::RegisterSpec for CE_ESR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ce_esr::R](R) reader structure"]
impl crate::Readable for CE_ESR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ce_esr::W](W) writer structure"]
impl crate::Writable for CE_ESR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CE_ESR to value 0"]
impl crate::Resettable for CE_ESR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
