#[doc = "Register `RISCV_STA_ADD1_REG` reader"]
pub struct R(crate::R<RISCV_STA_ADD1_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RISCV_STA_ADD1_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RISCV_STA_ADD1_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RISCV_STA_ADD1_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RISCV_STA_ADD1_REG` writer"]
pub struct W(crate::W<RISCV_STA_ADD1_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RISCV_STA_ADD1_REG_SPEC>;
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
impl From<crate::W<RISCV_STA_ADD1_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RISCV_STA_ADD1_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STA_ADD_H` reader - Start Address High 8-bit"]
pub struct STA_ADD_H_R(crate::FieldReader<u8>);
impl STA_ADD_H_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STA_ADD_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STA_ADD_H_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STA_ADD_H` writer - Start Address High 8-bit"]
pub struct STA_ADD_H_W<'a> {
    w: &'a mut W,
}
impl<'a> STA_ADD_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Start Address High 8-bit"]
    #[inline(always)]
    pub fn sta_add_h(&self) -> STA_ADD_H_R {
        STA_ADD_H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Start Address High 8-bit"]
    #[inline(always)]
    pub fn sta_add_h(&mut self) -> STA_ADD_H_W {
        STA_ADD_H_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RISCV Start Address1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [riscv_sta_add1_reg](index.html) module"]
pub struct RISCV_STA_ADD1_REG_SPEC;
impl crate::RegisterSpec for RISCV_STA_ADD1_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [riscv_sta_add1_reg::R](R) reader structure"]
impl crate::Readable for RISCV_STA_ADD1_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [riscv_sta_add1_reg::W](W) writer structure"]
impl crate::Writable for RISCV_STA_ADD1_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RISCV_STA_ADD1_REG to value 0"]
impl crate::Resettable for RISCV_STA_ADD1_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
