#[doc = "Register `res240_ctrl` reader"]
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
#[doc = "Register `res240_ctrl` writer"]
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
#[doc = "Field `ddr_res240_trim` reader - 240ohms Resistor trimming bit"]
pub type DDR_RES240_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ddr_res240_trim` writer - 240ohms Resistor trimming bit"]
pub type DDR_RES240_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RES240_CTRL_SPEC, u8, u8, 6, O>;
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
    #[must_use]
    pub fn ddr_res240_trim(&mut self) -> DDR_RES240_TRIM_W<0> {
        DDR_RES240_TRIM_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets res240_ctrl to value 0"]
impl crate::Resettable for RES240_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
