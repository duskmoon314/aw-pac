#[doc = "Register `RES240_CTRL` reader"]
pub struct R(crate::R<RES240_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RES240_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RES240_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RES240_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RES240_CTRL` writer"]
pub struct W(crate::W<RES240_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RES240_CTRL_SPEC>;
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
impl From<crate::W<RES240_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RES240_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DDR_RES240_TRIM` reader - 240ohms Resistor trimming bit"]
pub struct DDR_RES240_TRIM_R(crate::FieldReader<u8>);
impl DDR_RES240_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DDR_RES240_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDR_RES240_TRIM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDR_RES240_TRIM` writer - 240ohms Resistor trimming bit"]
pub struct DDR_RES240_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DDR_RES240_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - 240ohms Resistor trimming bit"]
    #[inline(always)]
    pub fn ddr_res240_trim(&self) -> DDR_RES240_TRIM_R {
        DDR_RES240_TRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 240ohms Resistor trimming bit"]
    #[inline(always)]
    pub fn ddr_res240_trim(&mut self) -> DDR_RES240_TRIM_W {
        DDR_RES240_TRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "240ohms Resistor Manual Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res240_ctrl](index.html) module"]
pub struct RES240_CTRL_SPEC;
impl crate::RegisterSpec for RES240_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [res240_ctrl::R](R) reader structure"]
impl crate::Readable for RES240_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [res240_ctrl::W](W) writer structure"]
impl crate::Writable for RES240_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RES240_CTRL to value 0"]
impl crate::Resettable for RES240_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
