#[doc = "Register `pe_eint_cfg0` reader"]
pub struct R(crate::R<PE_EINT_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PE_EINT_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PE_EINT_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PE_EINT_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pe_eint_cfg0` writer"]
pub struct W(crate::W<PE_EINT_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PE_EINT_CFG0_SPEC>;
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
impl From<crate::W<PE_EINT_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PE_EINT_CFG0_SPEC>) -> Self {
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
#[doc = "PE External Interrupt Configure Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe_eint_cfg0](index.html) module"]
pub struct PE_EINT_CFG0_SPEC;
impl crate::RegisterSpec for PE_EINT_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pe_eint_cfg0::R](R) reader structure"]
impl crate::Readable for PE_EINT_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pe_eint_cfg0::W](W) writer structure"]
impl crate::Writable for PE_EINT_CFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pe_eint_cfg0 to value 0"]
impl crate::Resettable for PE_EINT_CFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
