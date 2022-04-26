#[doc = "Register `SMHC_EXT_CMD` reader"]
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
#[doc = "Register `SMHC_EXT_CMD` writer"]
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
#[doc = "Field `AUTO_CMD23_EN` reader - Send CMD23 Automatically"]
pub struct AUTO_CMD23_EN_R(crate::FieldReader<bool>);
impl AUTO_CMD23_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTO_CMD23_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTO_CMD23_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTO_CMD23_EN` writer - Send CMD23 Automatically"]
pub struct AUTO_CMD23_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_CMD23_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
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
    pub fn auto_cmd23_en(&mut self) -> AUTO_CMD23_EN_W {
        AUTO_CMD23_EN_W { w: self }
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
}
#[doc = "`reset()` method sets SMHC_EXT_CMD to value 0"]
impl crate::Resettable for SMHC_EXT_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
