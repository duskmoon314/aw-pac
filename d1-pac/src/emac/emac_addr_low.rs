#[doc = "Register `emac_addr_low%s` reader"]
pub struct R(crate::R<EMAC_ADDR_LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMAC_ADDR_LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMAC_ADDR_LOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMAC_ADDR_LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `emac_addr_low%s` writer"]
pub struct W(crate::W<EMAC_ADDR_LOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMAC_ADDR_LOW_SPEC>;
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
impl From<crate::W<EMAC_ADDR_LOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMAC_ADDR_LOW_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMAC MAC Address Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emac_addr_low](index.html) module"]
pub struct EMAC_ADDR_LOW_SPEC;
impl crate::RegisterSpec for EMAC_ADDR_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emac_addr_low::R](R) reader structure"]
impl crate::Readable for EMAC_ADDR_LOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emac_addr_low::W](W) writer structure"]
impl crate::Writable for EMAC_ADDR_LOW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets emac_addr_low%s to value 0"]
impl crate::Resettable for EMAC_ADDR_LOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
