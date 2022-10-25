#[doc = "Register `sram_addr_twist` reader"]
pub struct R(crate::R<SRAM_ADDR_TWIST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_ADDR_TWIST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_ADDR_TWIST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_ADDR_TWIST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sram_addr_twist` writer"]
pub struct W(crate::W<SRAM_ADDR_TWIST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_ADDR_TWIST_SPEC>;
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
impl From<crate::W<SRAM_ADDR_TWIST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_ADDR_TWIST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sram_addr_ts_fg` reader - SRAM Address Twist Flag"]
pub type SRAM_ADDR_TS_FG_R = crate::BitReader<bool>;
#[doc = "Field `sram_addr_ts_fg` writer - SRAM Address Twist Flag"]
pub type SRAM_ADDR_TS_FG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SRAM_ADDR_TWIST_SPEC, bool, O>;
#[doc = "Field `sram_ts_kf` writer - SRAM Twist Keyfield"]
pub type SRAM_TS_KF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRAM_ADDR_TWIST_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - SRAM Address Twist Flag"]
    #[inline(always)]
    pub fn sram_addr_ts_fg(&self) -> SRAM_ADDR_TS_FG_R {
        SRAM_ADDR_TS_FG_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM Address Twist Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sram_addr_ts_fg(&mut self) -> SRAM_ADDR_TS_FG_W<0> {
        SRAM_ADDR_TS_FG_W::new(self)
    }
    #[doc = "Bits 16:31 - SRAM Twist Keyfield"]
    #[inline(always)]
    #[must_use]
    pub fn sram_ts_kf(&mut self) -> SRAM_TS_KF_W<16> {
        SRAM_TS_KF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Address Twist Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_addr_twist](index.html) module"]
pub struct SRAM_ADDR_TWIST_SPEC;
impl crate::RegisterSpec for SRAM_ADDR_TWIST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_addr_twist::R](R) reader structure"]
impl crate::Readable for SRAM_ADDR_TWIST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_addr_twist::W](W) writer structure"]
impl crate::Writable for SRAM_ADDR_TWIST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sram_addr_twist to value 0"]
impl crate::Resettable for SRAM_ADDR_TWIST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
