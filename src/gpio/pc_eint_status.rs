#[doc = "Register `pc_eint_status` reader"]
pub struct R(crate::R<PC_EINT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PC_EINT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PC_EINT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PC_EINT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pc_eint_status` writer"]
pub struct W(crate::W<PC_EINT_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PC_EINT_STATUS_SPEC>;
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
impl From<crate::W<PC_EINT_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PC_EINT_STATUS_SPEC>) -> Self {
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
#[doc = "PC External Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc_eint_status](index.html) module"]
pub struct PC_EINT_STATUS_SPEC;
impl crate::RegisterSpec for PC_EINT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pc_eint_status::R](R) reader structure"]
impl crate::Readable for PC_EINT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pc_eint_status::W](W) writer structure"]
impl crate::Writable for PC_EINT_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pc_eint_status to value 0"]
impl crate::Resettable for PC_EINT_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
