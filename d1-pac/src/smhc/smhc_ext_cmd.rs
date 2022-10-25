#[doc = "Register `smhc_ext_cmd` reader"]
pub struct R(crate::R<SMHC_EXT_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_EXT_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_EXT_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_EXT_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `smhc_ext_cmd` writer"]
pub struct W(crate::W<SMHC_EXT_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_EXT_CMD_SPEC>;
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
impl From<crate::W<SMHC_EXT_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_EXT_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `auto_cmd23_en` reader - Send CMD23 Automatically"]
pub type AUTO_CMD23_EN_R = crate::BitReader<bool>;
#[doc = "Field `auto_cmd23_en` writer - Send CMD23 Automatically"]
pub type AUTO_CMD23_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_EXT_CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Send CMD23 Automatically"]
    #[inline(always)]
    pub fn auto_cmd23_en(&self) -> AUTO_CMD23_EN_R {
        AUTO_CMD23_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Send CMD23 Automatically"]
    #[inline(always)]
    #[must_use]
    pub fn auto_cmd23_en(&mut self) -> AUTO_CMD23_EN_W<0> {
        AUTO_CMD23_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extended Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_ext_cmd](index.html) module"]
pub struct SMHC_EXT_CMD_SPEC;
impl crate::RegisterSpec for SMHC_EXT_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_ext_cmd::R](R) reader structure"]
impl crate::Readable for SMHC_EXT_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_ext_cmd::W](W) writer structure"]
impl crate::Writable for SMHC_EXT_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_ext_cmd to value 0"]
impl crate::Resettable for SMHC_EXT_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
