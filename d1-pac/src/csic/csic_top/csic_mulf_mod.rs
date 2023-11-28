#[doc = "Register `csic_mulf_mod` reader"]
pub type R = crate::R<CSIC_MULF_MOD_SPEC>;
#[doc = "Register `csic_mulf_mod` writer"]
pub type W = crate::W<CSIC_MULF_MOD_SPEC>;
#[doc = "Field `mulf_en` reader - "]
pub type MULF_EN_R = crate::BitReader;
#[doc = "Field `mulf_en` writer - "]
pub type MULF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mulf_cs` reader - "]
pub type MULF_CS_R = crate::FieldReader;
#[doc = "Field `mulf_cs` writer - "]
pub type MULF_CS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `mulf_status` reader - "]
pub type MULF_STATUS_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mulf_en(&self) -> MULF_EN_R {
        MULF_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn mulf_cs(&self) -> MULF_CS_R {
        MULF_CS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn mulf_status(&self) -> MULF_STATUS_R {
        MULF_STATUS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn mulf_en(&mut self) -> MULF_EN_W<CSIC_MULF_MOD_SPEC> {
        MULF_EN_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn mulf_cs(&mut self) -> MULF_CS_W<CSIC_MULF_MOD_SPEC> {
        MULF_CS_W::new(self, 8)
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
#[doc = "CSIC Multi-Frame Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_mulf_mod::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_mulf_mod::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_MULF_MOD_SPEC;
impl crate::RegisterSpec for CSIC_MULF_MOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_mulf_mod::R`](R) reader structure"]
impl crate::Readable for CSIC_MULF_MOD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_mulf_mod::W`](W) writer structure"]
impl crate::Writable for CSIC_MULF_MOD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_mulf_mod to value 0"]
impl crate::Resettable for CSIC_MULF_MOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
