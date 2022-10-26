#[doc = "Register `hc_control_head_ed` reader"]
pub struct R(crate::R<HC_CONTROL_HEAD_ED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HC_CONTROL_HEAD_ED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HC_CONTROL_HEAD_ED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HC_CONTROL_HEAD_ED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hc_control_head_ed` writer"]
pub struct W(crate::W<HC_CONTROL_HEAD_ED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HC_CONTROL_HEAD_ED_SPEC>;
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
impl From<crate::W<HC_CONTROL_HEAD_ED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HC_CONTROL_HEAD_ED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ehcd_3_0` reader - Because the general TD length is 16 bytes, the memory structure for the TD must be aligned to a 16-byte boundary. So the lower bits in the PCED, through bit 0 to bit 3 must be zero in this field."]
pub type EHCD_3_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ehcd_31_4` reader - The HcControlHeadED register contains the physical address of the first Endpoint Descriptor of the Control list. HC traverse the Control list starting with the HcControlHeadED pointer. The content is loaded from HCCA during the initialization of HC."]
pub type EHCD_31_4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ehcd_31_4` writer - The HcControlHeadED register contains the physical address of the first Endpoint Descriptor of the Control list. HC traverse the Control list starting with the HcControlHeadED pointer. The content is loaded from HCCA during the initialization of HC."]
pub type EHCD_31_4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HC_CONTROL_HEAD_ED_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:3 - Because the general TD length is 16 bytes, the memory structure for the TD must be aligned to a 16-byte boundary. So the lower bits in the PCED, through bit 0 to bit 3 must be zero in this field."]
    #[inline(always)]
    pub fn ehcd_3_0(&self) -> EHCD_3_0_R {
        EHCD_3_0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - The HcControlHeadED register contains the physical address of the first Endpoint Descriptor of the Control list. HC traverse the Control list starting with the HcControlHeadED pointer. The content is loaded from HCCA during the initialization of HC."]
    #[inline(always)]
    pub fn ehcd_31_4(&self) -> EHCD_31_4_R {
        EHCD_31_4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - The HcControlHeadED register contains the physical address of the first Endpoint Descriptor of the Control list. HC traverse the Control list starting with the HcControlHeadED pointer. The content is loaded from HCCA during the initialization of HC."]
    #[inline(always)]
    #[must_use]
    pub fn ehcd_31_4(&mut self) -> EHCD_31_4_W<4> {
        EHCD_31_4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OHCI Control Head ED Base\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc_control_head_ed](index.html) module"]
pub struct HC_CONTROL_HEAD_ED_SPEC;
impl crate::RegisterSpec for HC_CONTROL_HEAD_ED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hc_control_head_ed::R](R) reader structure"]
impl crate::Readable for HC_CONTROL_HEAD_ED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hc_control_head_ed::W](W) writer structure"]
impl crate::Writable for HC_CONTROL_HEAD_ED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_control_head_ed to value 0"]
impl crate::Resettable for HC_CONTROL_HEAD_ED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
