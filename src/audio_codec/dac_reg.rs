#[doc = "Register `DAC_REG` reader"]
pub struct R(crate::R<DAC_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_REG` writer"]
pub struct W(crate::W<DAC_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_REG_SPEC>;
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
impl From<crate::W<DAC_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_REG_SPEC>) -> Self {
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
#[doc = "DAC Analog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_reg](index.html) module"]
pub struct DAC_REG_SPEC;
impl crate::RegisterSpec for DAC_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_reg::R](R) reader structure"]
impl crate::Readable for DAC_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_reg::W](W) writer structure"]
impl crate::Writable for DAC_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC_REG to value 0"]
impl crate::Resettable for DAC_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
