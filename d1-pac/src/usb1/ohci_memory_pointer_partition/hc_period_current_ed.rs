#[doc = "Register `hc_period_current_ed` reader"]
pub struct R(crate::R<HC_PERIOD_CURRENT_ED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HC_PERIOD_CURRENT_ED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HC_PERIOD_CURRENT_ED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HC_PERIOD_CURRENT_ED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hc_period_current_ed` writer"]
pub struct W(crate::W<HC_PERIOD_CURRENT_ED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HC_PERIOD_CURRENT_ED_SPEC>;
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
impl From<crate::W<HC_PERIOD_CURRENT_ED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HC_PERIOD_CURRENT_ED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pced_3_0` reader - Because the general TD length is 16 bytes, the memory structure for the TD must be aligned to a 16-byte boundary. So the lower bits in the PCED, through bit 0 to bit 3 must be zero in this field."]
pub type PCED_3_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pced_31_4` reader - This is used by HC to point to the head of one of the Periodec list which will be processed in the current Frame. The content of this register is updated by HC after a periodic ED has been processed. HCD may read the content in determining which ED is currently being processed at the time of reading."]
pub type PCED_31_4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `pced_31_4` writer - This is used by HC to point to the head of one of the Periodec list which will be processed in the current Frame. The content of this register is updated by HC after a periodic ED has been processed. HCD may read the content in determining which ED is currently being processed at the time of reading."]
pub type PCED_31_4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HC_PERIOD_CURRENT_ED_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:3 - Because the general TD length is 16 bytes, the memory structure for the TD must be aligned to a 16-byte boundary. So the lower bits in the PCED, through bit 0 to bit 3 must be zero in this field."]
    #[inline(always)]
    pub fn pced_3_0(&self) -> PCED_3_0_R {
        PCED_3_0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - This is used by HC to point to the head of one of the Periodec list which will be processed in the current Frame. The content of this register is updated by HC after a periodic ED has been processed. HCD may read the content in determining which ED is currently being processed at the time of reading."]
    #[inline(always)]
    pub fn pced_31_4(&self) -> PCED_31_4_R {
        PCED_31_4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - This is used by HC to point to the head of one of the Periodec list which will be processed in the current Frame. The content of this register is updated by HC after a periodic ED has been processed. HCD may read the content in determining which ED is currently being processed at the time of reading."]
    #[inline(always)]
    #[must_use]
    pub fn pced_31_4(&mut self) -> PCED_31_4_W<4> {
        PCED_31_4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OHCI Period Current ED Base\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc_period_current_ed](index.html) module"]
pub struct HC_PERIOD_CURRENT_ED_SPEC;
impl crate::RegisterSpec for HC_PERIOD_CURRENT_ED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hc_period_current_ed::R](R) reader structure"]
impl crate::Readable for HC_PERIOD_CURRENT_ED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hc_period_current_ed::W](W) writer structure"]
impl crate::Writable for HC_PERIOD_CURRENT_ED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_period_current_ed to value 0"]
impl crate::Resettable for HC_PERIOD_CURRENT_ED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
