#[doc = "Register `hc_done_head` reader"]
pub struct R(crate::R<HC_DONE_HEAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HC_DONE_HEAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HC_DONE_HEAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HC_DONE_HEAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hc_done_head` writer"]
pub struct W(crate::W<HC_DONE_HEAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HC_DONE_HEAD_SPEC>;
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
impl From<crate::W<HC_DONE_HEAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HC_DONE_HEAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dh_3_0` reader - Because the general TD length is 16 bytes, the memory structure for the TD must be aligned to a 16-byte boundary. So the lower bits in the PCED, through bit 0 to bit 3 must be zero in this field."]
pub type DH_3_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dh_31_4` reader - When a TD is completed, HC writes the content of HcDoneHead to the NextTD field of the TD. HC then overwrites the content of HcDoneHead with the address of this TD. This is set to zero whenever HC writes the content of this register to HCCA. It also sets the WritebackDoneHead of HcInterruptStatus."]
pub type DH_31_4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `dh_31_4` writer - When a TD is completed, HC writes the content of HcDoneHead to the NextTD field of the TD. HC then overwrites the content of HcDoneHead with the address of this TD. This is set to zero whenever HC writes the content of this register to HCCA. It also sets the WritebackDoneHead of HcInterruptStatus."]
pub type DH_31_4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HC_DONE_HEAD_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:3 - Because the general TD length is 16 bytes, the memory structure for the TD must be aligned to a 16-byte boundary. So the lower bits in the PCED, through bit 0 to bit 3 must be zero in this field."]
    #[inline(always)]
    pub fn dh_3_0(&self) -> DH_3_0_R {
        DH_3_0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - When a TD is completed, HC writes the content of HcDoneHead to the NextTD field of the TD. HC then overwrites the content of HcDoneHead with the address of this TD. This is set to zero whenever HC writes the content of this register to HCCA. It also sets the WritebackDoneHead of HcInterruptStatus."]
    #[inline(always)]
    pub fn dh_31_4(&self) -> DH_31_4_R {
        DH_31_4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - When a TD is completed, HC writes the content of HcDoneHead to the NextTD field of the TD. HC then overwrites the content of HcDoneHead with the address of this TD. This is set to zero whenever HC writes the content of this register to HCCA. It also sets the WritebackDoneHead of HcInterruptStatus."]
    #[inline(always)]
    #[must_use]
    pub fn dh_31_4(&mut self) -> DH_31_4_W<4> {
        DH_31_4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OHCI Done Head Base\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc_done_head](index.html) module"]
pub struct HC_DONE_HEAD_SPEC;
impl crate::RegisterSpec for HC_DONE_HEAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hc_done_head::R](R) reader structure"]
impl crate::Readable for HC_DONE_HEAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hc_done_head::W](W) writer structure"]
impl crate::Writable for HC_DONE_HEAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_done_head to value 0"]
impl crate::Resettable for HC_DONE_HEAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
