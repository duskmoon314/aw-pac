#[doc = "Register `hc_hcca` reader"]
pub struct R(crate::R<HC_HCCA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HC_HCCA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HC_HCCA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HC_HCCA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hc_hcca` writer"]
pub struct W(crate::W<HC_HCCA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HC_HCCA_SPEC>;
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
impl From<crate::W<HC_HCCA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HC_HCCA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `hcca_7_0` reader - The alignment restriction in HcHCCA register is evaluated by examining the number of zeros in the lower order bits. The minimum alignment is 256 bytes, therefore, bits 0 through 7 must always return 0 when read."]
pub type HCCA_7_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `hcca_31_8` reader - This is the base address of the Host Controller Communication Area. This area is used to hold the control structures and the Interrupt table that are accessed by both the Host Controller and the Host Controller Driver."]
pub type HCCA_31_8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `hcca_31_8` writer - This is the base address of the Host Controller Communication Area. This area is used to hold the control structures and the Interrupt table that are accessed by both the Host Controller and the Host Controller Driver."]
pub type HCCA_31_8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HC_HCCA_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - The alignment restriction in HcHCCA register is evaluated by examining the number of zeros in the lower order bits. The minimum alignment is 256 bytes, therefore, bits 0 through 7 must always return 0 when read."]
    #[inline(always)]
    pub fn hcca_7_0(&self) -> HCCA_7_0_R {
        HCCA_7_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - This is the base address of the Host Controller Communication Area. This area is used to hold the control structures and the Interrupt table that are accessed by both the Host Controller and the Host Controller Driver."]
    #[inline(always)]
    pub fn hcca_31_8(&self) -> HCCA_31_8_R {
        HCCA_31_8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 8:31 - This is the base address of the Host Controller Communication Area. This area is used to hold the control structures and the Interrupt table that are accessed by both the Host Controller and the Host Controller Driver."]
    #[inline(always)]
    #[must_use]
    pub fn hcca_31_8(&mut self) -> HCCA_31_8_W<8> {
        HCCA_31_8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OHCI HCCA Base\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc_hcca](index.html) module"]
pub struct HC_HCCA_SPEC;
impl crate::RegisterSpec for HC_HCCA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hc_hcca::R](R) reader structure"]
impl crate::Readable for HC_HCCA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hc_hcca::W](W) writer structure"]
impl crate::Writable for HC_HCCA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_hcca to value 0"]
impl crate::Resettable for HC_HCCA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
