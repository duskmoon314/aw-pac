#[doc = "Register `RF1P_CFG_REG` reader"]
pub struct R(crate::R<RF1P_CFG_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF1P_CFG_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF1P_CFG_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF1P_CFG_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF1P_CFG_REG` writer"]
pub struct W(crate::W<RF1P_CFG_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF1P_CFG_REG_SPEC>;
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
impl From<crate::W<RF1P_CFG_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF1P_CFG_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RF1P_CFG` reader - RF1P Configuration"]
pub struct RF1P_CFG_R(crate::FieldReader<u8, u8>);
impl RF1P_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RF1P_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF1P_CFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF1P_CFG` writer - RF1P Configuration"]
pub struct RF1P_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1P_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - RF1P Configuration"]
    #[inline(always)]
    pub fn rf1p_cfg(&self) -> RF1P_CFG_R {
        RF1P_CFG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RF1P Configuration"]
    #[inline(always)]
    pub fn rf1p_cfg(&mut self) -> RF1P_CFG_W {
        RF1P_CFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RF1P Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf1p_cfg_reg](index.html) module"]
pub struct RF1P_CFG_REG_SPEC;
impl crate::RegisterSpec for RF1P_CFG_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf1p_cfg_reg::R](R) reader structure"]
impl crate::Readable for RF1P_CFG_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf1p_cfg_reg::W](W) writer structure"]
impl crate::Writable for RF1P_CFG_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF1P_CFG_REG to value 0"]
impl crate::Resettable for RF1P_CFG_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
