#[doc = "Register `IC_CHARA_REG` reader"]
pub struct R(crate::R<IC_CHARA_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_CHARA_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_CHARA_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_CHARA_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IC_CHARA_REG` writer"]
pub struct W(crate::W<IC_CHARA_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC_CHARA_REG_SPEC>;
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
impl From<crate::W<IC_CHARA_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IC_CHARA_REG_SPEC>) -> Self {
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
#[doc = "IC Characteristic Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_chara_reg](index.html) module"]
pub struct IC_CHARA_REG_SPEC;
impl crate::RegisterSpec for IC_CHARA_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_chara_reg::R](R) reader structure"]
impl crate::Readable for IC_CHARA_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ic_chara_reg::W](W) writer structure"]
impl crate::Writable for IC_CHARA_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IC_CHARA_REG to value 0"]
impl crate::Resettable for IC_CHARA_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
