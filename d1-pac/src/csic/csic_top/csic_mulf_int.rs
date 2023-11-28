#[doc = "Register `csic_mulf_int` reader"]
pub type R = crate::R<CSIC_MULF_INT_SPEC>;
#[doc = "Register `csic_mulf_int` writer"]
pub type W = crate::W<CSIC_MULF_INT_SPEC>;
#[doc = "Field `mulf_done_en` reader - "]
pub type MULF_DONE_EN_R = crate::BitReader;
#[doc = "Field `mulf_done_en` writer - "]
pub type MULF_DONE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mulf_err_en` reader - "]
pub type MULF_ERR_EN_R = crate::BitReader;
#[doc = "Field `mulf_err_en` writer - "]
pub type MULF_ERR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mulf_done_pd` reader - "]
pub type MULF_DONE_PD_R = crate::BitReader;
#[doc = "Field `mulf_done_pd` writer - "]
pub type MULF_DONE_PD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `mulf_err_pd` reader - "]
pub type MULF_ERR_PD_R = crate::BitReader;
#[doc = "Field `mulf_err_pd` writer - "]
pub type MULF_ERR_PD_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mulf_done_en(&self) -> MULF_DONE_EN_R {
        MULF_DONE_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mulf_err_en(&self) -> MULF_ERR_EN_R {
        MULF_ERR_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn mulf_done_pd(&self) -> MULF_DONE_PD_R {
        MULF_DONE_PD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn mulf_err_pd(&self) -> MULF_ERR_PD_R {
        MULF_ERR_PD_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn mulf_done_en(&mut self) -> MULF_DONE_EN_W<CSIC_MULF_INT_SPEC> {
        MULF_DONE_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn mulf_err_en(&mut self) -> MULF_ERR_EN_W<CSIC_MULF_INT_SPEC> {
        MULF_ERR_EN_W::new(self, 1)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn mulf_done_pd(&mut self) -> MULF_DONE_PD_W<CSIC_MULF_INT_SPEC> {
        MULF_DONE_PD_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn mulf_err_pd(&mut self) -> MULF_ERR_PD_W<CSIC_MULF_INT_SPEC> {
        MULF_ERR_PD_W::new(self, 17)
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
#[doc = "CSIC Multi-Frame Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_mulf_int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_mulf_int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_MULF_INT_SPEC;
impl crate::RegisterSpec for CSIC_MULF_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_mulf_int::R`](R) reader structure"]
impl crate::Readable for CSIC_MULF_INT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_mulf_int::W`](W) writer structure"]
impl crate::Writable for CSIC_MULF_INT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0003_0000;
}
#[doc = "`reset()` method sets csic_mulf_int to value 0"]
impl crate::Resettable for CSIC_MULF_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
