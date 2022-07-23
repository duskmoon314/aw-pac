#[doc = "Register `riscv_sta_add1_reg` reader"]
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
#[doc = "Register `riscv_sta_add1_reg` writer"]
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
#[doc = "Field `sta_add_h` reader - Start Address High 8-bit"]
pub type STA_ADD_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sta_add_h` writer - Start Address High 8-bit"]
pub type STA_ADD_H_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RISCV_STA_ADD1_REG_SPEC, u8, u8, 8, O>;
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
    pub fn sta_add_h(&mut self) -> STA_ADD_H_W<0> {
        STA_ADD_H_W::new(self)
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
#[doc = "`reset()` method sets riscv_sta_add1_reg to value 0"]
impl crate::Resettable for RISCV_STA_ADD1_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
