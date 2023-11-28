#[doc = "Register `riscv_sta_add1` reader"]
pub type R = crate::R<RISCV_STA_ADD1_SPEC>;
#[doc = "Register `riscv_sta_add1` writer"]
pub type W = crate::W<RISCV_STA_ADD1_SPEC>;
#[doc = "Field `sta_add_h` reader - Start Address High 8-bit"]
pub type STA_ADD_H_R = crate::FieldReader;
#[doc = "Field `sta_add_h` writer - Start Address High 8-bit"]
pub type STA_ADD_H_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
    #[must_use]
    pub fn sta_add_h(&mut self) -> STA_ADD_H_W<RISCV_STA_ADD1_SPEC> {
        STA_ADD_H_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RISCV Start Address1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`riscv_sta_add1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`riscv_sta_add1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RISCV_STA_ADD1_SPEC;
impl crate::RegisterSpec for RISCV_STA_ADD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`riscv_sta_add1::R`](R) reader structure"]
impl crate::Readable for RISCV_STA_ADD1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`riscv_sta_add1::W`](W) writer structure"]
impl crate::Writable for RISCV_STA_ADD1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets riscv_sta_add1 to value 0"]
impl crate::Resettable for RISCV_STA_ADD1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
