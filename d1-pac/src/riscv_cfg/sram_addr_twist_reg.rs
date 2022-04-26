#[doc = "Register `SRAM_ADDR_TWIST_REG` reader"]
pub struct R(crate::R<SRAM_ADDR_TWIST_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_ADDR_TWIST_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_ADDR_TWIST_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_ADDR_TWIST_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAM_ADDR_TWIST_REG` writer"]
pub struct W(crate::W<SRAM_ADDR_TWIST_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_ADDR_TWIST_REG_SPEC>;
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
impl From<crate::W<SRAM_ADDR_TWIST_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_ADDR_TWIST_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM_TS_KF` writer - SRAM Twist Keyfield"]
pub struct SRAM_TS_KF_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_TS_KF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `SRAM_ADDR_TS_FG` reader - SRAM Address Twist Flag"]
pub struct SRAM_ADDR_TS_FG_R(crate::FieldReader<bool>);
impl SRAM_ADDR_TS_FG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_ADDR_TS_FG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_ADDR_TS_FG_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_ADDR_TS_FG` writer - SRAM Address Twist Flag"]
pub struct SRAM_ADDR_TS_FG_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_ADDR_TS_FG_W<'a> {
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
    #[doc = "Bit 0 - SRAM Address Twist Flag"]
    #[inline(always)]
    pub fn sram_addr_ts_fg(&self) -> SRAM_ADDR_TS_FG_R {
        SRAM_ADDR_TS_FG_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - SRAM Twist Keyfield"]
    #[inline(always)]
    pub fn sram_ts_kf(&mut self) -> SRAM_TS_KF_W {
        SRAM_TS_KF_W { w: self }
    }
    #[doc = "Bit 0 - SRAM Address Twist Flag"]
    #[inline(always)]
    pub fn sram_addr_ts_fg(&mut self) -> SRAM_ADDR_TS_FG_W {
        SRAM_ADDR_TS_FG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Address Twist Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_addr_twist_reg](index.html) module"]
pub struct SRAM_ADDR_TWIST_REG_SPEC;
impl crate::RegisterSpec for SRAM_ADDR_TWIST_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_addr_twist_reg::R](R) reader structure"]
impl crate::Readable for SRAM_ADDR_TWIST_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_addr_twist_reg::W](W) writer structure"]
impl crate::Writable for SRAM_ADDR_TWIST_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRAM_ADDR_TWIST_REG to value 0"]
impl crate::Resettable for SRAM_ADDR_TWIST_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
